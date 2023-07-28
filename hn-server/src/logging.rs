use tracing_subscriber::prelude::*;

static DEFAULT_RUST_LOG_ENV: &'static str = "server=debug,tower_http=debug";

// sorry, I don't know how to make this a simple function
// without an insane return type
macro_rules! logger {
    () => {{
        use tracing_subscriber as ts;
        let env_filter = ts::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| DEFAULT_RUST_LOG_ENV.into());
        ts::registry().with(env_filter).with(ts::fmt::layer())
    }};
}

pub(super) fn expect_init_logger() {
    logger!().init()
}

#[cfg(test)]
pub(crate) fn test_logger() {
    // in case a test needs logging
    let _ = logger!().try_init();
}
