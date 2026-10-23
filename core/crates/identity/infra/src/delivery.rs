//! Code delivery (auth policy rev. 2 §2.3) and e-mail notices.
//!
//! - [`DeliveryWorker`]: jobs are queued by the use cases after the commit and sent by a detached
//!   task, so the HTTP response never waits for (or reveals) a send. The outcome goes to
//!   `RecordDelivery`.
//! - [`DisabledSender`]: the default while no provider exists (Q8 e-mail, U17 WhatsApp); every
//!   send fails, so delivery reads `failed` and sign-in is not possible. Correct by design.
//! - [`SmtpDevSender`]: plain SMTP to a local Mailpit, accepted ONLY with `APP_ENV` development or
//!   test and `CODE_SENDER=dev_smtp` (Esquadro decision 3, 2026-10-23); `app` refuses to start it
//!   anywhere else. It never logs a code or an address.

use std::sync::Arc;

use async_trait::async_trait;
use identity_application::changes::{CodeTable, Notice};
use identity_application::deps::IdentityDeps;
use identity_application::ports::{
    CodeDelivery, CodeMessage, CodeSender, DeliveryError, DeliveryQueue, MailError, Mailer,
};
use identity_application::usecases::delivery::{DeliveryJob, RecordDelivery};
use identity_enterprise::channel::{Channel, Destination};
use identity_enterprise::otp::StepUpAction;
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

/// Queue side, held in `IdentityDeps`.
pub struct ChannelDeliveryQueue {
    tx: mpsc::UnboundedSender<DeliveryJob>,
}

impl DeliveryQueue for ChannelDeliveryQueue {
    fn dispatch(&self, job: DeliveryJob) {
        if self.tx.send(job).is_err() {
            tracing::error!("code delivery worker is not running; the code is lost");
        }
    }
}

/// Worker side: started once by `app` with the deps (which hold the queue).
pub struct DeliveryWorker {
    rx: mpsc::UnboundedReceiver<DeliveryJob>,
}

pub fn delivery_channel() -> (ChannelDeliveryQueue, DeliveryWorker) {
    let (tx, rx) = mpsc::unbounded_channel();
    (ChannelDeliveryQueue { tx }, DeliveryWorker { rx })
}

impl DeliveryWorker {
    /// Sends every job on its own task and records the outcome. Runs until the queue closes.
    pub async fn run(mut self, sender: Arc<dyn CodeSender>, deps: IdentityDeps) {
        while let Some(job) = self.rx.recv().await {
            let sender = sender.clone();
            let deps = deps.clone();
            tokio::spawn(async move {
                let outcome = if job.suppressed {
                    Ok(())
                } else {
                    sender.send(&job.delivery).await
                };
                if let Err(e) = &outcome {
                    tracing::warn!(
                        purpose = job.purpose.as_str(),
                        channel = job.delivery.destination.channel().as_str(),
                        table = match job.table {
                            CodeTable::OneTime => "one_time_code",
                            CodeTable::Signup => "signup_code",
                        },
                        class = e.class(),
                        "code delivery failed"
                    );
                }
                if let Err(e) = RecordDelivery::new(deps).handle(&job, outcome).await {
                    tracing::error!(error = %e, "recording a delivery outcome failed");
                }
            });
        }
    }
}

/// No provider: every code send fails (delivery `failed`), every notice is dropped with a warning.
pub struct DisabledSender;

#[async_trait]
impl CodeSender for DisabledSender {
    async fn send(&self, _delivery: &CodeDelivery) -> Result<(), DeliveryError> {
        Err(DeliveryError::Disabled)
    }
}

/// Sends the e-mail notices of `identity.notice_requested`.
#[async_trait]
pub trait NoticeSender: Send + Sync {
    async fn send_notice(&self, locale: Locale, notice: &Notice) -> Result<(), DeliveryError>;
}

#[async_trait]
impl NoticeSender for DisabledSender {
    async fn send_notice(&self, _locale: Locale, _notice: &Notice) -> Result<(), DeliveryError> {
        Err(DeliveryError::Disabled)
    }
}

/// Plain SMTP to a development mail catcher (Mailpit). No TLS, no auth: never outside dev/test.
pub struct SmtpDevSender {
    host: String,
    port: u16,
    from: String,
}

impl SmtpDevSender {
    pub fn new(host: impl Into<String>, port: u16, from: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port,
            from: from.into(),
        }
    }

    async fn deliver(&self, to: &str, subject: &str, body: &str) -> Result<(), DeliveryError> {
        smtp_send(&self.host, self.port, &self.from, to, subject, body)
            .await
            .map_err(|e| DeliveryError::Failed(e.to_owned()))
    }
}

