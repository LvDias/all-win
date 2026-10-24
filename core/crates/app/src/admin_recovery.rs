//! Operator command `allwin-core admin recovery <start|confirm|status|cancel>` (S3 AC-12, auth
//! policy §9). A thin adapter over identity's admin-recovery use cases: it runs only on the host
//! (`ssh` then `docker compose exec -it core allwin-core admin recovery …`) and has no HTTP route.
//!
//! - Codes are typed at a prompt on `/dev/tty` with echo off, never passed as arguments (shell
//!   history, `ps`). The admin's step-up challenge stays in this process's memory.
//! - Output shows masked values only. The operator log line names the acting admin's id, which
//!   never reaches the user's trail.
//! - Codes are sent synchronously before the command exits (the server's delivery worker does
//!   not run here); notices go through the outbox like any other.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use anyhow::{Context, bail};
use identity_application::changes::HostReport;
use identity_application::deps::IdentityDeps;
use identity_application::error::IdentityError;
use identity_application::ports::{CodeSender, DeliveryQueue, SourceKeyListener};
use identity_application::usecases::admin_recovery::{
    CancelAdminRecovery, ConfirmAdminRecovery, ConfirmRecoveryCommand, Operator, RecoveryError,
    RecoveryStatus, RecoveryView, RequestOperatorStepUp, ResolveOperator, ResolveOperatorCommand,
    StartAdminRecovery, StartRecoveryCommand, StepUpProof, UserRef,
};
use identity_application::usecases::delivery::{DeliveryJob, RecordDelivery};
use identity_enterprise::channel_change::{
    ChangeStatus, VERIFICATION_REF_MAX_CHARS, VerificationRef, VerifiedBy,
};
use identity_enterprise::ids::ChannelChangeId;
use identity_enterprise::tokens::KeyId;
use shared_kernel::{SystemClock, UserId};
use uuid::Uuid;

use crate::config::Config;

pub const USAGE: &str = "\
usage: allwin-core admin recovery <start|confirm|status|cancel> --user <e-mail|user id>
                                  (--admin <admin e-mail> | --break-glass) [options]
  start   --new-email <e-mail> --verified-by <video_call|phone_known_number|in_person|other>
          --verification-ref <text>   plain text, at most 200 characters: a ticket or call
                                      reference. It is shown to the user: never an e-mail,
                                      a phone, a document number or anyone's personal data
  confirm --change-id <id>            prompts for the code the user read from the new e-mail
  status                              the user's latest channel change (no code)
  cancel  --change-id <id>
Without --break-glass every action except status prompts for a fresh code sent to the admin's
e-mail. --break-glass is only for a platform admin recovering their own account when no other
active admin exists (SSH only, through the host wrapper that reports the SSH session to the
operator log): it trusts whoever has a shell on the host, so it raises an alert and e-mails the
account's channels, and the 72-h hold and the notices to every old channel still apply. Codes are never accepted as arguments. Run with a terminal: docker compose exec -it core ...";

/// How many times `confirm` asks again for the user's code after a wrong one.
const CONFIRM_PROMPTS: usize = 5;

#[derive(Debug, Default, PartialEq, Eq)]
struct Args {
    action: String,
    user: Option<String>,
    admin: Option<String>,
    break_glass: bool,
    new_email: Option<String>,
    verified_by: Option<String>,
    verification_ref: Option<String>,
    change_id: Option<String>,
}

