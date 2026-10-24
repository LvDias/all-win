//! `adapter-sim --url ws://127.0.0.1:8080/adapter/v1 --token-file <path>
//!  [--account Sim101[:kind]]... [--kind <kind>] [--script <scenario.json>] [--no-ack]
//!  [--drop-after <n> [--drop-before-ack]]`
//! The first `--account` is the venue's (it executes); later ones are only announced in `hello`
//! with their kind (`simulation` by default), e.g. `--account Sim101 --account Live1:live`.
//! `--kind` sets the first account's kind.
//! The token is read from a file (never from the command line, which other users can see) and
//! never printed. Reconnects with exponential backoff (1 s → 30 s) like the plugin, and stops on
//! a close that tells the plugin to stop (4001–4004, or 4100 sent by itself).

use std::time::Duration;

use adapter_sim::{Script, SimClient, SimConfig, SimEnd, Venue};
use anyhow::{Context, bail};
use uuid::Uuid;

fn all(args: &[String], name: &str) -> Vec<String> {
    args.windows(2)
        .filter(|w| w[0] == name)
        .map(|w| w[1].clone())
        .collect()
}

/// `NAME[:kind]`.
fn account(spec: &str) -> (String, String) {
    match spec.split_once(':') {
        Some((name, kind)) => (name.to_owned(), kind.to_owned()),
        None => (spec.to_owned(), "simulation".to_owned()),
    }
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).init();
    let args: Vec<String> = std::env::args().collect();
    let Some(url) = arg(&args, "--url") else {
        bail!("--url is required")
    };
    let Some(token_file) = arg(&args, "--token-file") else {
        bail!("--token-file is required")
    };
    let token = std::fs::read_to_string(&token_file)
        .context("token file")?
        .trim()
        .to_owned();
    let script: Script = match arg(&args, "--script") {
        Some(p) => serde_json::from_str(&std::fs::read_to_string(p).context("script")?)
            .context("script json")?,
        None => Script::default(),
    };
    let mut accounts: Vec<(String, String)> =
        all(&args, "--account").iter().map(|s| account(s)).collect();
    if accounts.is_empty() {
        accounts.push(("Sim101".into(), "simulation".into()));
    }
    let (account, mut kind) = accounts.remove(0);
    if let Some(k) = arg(&args, "--kind") {
        kind = k;
    }
    let mut cfg = SimConfig::new(url, token);
    cfg.kind = kind;
    cfg.extra_accounts = accounts;
    cfg.no_ack = args.iter().any(|a| a == "--no-ack");
    cfg.drop_after_commands = match arg(&args, "--drop-after") {
        Some(n) => Some(n.parse().context("--drop-after")?),
        None => None,
    };
    cfg.drop_before_ack = args.iter().any(|a| a == "--drop-before-ack");
    tracing::info!(config = ?cfg, "adapter-sim");
    let mut client = SimClient::new(Venue::new(Uuid::nil(), account, script));
    let mut backoff = Duration::from_secs(1);
    loop {
        match client.run_session(&cfg).await {
            Ok(SimEnd::Closed(Some(code @ 4001..=4004))) => {
                tracing::error!(code, "the Core asked the adapter to stop");
                return Ok(());
            }
            Ok(SimEnd::UserDisconnected) => return Ok(()),
            Ok(SimEnd::Dropped) => {
                // A scripted drop happens once; the next session runs normally.
                tracing::warn!("dropped on purpose; reconnecting");
                cfg.drop_after_commands = None;
            }
            Ok(end) => tracing::warn!(?end, "session ended; reconnecting"),
            Err(e) => tracing::warn!(error = %e, "session failed; reconnecting"),
        }
        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(Duration::from_secs(30));
    }
}