#[async_trait]
impl CodeSender for SmtpDevSender {
    async fn send(&self, delivery: &CodeDelivery) -> Result<(), DeliveryError> {
        let Destination::Email(to) = &delivery.destination else {
            return Err(DeliveryError::Disabled);
        };
        let (subject, body) = code_message(delivery);
        self.deliver(to.normalized(), &subject, body.expose()).await
    }
}

#[async_trait]
impl NoticeSender for SmtpDevSender {
    async fn send_notice(&self, locale: Locale, notice: &Notice) -> Result<(), DeliveryError> {
        let (to, subject, body) = notice_message(locale, notice);
        match to {
            Destination::Email(e) => self.deliver(e.normalized(), &subject, &body).await,
            // No WhatsApp sender in V1 (Q8).
            _ => Err(DeliveryError::Disabled),
        }
    }
}

#[async_trait]
impl Mailer for SmtpDevSender {
    async fn send_invitation(
        &self,
        to: &str,
        locale: Locale,
        invite_url: &SecretString,
    ) -> Result<(), MailError> {
        let (subject, intro) = match locale {
            Locale::PtBr => (
                "Convite para o All Win",
                "Você foi convidado para o All Win.",
            ),
            Locale::En => (
                "Your All Win invitation",
                "You have been invited to All Win.",
            ),
        };
        let body = format!("{intro}\n\n{}\n", invite_url.expose());
        self.deliver(to, subject, &body)
            .await
            .map_err(|_| MailError)
    }
}

/// Minimal SMTP dialogue (RFC 5321) for a local catcher. Errors carry a class only.
async fn smtp_send(
    host: &str,
    port: u16,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), &'static str> {
    let stream = TcpStream::connect((host, port))
        .await
        .map_err(|_| "connect")?;
    let (read, mut write) = stream.into_split();
    let mut lines = BufReader::new(read).lines();
    async fn expect(
        lines: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>,
        code: &str,
    ) -> Result<(), &'static str> {
        loop {
            let line = lines
                .next_line()
                .await
                .map_err(|_| "read")?
                .ok_or("closed")?;
            if line.len() < 4 || !line.starts_with(code) {
                return Err("rejected");
            }
            if line.as_bytes()[3] == b' ' {
                return Ok(());
            }
        }
    }
    let clean = |s: &str| s.replace(['\r', '\n'], " ");
    expect(&mut lines, "220").await?;
    for (cmd, code) in [
        ("EHLO allwin-core\r\n".to_owned(), "250"),
        (format!("MAIL FROM:<{}>\r\n", clean(from)), "250"),
        (format!("RCPT TO:<{}>\r\n", clean(to)), "250"),
        ("DATA\r\n".to_owned(), "354"),
    ] {
        write.write_all(cmd.as_bytes()).await.map_err(|_| "write")?;
        expect(&mut lines, code).await?;
    }
    let mut data = format!(
        "From: All Win <{}>\r\nTo: <{}>\r\nSubject: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n",
        clean(from),
        clean(to),
        clean(subject)
    );
    for line in body.lines() {
        // Dot-stuffing.
        if line.starts_with('.') {
            data.push('.');
        }
        data.push_str(line);
        data.push_str("\r\n");
    }
    data.push_str(".\r\n");
    write
        .write_all(data.as_bytes())
        .await
        .map_err(|_| "write")?;
    expect(&mut lines, "250").await?;
    write.write_all(b"QUIT\r\n").await.map_err(|_| "write")?;
    Ok(())
}

fn action_text(action: StepUpAction, locale: Locale) -> &'static str {
    let pt = matches!(locale, Locale::PtBr);
    match action {
        StepUpAction::SourceCreate => {
            if pt {
                "criar uma fonte de sinais"
            } else {
                "create a signal source"
            }
        }
        StepUpAction::SourceRotate => {
            if pt {
                "trocar a URL de uma fonte de sinais"
            } else {
                "rotate a signal source URL"
            }
        }
        StepUpAction::ConnectionCreate => {
            if pt {
                "criar uma conexão de plataforma"
            } else {
                "create a platform connection"
            }
        }
        StepUpAction::PairingCodeNew => {
            if pt {
                "gerar um código de pareamento"
            } else {
                "create a pairing code"
            }
        }
        StepUpAction::ConnectionUnpair => {
            if pt {
                "desparear uma instância"
            } else {
                "unpair an instance"
            }
        }
        StepUpAction::AccountOptInAccept | StepUpAction::AccountCreateOptInAccept => {
            if pt {
                "ativar a automação em uma conta"
            } else {
                "turn on automation on an account"
            }
        }
        StepUpAction::AdminInvitationCreate => {
            if pt {
                "emitir um convite"
            } else {
                "issue an invitation"
            }
        }
        StepUpAction::RouteEnable => {
            if pt {
                "ativar uma rota automatizada"
            } else {
                "enable an automated route"
            }
        }
        StepUpAction::AccountBindingChange => {
            if pt {
                "trocar a conta vinculada"
            } else {
                "change the bound account"
            }
        }
    }
}