fn parse(args: &[String]) -> anyhow::Result<Args> {
    let mut it = args.iter();
    let mut out = Args {
        action: it.next().cloned().unwrap_or_default(),
        ..Args::default()
    };
    if !matches!(
        out.action.as_str(),
        "start" | "confirm" | "status" | "cancel"
    ) {
        bail!("{USAGE}");
    }
    while let Some(flag) = it.next() {
        let mut value = || {
            it.next()
                .cloned()
                .with_context(|| format!("{flag} needs a value"))
        };
        match flag.as_str() {
            "--user" => out.user = Some(value()?),
            "--admin" => out.admin = Some(value()?),
            "--break-glass" => out.break_glass = true,
            "--new-email" => out.new_email = Some(value()?),
            "--verified-by" => out.verified_by = Some(value()?),
            "--verification-ref" => out.verification_ref = Some(value()?),
            "--change-id" => out.change_id = Some(value()?),
            "--code" | "--step-up-code" => {
                bail!("codes are typed at the prompt, never as arguments")
            }
            other => bail!("unknown option {other}\n{USAGE}"),
        }
    }
    if out.break_glass == out.admin.is_some() {
        bail!("give exactly one of --admin and --break-glass\n{USAGE}");
    }
    Ok(out)
}

fn user_ref(raw: &str) -> UserRef {
    match Uuid::parse_str(raw) {
        Ok(id) => UserRef::Id(UserId::from_uuid(id)),
        Err(_) => UserRef::Email(raw.to_owned()),
    }
}

fn change_id(args: &Args) -> anyhow::Result<ChannelChangeId> {
    let raw = args
        .change_id
        .as_deref()
        .context("--change-id is required")?;
    Ok(ChannelChangeId::from_uuid(
        Uuid::parse_str(raw).context("--change-id is not a uuid")?,
    ))
}

/// Runs `stty <arg>` on the controlling terminal.
fn stty(arg: &str) -> bool {
    File::open("/dev/tty")
        .ok()
        .and_then(|t| {
            Command::new("stty")
                .arg(arg)
                .stdin(t)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .ok()
        })
        .is_some_and(|s| s.success())
}

/// A terminal that can hide what is typed: the only way a code enters the command (never argv,
/// env or stdin, and the command never prints a code, so the wrapper's syslog copy holds none).
trait HiddenInput {
    /// Turns echo off; `false` when it cannot.
    fn hide(&mut self) -> bool;
    /// Turns echo back on.
    fn show(&mut self);
    fn read_line(&mut self) -> std::io::Result<String>;
}

/// Turns echo off or refuses: a line is never read while echo may be on.
fn hide_or_refuse(input: &mut impl HiddenInput) -> anyhow::Result<()> {
    if input.hide() {
        Ok(())
    } else {
        bail!("cannot turn terminal echo off (stty); the code is not read")
    }
}

/// Reads one line of hidden input and shows the echo again, whatever the read did.
fn read_then_show(input: &mut impl HiddenInput) -> anyhow::Result<String> {
    let line = input.read_line();
    input.show();
    Ok(line.context("reading the code")?.trim().to_owned())
}

/// The controlling terminal, `/dev/tty`.
struct Tty(File);

impl HiddenInput for Tty {
    fn hide(&mut self) -> bool {
        stty("-echo")
    }

    fn show(&mut self) {
        stty("echo");
    }

    fn read_line(&mut self) -> std::io::Result<String> {
        let mut line = String::new();
        BufReader::new(&self.0).read_line(&mut line)?;
        Ok(line)
    }
}

/// Reads one code from the terminal with echo off. Without a terminal the command refuses.
/// Ctrl-C restores the echo and stops the command at once (exit 130, Brasa N-2).
async fn prompt_secret(label: &str) -> anyhow::Result<String> {
    let mut tty = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .context("an interactive terminal is required (docker compose exec -it)")?;
    write!(tty, "{label}: ")?;
    tty.flush()?;
    let mut input = Tty(tty.try_clone().context("terminal")?);
    // Echo goes off here, before the blocking read starts, so Ctrl-C can never race a late
    // `stty -echo` on the reading thread (Brasa R3-4).
    hide_or_refuse(&mut input)?;
    let read = tokio::task::spawn_blocking(move || read_then_show(&mut input));
    let line = tokio::select! {
        r = read => r.context("reading the code")??,
        _ = tokio::signal::ctrl_c() => {
            stty("echo");
            let _ = writeln!(tty, "\ninterrupted");
            std::process::exit(130);
        }
    };
    writeln!(tty)?;
    Ok(line)
}

