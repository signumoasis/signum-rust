use std::collections::HashMap;

use opentelemetry::sdk::{
    export::trace::stdout,
    trace::{self, Config, IdGenerator, Sampler},
};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::MakeWriter, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry,
};

use tonic::metadata::*;

/// Compose multiple 'layers' into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as the return type to avoid having to
/// spell out the actual type of the returned subscriber, which is quite
/// complex to do.
///
/// We need to explicitly call out that the returned subscriber is `Send`
/// and `Sync` to make it possible to pass it to the `init_subscriber`
/// function later on.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let bunyan_formatting_layer = BunyanFormattingLayer::new(name, sink);

    //TODO: Use opentelemetry_otlp to output to honeycomb instead of stdio

    let mut map = MetadataMap::with_capacity(3);

    map.insert(
        "x-honeycomb-team",
        "MsGtxyie2tTLQr2AIDVMyD".parse().unwrap(),
    );
    map.insert("x-honeycomb-dataset", "signum-node-rs".parse().unwrap());
    map.insert("x-host", "example.com".parse().unwrap());
    map.insert("x-number", "123".parse().unwrap());
    map.insert_bin(
        "trace-proto-bin",
        MetadataValue::from_bytes(b"[binary data]"),
    );

    let mut headers = HashMap::<String, String>::new();
    headers.insert(
        "x-honeycomb-team".to_string(),
        "MsGtxyie2tTLQr2AIDVMyD".to_string(),
    );
    headers.insert(
        "x-honeycomb-dataset".to_string(),
        "signum-node-rs".to_string(),
    );

    let http_exporter = opentelemetry_otlp::HttpExporterBuilder::default()
        .with_endpoint("https://api.honeycomb.io/v1/trace")
        .with_headers(headers);

    //let tracer = stdout::new_pipeline().install_simple();
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(http_exporter
            // opentelemetry_otlp::new_exporter()
            //     .tonic()
            //     .with_endpoint("https://api.honeycomb.io:443/v1/trace")
            //     .with_protocol(Protocol::Grpc),
        ).with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(IdGenerator::default())
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer)
        .with(opentelemetry)
}

/// Register a subscriber as a global default to process span data.
///
/// IMPORTANT: This should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set up logger.");
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up global subscriber.");
}
