//! Tracing setup. Request logs carry route templates only, never raw URIs
//! (s1-webhook-ingest §4.1 R1/R2); the panic hook prints no request data.

use tracing_subscriber::EnvFilter;

pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // JSON lines on stdout; a second init (tests) is ignored.
    let _ = tracing_subscriber::fmt()
        .json()
        .with_env_filter(filter)
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