fn say(text: &str) -> anyhow::Result<()> {
    writeln!(std::io::stdout().lock(), "{text}").context("stdout")
}

fn show(v: &RecoveryView) -> anyhow::Result<()> {
    let status = match v.status {
        ChangeStatus::AwaitingCodes => "awaiting_codes",
        ChangeStatus::OnHold => "on_hold",
        ChangeStatus::Completed => "completed",
        ChangeStatus::Cancelled => "cancelled",
        ChangeStatus::Expired => "expired",
    };
    let mut out = format!(
        "change_id: {}\nkind: {}\nstatus: {status}\nnew e-mail: {}\ncreated_at: {}\ncode expires_at: {}",
        v.change_id.as_uuid(),
        v.kind.as_str(),
        v.masked_new_value,
        v.created_at.to_rfc3339(),
        v.expires_at.to_rfc3339(),
    );
    if let Some(h) = v.hold_until {
        out.push_str(&format!("\nhold_until: {}", h.to_rfc3339()));
    }
    if v.break_glass {
        out.push_str("\nbreak_glass: true");
    }
    say(&out)
}

/// The jobs of this process, sent synchronously after each use case.
#[derive(Default)]
struct CollectedDelivery(Mutex<Vec<DeliveryJob>>);

impl DeliveryQueue for CollectedDelivery {
    fn dispatch(&self, job: DeliveryJob) {
        if let Ok(mut jobs) = self.0.lock() {
            jobs.push(job);
        }
    }
}

/// The command creates no source key.
struct NoKeys;

impl SourceKeyListener for NoKeys {
    fn key_activated(&self, _: &KeyId) {}
}

struct Cli {
    deps: IdentityDeps,
    sender: Arc<dyn CodeSender>,
    queue: Arc<CollectedDelivery>,
}

impl Cli {
    /// Sends what the last use case queued; `false` when a send failed.
    async fn flush(&self) -> anyhow::Result<bool> {
        let jobs = self
            .queue
            .0
            .lock()
            .map(|mut j| std::mem::take(&mut *j))
            .unwrap_or_default();
        let mut all_sent = true;
        for job in jobs {
            let outcome = if job.suppressed {
                Ok(())
            } else {
                self.sender.send(&job.delivery).await
            };
            all_sent &= outcome.is_ok();
            RecordDelivery::new(self.deps.clone())
                .handle(&job, outcome)
                .await
                .context("recording the delivery")?;
        }
        Ok(all_sent)
    }

    /// A fresh admin step-up for `target`, prompted on the terminal; `None` on break-glass.
    async fn step_up(&self, op: &Operator, target: Uuid) -> anyhow::Result<Option<StepUpProof>> {
        if op.break_glass() {
            return Ok(None);
        }
        let issued = RequestOperatorStepUp::new(self.deps.clone())
            .handle(op, target)
            .await
            .map_err(explain)?;
        if !self.flush().await? {
            bail!("the admin's code could not be sent (check the e-mail provider)");
        }
        say(&format!(
            "A code was sent to the admin's e-mail {}.",
            op.masked_step_up_channel()
        ))?;
        let code = prompt_secret("Admin code").await?;
        Ok(Some(StepUpProof {
            challenge_id: issued.challenge_id.expose().to_owned(),
            code,
        }))
    }
}

fn explain(e: RecoveryError) -> anyhow::Error {
    match e {
        RecoveryError::Identity(IdentityError::RateLimited { retry_after }) => {
            anyhow::anyhow!("too many codes; try again in {} s", retry_after.as_secs())
        }
        RecoveryError::Identity(IdentityError::Internal(_)) => {
            anyhow::anyhow!("internal error (see the server log)")
        }
        other => anyhow::anyhow!("{other}"),
    }
}

