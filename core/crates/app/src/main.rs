use std::io::Write;

use anyhow::{Context, bail};
use app::config::{Config, SecretString};
use app::{App, MIGRATOR, dev, http, telemetry};

/// `allwin-core [serve | migrate | openapi | seed [LABEL] | revoke-key KEY_ID]`
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("serve") => serve().await,
        Some("migrate") => {
            telemetry::init();
            migrate(&owner_url()?).await
        }
        Some("openapi") => {
            // The generated spec as JSON on stdout (CI diffs it against the contract).
            let doc = app::openapi::document().context("openapi")?;
            let text = serde_json::to_string_pretty(&doc).context("openapi json")?;
            writeln!(std::io::stdout().lock(), "{text}").context("stdout")?;
            Ok(())
        }
        Some("seed") => {
            let label = args
                .get(1)
                .cloned()
                .or_else(|| std::env::var("SEED_LABEL").ok())
                .unwrap_or_else(|| "a".to_owned());
            let owner = dev_owner_pool().await?;
            let token = dev::seed(&owner, &label).await?;
            // The token goes to stdout exactly once; nothing else is printed there.
            writeln!(std::io::stdout().lock(), "{token}").context("stdout")?;
            Ok(())
        }
        Some("revoke-key") => {
            let key_id = args
                .get(1)
                .cloned()
                .or_else(|| std::env::var("KEY_ID").ok())
                .context("revoke-key KEY_ID")?;
            let owner = dev_owner_pool().await?;
            if !dev::revoke_key(&owner, &key_id).await? {
                bail!("no active key with this key_id");
            }
            Ok(())
        }
        Some(_) => bail!(
            "usage: allwin-core [serve | migrate | openapi | seed [LABEL] | revoke-key KEY_ID]"
        ),
    }
}

async fn serve() -> anyhow::Result<()> {
    telemetry::init();
    let config = Config::from_env().context("configuration")?;
    if let Some(owner) = &config.migration_database_url {
        migrate(owner).await?;
    }
    if !config.trust_forwarded_for {
        // Behind Caddy every request would share Caddy's address (QA Security I1).
        tracing::warn!(
            "INGRESS_TRUST_FORWARDED_FOR=false: per-IP ingress limits use the TCP peer; set it to true when the Core is reachable only through Caddy"
        );
    }
    let app = App::connect(&config).await?;
    tokio::spawn(app.dispatcher.run());
    let listener = tokio::net::TcpListener::bind(config.bind)
        .await
        .with_context(|| format!("bind {}", config.bind))?;
    tracing::info!(bind = %config.bind, "allwin-core listening");
    axum::serve(
        listener,
        http::router(app.state).into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("serve")
}

async fn migrate(owner: &SecretString) -> anyhow::Result<()> {
    let pool = app::connect(owner, 1)
        .await
        .context("MIGRATION_DATABASE_URL")?;
    MIGRATOR.run(&pool).await.context("migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}

fn owner_url() -> anyhow::Result<SecretString> {
    std::env::var("MIGRATION_DATABASE_URL")
        .ok()
        .filter(|v| !v.is_empty())
        .map(SecretString::new)
        .context("MIGRATION_DATABASE_URL is required")
}

async fn dev_owner_pool() -> anyhow::Result<sqlx::PgPool> {
    if std::env::var("ALLWIN_ENABLE_DEV_COMMANDS").as_deref() != Ok("true") {
        bail!("dev commands are disabled (set ALLWIN_ENABLE_DEV_COMMANDS=true in dev/test only)");
    }
    app::connect(&owner_url()?, 1)
        .await
        .context("MIGRATION_DATABASE_URL")
}

async fn shutdown_signal() {
    // If the signal handler cannot be installed the server keeps running until killed.
    if tokio::signal::ctrl_c().await.is_err() {
        std::future::pending::<()>().await;
    }
}