/// Subject and body of a code message (policy §2.3: what it is for, never asked by the team, no link).
pub fn code_message(d: &CodeDelivery) -> (String, SecretString) {
    let pt = matches!(d.locale, Locale::PtBr);
    let purpose = match &d.message {
        CodeMessage::Signup => {
            if pt {
                "para concluir o cadastro".to_owned()
            } else {
                "to finish signing up".to_owned()
            }
        }
        CodeMessage::Login {
            device_label,
            requested_at,
        } => {
            let device = device_label.as_deref().unwrap_or("?");
            let at = requested_at.format("%Y-%m-%d %H:%M UTC");
            if pt {
                format!("para entrar (pedido em {device}, {at})")
            } else {
                format!("to sign in (requested from {device}, {at})")
            }
        }
        CodeMessage::StepUp(a) => {
            if pt {
                format!("para {}", action_text(*a, d.locale))
            } else {
                format!("to {}", action_text(*a, d.locale))
            }
        }
        CodeMessage::ChannelChange { channel } => match (channel, pt) {
            (Channel::Email, true) => "para trocar o e-mail".to_owned(),
            (Channel::Whatsapp, true) => "para trocar o WhatsApp".to_owned(),
            (Channel::Email, false) => "to change your e-mail".to_owned(),
            (Channel::Whatsapp, false) => "to change your WhatsApp number".to_owned(),
        },
    };
    let (subject, never) = if pt {
        (
            "Seu código All Win",
            "A equipe All Win nunca pede este código.",
        )
    } else {
        (
            "Your All Win code",
            "The All Win team will never ask you for this code.",
        )
    };
    let body = if pt {
        format!(
            "Seu código {purpose}: {}\n\nVale por 10 minutos.\n{never}\n",
            d.code.expose()
        )
    } else {
        format!(
            "Your code {purpose}: {}\n\nValid for 10 minutes.\n{never}\n",
            d.code.expose()
        )
    };
    (subject.to_owned(), SecretString::new(body))
}