/// The host wrapper's report of the SSH session (policy 3.3 §9.1 b; formats validated by
/// `HostReport::parse`). Host-reported: anyone with a host shell can set it.
fn host_report(get: impl Fn(&str) -> Option<String>) -> HostReport {
    HostReport::parse(
        get("ALLWIN_OPERATOR_LOGIN").as_deref(),
        get("ALLWIN_OPERATOR_SSH_CONNECTION").as_deref(),
        get("ALLWIN_OPERATOR_SSH_KEY").as_deref(),
    )
}

/// The command's own log line (its terminal); the durable record is the
/// `identity.operator_recovery` event the use case commits.
fn operator_log(action: &str, op: &Operator, user: UserId, change: Option<ChannelChangeId>) {
    let host = op.host();
    tracing::info!(
        target: "operator",
        action,
        admin_user_id = %op.admin_id().as_uuid(),
        user_id = %user.as_uuid(),
        change_id = change.map(|c| c.as_uuid().to_string()).unwrap_or_default(),
        break_glass = op.break_glass(),
        host_login = host.login().unwrap_or("-"),
        host_ssh_connection = host.ssh_connection().unwrap_or("-"),
        host_ssh_key = host.key_fingerprint().unwrap_or("-"),
        "admin recovery"
    );
}

