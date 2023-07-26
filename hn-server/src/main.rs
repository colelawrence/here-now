use std::sync::Arc;

use tokio;
use tracing_subscriber::{prelude::*, util::SubscriberInitExt};

use crate::config::Settings;

// mod schema;
mod app_ctx;
mod hmm;

#[derive(Debug)]
enum Game {}

#[allow(unused)]
mod prelude {
    pub(crate) use super::app_ctx::{AppCtx, AppCtxPlugin, AppSenderExt};
    pub(crate) use async_trait::async_trait;
    pub(crate) use shipyard_app::prelude::*;
    /// You might have meant to use [UniqueView]
    pub(crate) struct Unique;
    pub(crate) use std::borrow::Cow;
    use std::fmt::format;
    pub(crate) use std::fmt::Debug;
    pub(crate) use std::sync::Arc;
    pub(crate) use tracing::{
        debug, debug_span, error, error_span, info, info_span, warn, warn_span,
    };

    pub(crate) use anyhow::{Context, Error, Result};
    pub(crate) use serde::{Deserialize, Serialize};

    pub type JSON = serde_json::Value;
    pub type Cowstr = Cow<'static, str>;

    #[macro_use]
    pub(crate) use std::format_args as f;

    // Customizing the context behavior for Here Now app specific needs?
    // pub(crate) trait HereNowErrorContextualizer {
    //     fn with_hn_context(self, f: impl FnOnce() -> String) -> ;
    // }
    // impl <C: anyhow::Context> HereNowErrorContextualizer for C {
    //     fn with_hn_context(self, f: impl FnOnce() -> String) -> Result<Ok>;
    // }

    macro_rules! htmx_partial {
        ($name: expr) => {
            HTMXPartial {
                template_file: $name,
            }
        };
    }
    pub(crate) use htmx_partial;

    /// Dev version with auto reloading from disk
    /// Future: use macro to replace with static versions
    #[derive(Copy, Clone)]
    pub(crate) struct HTMXPartial {
        pub(crate) template_file: &'static str,
    }

    /// Create an [App] with 0 plugins.
    #[cfg(test)]
    #[allow(unused)]
    pub(crate) fn test_app0() -> App {
        let app = App::new();
        let mut builder = AppBuilder::new(&app);
        builder.finish();
        app
    }
    /// Create an [App] with 1 plugin.
    #[cfg(test)]
    pub(crate) fn test_app1<T>(z: T) -> App
    where
        T: Plugin,
    {
        test_app(move |builder| {
            builder.add_plugin(z);
        })
    }
    /// Create an [App] with 2 plugins.
    #[cfg(test)]
    pub(crate) fn test_app2<T, U>(y: T, z: U) -> App
    where
        T: Plugin,
        U: Plugin,
    {
        test_app(move |builder| {
            builder.add_plugin(y).add_plugin(z);
        })
    }
    /// Create an [App] with 3 plugins.
    #[cfg(test)]
    #[allow(unused)]
    pub(crate) fn test_app3<T, U, V>(x: T, y: U, z: V) -> App
    where
        T: Plugin,
        U: Plugin,
        V: Plugin,
    {
        test_app(move |builder| {
            builder.add_plugin(x).add_plugin(y).add_plugin(z);
        })
    }
    /// Create an [App] with 4 plugins.
    #[cfg(test)]
    #[allow(unused)]
    pub(crate) fn test_app4<T, U, V, W>(x: T, y: U, z: V, q: W) -> App
    where
        T: Plugin,
        U: Plugin,
        V: Plugin,
        W: Plugin,
    {
        test_app(move |builder| {
            builder
                .add_plugin(x)
                .add_plugin(y)
                .add_plugin(z)
                .add_plugin(q);
        })
    }
    /// Create an [App] with 3 plugins.
    #[cfg(test)]
    fn test_app<F>(builder_fn: F) -> App
    where
        F: FnOnce(&mut AppBuilder),
    {
        let app = App::new();
        let mut builder = AppBuilder::new(&app);
        builder_fn(&mut builder);
        builder.finish();
        app
    }

    #[cfg(test)]
    pub fn get_crate_path() -> std::path::PathBuf {
        return std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
    }

    pub(crate) trait ResultExt<T, E> {
        /// Use when you're not sure if we need to unwrap or ignore the error
        /// ```ignore
        /// // for example
        /// .todo(f!("configuring watcher (dur: {:?})", self.polling_duration));
        /// ```
        fn todo<'a>(self, f: std::fmt::Arguments<'a>) -> T;
    }

    impl<T, E> ResultExt<T, E> for Result<T, E>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        fn todo<'a>(self, f: std::fmt::Arguments<'a>) -> T {
            self.with_context(|| format!("{}", f)).unwrap()
        }
    }
}

mod config;
mod config_html_server;

fn expect_init_logger() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init()
}

pub(crate) fn test_logger() {
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}

#[tokio::main]
async fn main() {
    expect_init_logger();
    let sub = watchable::Watchable::new(());
    let hmm_handle = tokio::spawn(hmm::start(sub.watch()));

    // let process_dir = std::env::current_dir().expect("getting current directory (pwd)");
    // let config_file_path = std::env::args()
    //     .skip(1)
    //     .next()
    //     .expect("reading first argument for configuration file location");
    // let mut config_file = config::load_config_from_path(&config_file_path)
    //     .await
    //     .with_context(|| {
    //         format!(
    //             "loading config file at {config_file_path:?} relative to directory {process_dir:?}"
    //         )
    //     })
    //     .expect("loading configuration");

    let config_dir = crate::config::config_directory_setup::init_config_directory();
    let settings = Settings::new(config_dir)
        .with(config_html_server::app::AppSettings)
        .with(config_html_server::discord::DiscordSettings);

    tracing::info!("loaded {settings:#?}");

    tokio::select! {
        res = config_html_server::start(Arc::new(settings)) => {
            println!("Exited private server: {res:#?}");
            sub.shutdown();
        }
        res = hmm_handle => {
            println!("Hmm exited: {res:#?}");
        }
    }

    // let server = CustomServer::<Game>::open(
    //     ServerConfiguration::new("minority-game.bonsaidb")
    //         .server_name("minority-game.gooey.rs")
    //         .default_permissions(Permissions::from(
    //             Statement::for_any().allowing(&BonsaiAction::Server(ServerAction::Connect)),
    //         )),
    // )
    // .await?;
}
