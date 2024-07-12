use tokio::task::JoinHandle;
use tracing::Subscriber;
use tracing_subscriber::{fmt::MakeWriter, prelude::*, EnvFilter};

/// Sets up a tracing subscriber.
#[cfg(not(feature = "bunyan"))]
pub fn get_subscriber<Sink>(
    _name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    use tracing_subscriber::fmt::{self, format::FmtSpan};

    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // --This code uses tracing-subscriber--
    let fmt_layer = fmt::layer()
        .compact()
        .with_target(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::NONE)
        .with_writer(sink);

    let subscriber = tracing_subscriber::registry();
    #[cfg(feature = "tokio-console")]
    let subscriber = {
        // Only enable this if the feature is enabled.
        let tokio_console_fmt_layer =
            console_subscriber::spawn().with_filter(tracing_subscriber::filter::LevelFilter::TRACE);
        subscriber.with(tokio_console_fmt_layer)
    };

    subscriber.with(fmt_layer.with_filter(filter_layer))
}

/// Sets up a tracing subscriber.
#[cfg(feature = "bunyan")]
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_subscriber::Registry;

    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let bunyan_format = BunyanFormattingLayer::new(name, sink);

    let bunyan_layer = JsonStorageLayer
        .and_then(bunyan_format)
        .with_filter(filter_layer);

    let subscriber = Registry::default();
    #[cfg(feature = "tokio-console")]
    let subscriber = {
        // Only enable this if the feature is enabled.
        let tokio_console_fmt_layer =
            console_subscriber::spawn().with_filter(tracing_subscriber::filter::LevelFilter::TRACE);
        subscriber.with(tokio_console_fmt_layer)
    };

    subscriber.with(bunyan_layer)
}

/// Sets the global default subscriber. Should only be called once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    let _ = tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"));
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
