//! `adapter-sim --url ws://127.0.0.1:8080/adapter/v1 --token-file <path> [--account Sim101]
//!  [--script <scenario.json>] [--no-ack]`
//! The token is read from a file (never from the command line, which other users can see) and
//! never printed. Reconnects with exponential backoff (1 s → 30 s) like the plugin, and stops on
//! a close that tells the plugin to stop (4001–4004, or 4100 sent by itself).

use std::time::Duration;

use adapter_sim::{Script, SimClient, SimConfig, SimEnd, Venue};
use anyhow::{Context, bail};
use uuid::Uuid;

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
    let account = arg(&args, "--account").unwrap_or_else(|| "Sim101".into());
    let mut cfg = SimConfig::new(url, token);
    cfg.no_ack = args.iter().any(|a| a == "--no-ack");
    let mut client = SimClient::new(Venue::new(Uuid::nil(), account, script));
    let mut backoff = Duration::from_secs(1);
    loop {
        match client.run_session(&cfg).await {
            Ok(SimEnd::Closed(Some(code @ 4001..=4004))) => {
                tracing::error!(code, "the Core asked the adapter to stop");
                return Ok(());
            }
            Ok(SimEnd::UserDisconnected) => return Ok(()),
            Ok(end) => tracing::warn!(?end, "session ended; reconnecting"),
            Err(e) => tracing::warn!(error = %e, "session failed; reconnecting"),
        }
        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(Duration::from_secs(30));
    }
}
