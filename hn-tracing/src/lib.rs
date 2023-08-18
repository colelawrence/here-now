use opentelemetry::global;

use tracing_subscriber::prelude::*;

static DEFAULT_HERE_NOW_LOG_ENV: &'static str = "debug,hyper=warn,pot=warn,nebari=warn";

pub fn expect_init_logger(service_name: &str) {
    // Something like http://localhost:14268/api/traces
    let jaeger_collector_endpoint_var = std::env::var("JAEGER_COLLECTOR_ENDPOINT");
    expect_init_logger_jaeger(service_name, jaeger_collector_endpoint_var.ok());
}

pub fn expect_init_logger_jaeger(
    service_name: &str,
    jaeger_collector_endpoint_var: Option<String>,
) {
    let env_filter = tracing_subscriber::EnvFilter::try_from_env("HERE_NOW_LOG")
        .unwrap_or_else(|_| DEFAULT_HERE_NOW_LOG_ENV.into());

    if let Some(jaeger_collector_endpoint) = jaeger_collector_endpoint_var {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let handle = tokio::runtime::Handle::try_current().unwrap_or_else(|err| {
            tracing::info!(reason=?err, "creating a tokio runtime for open telemetry jaeger reqwest client");
            let tokio = tokio::runtime::Runtime::new().unwrap();
            let handle = tokio.handle().clone();
            std::mem::forget(tokio);
            handle
        });
        let _entered = handle.enter();

        let tracer = opentelemetry_jaeger::new_collector_pipeline()
            .with_service_name(service_name)
            .with_endpoint(jaeger_collector_endpoint.clone())
            .with_instrumentation_library_tags(false)
            .with_http_client(reqwest::Client::new())
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("connected to jaeger");

        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(
                tracing_subscriber::fmt::layer()
                    .with_filter(tracing_subscriber::EnvFilter::from("warn")),
            )
            .init();

        // Check out customization https://www.jaegertracing.io/docs/1.47/frontend-ui/
        tracing::warn!(
            "jaeger tracing enabled, sending to {jaeger_collector_endpoint}, the UI is usually viewable at http://localhost:16686/dev/traces/search?service={service_name}",
        );
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

pub fn test_logger() {
    // in case a test needs logging
    let env_filter = tracing_subscriber::EnvFilter::try_from_env("HERE_NOW_TEST_LOG")
        .or_else(|_| tracing_subscriber::EnvFilter::try_from_env("HERE_NOW_LOG"))
        .unwrap_or_else(|_| DEFAULT_HERE_NOW_LOG_ENV.into());
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}
