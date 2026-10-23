//! Tracing setup. Request logs carry route templates only, never raw URIs
//! (s1-webhook-ingest §4.1 R1/R2); the panic hook prints no request data.

use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::MakeWriter;

/// JSON lines on stdout, filtered by `RUST_LOG` (default `info`).
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    init_with(filter, std::io::stdout);
}

/// Same format with an explicit filter and writer (tests capture output with it).
/// A second initialization is ignored.
pub fn init_with<W>(filter: EnvFilter, writer: W)
where
    W: for<'w> MakeWriter<'w> + Send + Sync + 'static,
{
    let _ = tracing_subscriber::fmt()
        .json()
        .with_env_filter(filter)
        .with_writer(writer)
        .with_current_span(true)
        .with_span_list(false)
        .try_init();
    install_panic_hook();
}

/// Replaces the default hook, which prints the panic payload; payloads may be
/// built from request data. Only the source location is logged.
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "<unknown>".to_owned());
        tracing::error!(%location, "panic");
    }));
}
