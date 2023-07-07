use anyhow::Context;

use tokio;
use tracing_subscriber::prelude::*;

// mod schema;
// mod webserver;

#[derive(Debug)]
enum Game {}

#[allow(unused)]
mod prelude {
    pub(crate) use async_trait::async_trait;
    pub(crate) use std::fmt::Debug;

    pub(crate) use anyhow::{Context, Error, Result};
    pub(crate) use serde::{Deserialize, Serialize};

    // Customizing the context behavior for Here Now app specific needs?
    // pub(crate) trait HereNowErrorContextualizer {
    //     fn with_hn_context(self, f: impl FnOnce() -> String) -> ;
    // }
    // impl <C: anyhow::Context> HereNowErrorContextualizer for C {
    //     fn with_hn_context(self, f: impl FnOnce() -> String) -> Result<Ok>;
    // }
}

mod config {
    use crate::prelude::*;
    use std::sync::Arc;

    #[derive(Deserialize, Debug)]
    pub struct ConfigContent {
        /// e.g. `"0.0.0.0:8000"`
        pub public_bind_address: Option<String>,
        // /// Where should we expect all connecting services to come from?
        // /// e.g. http://localhost:8001
        // pub public_origins: Vec<String>,
        /// e.g. `"0.0.0.0:8001"`
        pub config_server_bind_address: Option<String>,
        pub dev_mode: Option<bool>,
    }

    #[derive(Clone, Debug)]
    pub struct ConfigFile {
        pub file_path: std::path::PathBuf,
        pub content: Arc<ConfigContent>,
    }

    pub async fn load_config_from_path(path: impl Into<std::path::PathBuf>) -> Result<ConfigFile> {
        let path_buf: std::path::PathBuf = path.into();
        let path = &path_buf
            .canonicalize()
            .with_context(|| format!("finding configuration file"))?;
        let content = tokio::fs::read(path)
            .await
            .with_context(|| format!("opening config file at {path:?}"))?;
        let content = toml_edit::de::from_slice(&content)
            .with_context(|| format!("loading config from file at {path:?}"))?;
        return Ok(ConfigFile {
            file_path: path_buf,
            content: Arc::new(content),
        });
    }
}

mod config_html_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let process_dir = std::env::current_dir().expect("getting current directory (pwd)");
    let config_file_path = std::env::args()
        .skip(1)
        .next()
        .expect("reading first argument for configuration file location");
    let config_file = config::load_config_from_path(&config_file_path)
        .await
        .with_context(|| {
            format!(
                "loading config file at {config_file_path:?} relative to directory {process_dir:?}"
            )
        })
        .expect("loading configuration");

    println!("{config_file:#?}");

    tokio::select! {
        res = config_html_server::start(config_file.clone()) => {
            println!("Exited private server: {res:#?}");
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