/// `admin recovery …` (args after `recovery`).
pub async fn run(args: &[String]) -> anyhow::Result<()> {
    let args = parse(args)?;
    let host = host_report(|k| std::env::var(k).ok());
    let config = Config::from_env().context("configuration")?;
    let Some(cfg) = &config.identity else {
        bail!("identity is not configured (IDENTITY_PEPPERS)");
    };
    let db = crate::connect(&config.database_url, 2)
        .await
        .context("DATABASE_URL")?;
    let worker = crate::connect(&config.worker_database_url, 1)
        .await
        .context("WORKER_DATABASE_URL")?;
    let queue = Arc::new(CollectedDelivery::default());
    let parts = crate::identity::parts(
        &config,
        cfg,
        &db,
        &worker,
        Arc::new(SystemClock),
        Arc::new(NoKeys),
        queue.clone(),
    )?;
    let cli = Cli {
        deps: parts.deps,
        sender: parts.sender,
        queue,
    };
    let user = args.user.as_deref().context("--user is required")?;
    let (op, target) = ResolveOperator::new(cli.deps.clone())
        .handle(ResolveOperatorCommand {
            admin_email: args.admin.clone(),
            break_glass: args.break_glass,
            user: user_ref(user),
            host,
        })
        .await
        .map_err(explain)?;
    match args.action.as_str() {
        "start" => {
            let new_email = args.new_email.clone().context("--new-email is required")?;
            let verified_by = args
                .verified_by
                .as_deref()
                .and_then(VerifiedBy::parse)
                .context(
                    "--verified-by must be video_call, phone_known_number, in_person or other",
                )?;
            let verification_ref = VerificationRef::parse(
                args.verification_ref.as_deref().unwrap_or_default(),
            )
            .map_err(|_| {
                anyhow::anyhow!(
                    "--verification-ref: 1 to {VERIFICATION_REF_MAX_CHARS} characters of plain text, without e-mail, phone, document numbers or tokens"
                )
            })?;
            let step_up = cli.step_up(&op, target.id.as_uuid()).await?;
            let view = StartAdminRecovery::new(cli.deps.clone())
                .handle(
                    &op,
                    &target,
                    StartRecoveryCommand {
                        new_email,
                        verified_by,
                        verification_ref,
                        step_up,
                    },
                )
                .await
                .map_err(explain)?;
            operator_log("start", &op, target.id, Some(view.change_id));
            let sent = cli.flush().await?;
            show(&view)?;
            if sent {
                say(
                    "A code was sent to the new e-mail. Verify the user again, ask them to read it to you, then run `confirm` within 10 minutes.",
                )?;
            } else {
                say(
                    "The code to the new e-mail could not be sent. Cancel this recovery and start again when e-mail works.",
                )?;
            }
        }
        "confirm" => {
            let id = change_id(&args)?;
            let step_up = cli.step_up(&op, id.as_uuid()).await?;
            let mut result = None;
            for _ in 0..CONFIRM_PROMPTS {
                let code = prompt_secret("Code the user read from the new e-mail").await?;
                match ConfirmAdminRecovery::new(cli.deps.clone())
                    .handle(
                        &op,
                        &target,
                        ConfirmRecoveryCommand {
                            change_id: id,
                            code,
                            step_up: step_up.clone(),
                        },
                    )
                    .await
                {
                    Ok(v) => {
                        result = Some(v);
                        break;
                    }
                    Err(e @ RecoveryError::CodeWrong { .. }) => say(&e.to_string())?,
                    Err(e) => return Err(explain(e)),
                }
            }
            let Some(view) = result else {
                bail!("not confirmed");
            };
            operator_log("confirm", &op, target.id, Some(view.change_id));
            show(&view)?;
            say(
                "The recovery is on hold; every old channel was notified. The user can cancel it from any session.",
            )?;
        }
        "status" => {
            match RecoveryStatus::new(cli.deps.clone())
                .handle(&op, &target)
                .await
                .map_err(explain)?
            {
                Some(view) => show(&view)?,
                None => say("no channel change")?,
            }
        }
        "cancel" => {
            let id = change_id(&args)?;
            let step_up = cli.step_up(&op, id.as_uuid()).await?;
            let view = CancelAdminRecovery::new(cli.deps.clone())
                .handle(&op, &target, id, step_up)
                .await
                .map_err(explain)?;
            operator_log("cancel", &op, target.id, Some(view.change_id));
            show(&view)?;
        }
        _ => bail!("{USAGE}"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    fn args(raw: &[&str]) -> anyhow::Result<Args> {
        parse(&raw.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>())
    }

    #[test]
    fn parses_flags_and_refuses_codes_as_arguments() {
        let a = args(&[
            "start",
            "--user",
            "ana@example.com",
            "--admin",
            "root@example.com",
            "--new-email",
            "n@example.com",
            "--verified-by",
            "video_call",
            "--verification-ref",
            "ticket 1",
        ])
        .unwrap();
        assert_eq!(a.action, "start");
        assert_eq!(a.admin.as_deref(), Some("root@example.com"));
        assert!(!a.break_glass);
        assert!(
            args(&[
                "confirm",
                "--user",
                "u",
                "--break-glass",
                "--change-id",
                "x"
            ])
            .unwrap()
            .break_glass
        );
        assert!(
            args(&[
                "confirm",
                "--user",
                "u",
                "--admin",
                "a",
                "--code",
                "AAAA-0000"
            ])
            .is_err()
        );
        assert!(
            args(&[
                "confirm",
                "--user",
                "u",
                "--admin",
                "a",
                "--step-up-code",
                "1"
            ])
            .is_err()
        );
        assert!(
            args(&["status", "--user", "u"]).is_err(),
            "neither --admin nor --break-glass"
        );
        assert!(args(&["status", "--user", "u", "--admin", "a", "--break-glass"]).is_err());
        assert!(args(&["delete", "--user", "u", "--admin", "a"]).is_err());
        assert!(args(&["status", "--user"]).is_err());
    }

    #[test]
    fn there_is_no_http_route_for_it() {
        let doc = crate::openapi::document().unwrap();
        let paths = doc["paths"].as_object().unwrap();
        assert!(!paths.is_empty());
        assert!(paths.keys().all(|p| !p.contains("recover")), "{paths:?}");
    }

    #[test]
    fn host_report_reads_the_wrapper_variables() {
        let env = |k: &str| match k {
            "ALLWIN_OPERATOR_LOGIN" => Some(" ops ".to_owned()),
            "ALLWIN_OPERATOR_SSH_CONNECTION" => Some("203.0.113.5 51234 10.0.0.2 22".to_owned()),
            "ALLWIN_OPERATOR_SSH_KEY" => Some(format!("SHA256:{}", "a".repeat(43))),
            _ => None,
        };
        let h = host_report(env);
        assert_eq!(h.login(), Some("ops"));
        assert_eq!(h.ssh_connection(), Some("203.0.113.5 51234 10.0.0.2 22"));
        assert!(h.key_fingerprint().is_some());
        assert_eq!(host_report(|_| None), HostReport::default());
    }

    /// A scripted terminal: whether echo can be hidden, the line typed, and what happened.
    struct FakeInput {
        can_hide: bool,
        typed: std::io::Result<String>,
        log: Vec<&'static str>,
    }

    impl HiddenInput for FakeInput {
        fn hide(&mut self) -> bool {
            self.log.push("hide");
            self.can_hide
        }
        fn show(&mut self) {
            self.log.push("show");
        }
        fn read_line(&mut self) -> std::io::Result<String> {
            self.log.push("read");
            std::mem::replace(&mut self.typed, Ok(String::new()))
        }
    }

    /// Both steps in order, as `prompt_secret` runs them.
    fn read_hidden(input: &mut impl HiddenInput) -> anyhow::Result<String> {
        hide_or_refuse(input)?;
        read_then_show(input)
    }

    fn fake(can_hide: bool, typed: std::io::Result<String>) -> FakeInput {
        FakeInput {
            can_hide,
            typed,
            log: Vec::new(),
        }
    }

    #[test]
    fn a_code_is_read_only_with_echo_off_and_echo_comes_back() {
        let mut t = fake(true, Ok(" K7Q2-0111\n".into()));
        assert_eq!(read_hidden(&mut t).unwrap(), "K7Q2-0111");
        assert_eq!(t.log, ["hide", "read", "show"]);
    }

    #[test]
    fn without_echo_off_nothing_is_read() {
        let mut t = fake(false, Ok("K7Q2-0111\n".into()));
        assert!(read_hidden(&mut t).is_err());
        assert_eq!(t.log, ["hide"], "never read while echo may be on");
    }

    #[test]
    fn echo_comes_back_after_a_failed_read() {
        let mut t = fake(true, Err(std::io::Error::other("closed")));
        assert!(read_hidden(&mut t).is_err());
        assert_eq!(t.log, ["hide", "read", "show"]);
    }

    #[test]
    fn both_codes_go_through_the_hidden_prompt_only() {
        // Structural guard: the admin's step-up code and the user's recovery code are read only
        // by `prompt_secret`, which hides first and then reads through `read_then_show`.
        let all = include_str!("admin_recovery.rs");
        let code = all.split(concat!("#[cfg", "(test)]")).next().unwrap();
        assert_eq!(
            code.matches("prompt_secret(\"").count(),
            2,
            "admin code + recovery code"
        );
        assert_eq!(
            code.matches("read_then_show(&mut ").count(),
            1,
            "only prompt_secret reads"
        );
        let prompt = &code[code.find("async fn prompt_secret").unwrap()..];
        let hide = prompt.find("hide_or_refuse(&mut input)?").unwrap();
        let spawn = prompt.find("spawn_blocking").unwrap();
        assert!(hide < spawn, "echo is off before the reading thread starts");
        assert!(!code.contains("io::stdin"), "codes never come from stdin");
    }

    #[test]
    fn user_is_an_id_or_an_email() {
        let id = Uuid::now_v7();
        assert_eq!(
            user_ref(&id.to_string()),
            UserRef::Id(UserId::from_uuid(id))
        );
        assert_eq!(
            user_ref("ana@example.com"),
            UserRef::Email("ana@example.com".into())
        );
    }
}