/// Recipient, subject and body of a notice (policy §5.3, §7, §9). No action links.
pub fn notice_message(locale: Locale, notice: &Notice) -> (Destination, String, String) {
    let pt = matches!(locale, Locale::PtBr);
    match notice {
        Notice::NewDevice {
            to,
            device_label,
            ip,
            channel,
        } => {
            let ip = ip.map(|i| i.to_string()).unwrap_or_default();
            let (subject, body) = if pt {
                (
                    "Novo acesso à sua conta All Win",
                    format!(
                        "Houve um acesso de um novo dispositivo: {device_label}, IP {ip}, código enviado por {}.\n\nNão foi você? Entre, abra Sessões e use \"Sair de todos os dispositivos\"; depois revise tokens de fontes e pareamentos NT8 que você não reconhece e a segurança do seu e-mail.\n",
                        channel.as_str()
                    ),
                )
            } else {
                (
                    "New sign-in to your All Win account",
                    format!(
                        "A new device signed in: {device_label}, IP {ip}, code sent by {}.\n\nNot you? Sign in, open Sessions and use \"Sign out of all devices\"; then review source tokens and NT8 pairings you do not recognize, and your e-mail account's security.\n",
                        channel.as_str()
                    ),
                )
            };
            (to.clone(), subject.to_owned(), body)
        }
        Notice::ChannelChanged {
            to,
            channel,
            masked_new_value,
        } => {
            let body = if pt {
                format!(
                    "Seu {} foi trocado para {masked_new_value}. Não foi você? Fale com o suporte.\n",
                    channel.as_str()
                )
            } else {
                format!(
                    "Your {} was changed to {masked_new_value}. Not you? Contact support.\n",
                    channel.as_str()
                )
            };
            let subject = if pt {
                "Canal da conta trocado"
            } else {
                "Account channel changed"
            };
            (to.clone(), subject.to_owned(), body)
        }
        Notice::ChannelChangeOnHold {
            to,
            channel,
            masked_new_value,
            hold_until,
        } => {
            let at = hold_until.format("%Y-%m-%d %H:%M UTC");
            let body = if pt {
                format!(
                    "Uma troca do seu {} para {masked_new_value} vale a partir de {at}. Não foi você? Entre e use \"Cancelar troca\".\n",
                    channel.as_str()
                )
            } else {
                format!(
                    "A change of your {} to {masked_new_value} takes effect at {at}. Not you? Sign in and use \"Cancel change\".\n",
                    channel.as_str()
                )
            };
            let subject = if pt {
                "Troca de canal pendente"
            } else {
                "Pending channel change"
            };
            (to.clone(), subject.to_owned(), body)
        }
        Notice::ChannelChangeCancelled { to, channel } => {
            let body = if pt {
                format!("A troca do seu {} foi cancelada.\n", channel.as_str())
            } else {
                format!("The change of your {} was cancelled.\n", channel.as_str())
            };
            let subject = if pt {
                "Troca de canal cancelada"
            } else {
                "Channel change cancelled"
            };
            (to.clone(), subject.to_owned(), body)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_enterprise::email::EmailAddress;

    #[test]
    fn messages_say_what_the_code_is_for_and_carry_no_link() {
        let d = CodeDelivery {
            destination: Destination::Email(EmailAddress::parse("a@example.com").unwrap()),
            locale: Locale::PtBr,
            code: SecretString::new("K7Q2-0111"),
            message: CodeMessage::StepUp(StepUpAction::ConnectionUnpair),
        };
        let (subject, body) = code_message(&d);
        assert_eq!(subject, "Seu código All Win");
        assert!(body.expose().contains("desparear") && body.expose().contains("K7Q2-0111"));
        assert!(!body.expose().contains("http"));
        assert!(!format!("{d:?}").contains("K7Q2"));
    }

    #[test]
    fn new_device_notice_goes_to_the_account_email() {
        let to = Destination::Email(EmailAddress::parse("ana@example.com").unwrap());
        let n = Notice::NewDevice {
            to: to.clone(),
            device_label: "Firefox on Linux".into(),
            ip: None,
            channel: Channel::Email,
        };
        let (dest, subject, _) = notice_message(Locale::En, &n);
        assert_eq!(dest, to);
        assert_eq!(subject, "New sign-in to your All Win account");
    }

    #[tokio::test]
    async fn disabled_sender_fails_every_send() {
        let d = CodeDelivery {
            destination: Destination::Email(EmailAddress::parse("a@example.com").unwrap()),
            locale: Locale::En,
            code: SecretString::new("K7Q2-0111"),
            message: CodeMessage::Signup,
        };
        assert!(matches!(
            DisabledSender.send(&d).await,
            Err(DeliveryError::Disabled)
        ));
    }

    #[tokio::test]
    async fn smtp_dialogue_against_a_fake_server() {
        use tokio::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (s, _) = listener.accept().await.unwrap();
            let (r, mut w) = s.into_split();
            let mut lines = BufReader::new(r).lines();
            w.write_all(b"220 hi\r\n").await.unwrap();
            let mut got = String::new();
            let mut in_data = false;
            while let Some(line) = lines.next_line().await.unwrap() {
                got.push_str(&line);
                got.push('\n');
                if in_data {
                    if line == "." {
                        in_data = false;
                        w.write_all(b"250 queued\r\n").await.unwrap();
                    }
                    continue;
                }
                match line.split(' ').next().unwrap_or_default() {
                    "DATA" => {
                        in_data = true;
                        w.write_all(b"354 go\r\n").await.unwrap();
                    }
                    "QUIT" => break,
                    _ => w.write_all(b"250 ok\r\n").await.unwrap(),
                }
            }
            got
        });
        let sender = SmtpDevSender::new("127.0.0.1", port, "core@allwin.test");
        let d = CodeDelivery {
            destination: Destination::Email(EmailAddress::parse("a@example.com").unwrap()),
            locale: Locale::En,
            code: SecretString::new("K7Q2-0111"),
            message: CodeMessage::Signup,
        };
        sender.send(&d).await.unwrap();
        let got = server.await.unwrap();
        assert!(got.contains("RCPT TO:<a@example.com>") && got.contains("K7Q2-0111"));
    }
}
