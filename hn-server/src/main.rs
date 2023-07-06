use actionable::{Permissions, Statement};
use anyhow::Context;
use bonsaidb::{
    core::permissions::bonsai::{BonsaiAction, ServerAction},
    server::{CustomServer, ServerConfiguration},
};
use tokio;

// mod schema;
// mod webserver;

#[derive(Debug)]
enum Game {}

#[allow(unused)]
mod prelude {

    use std::fmt::Debug;

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
        pub private_bind_address: Option<String>,
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

mod private_server {
    use std::net::SocketAddr;

    use crate::prelude::*;
    use axum::{routing::get, Router};
    use maud::{html, Markup};

    async fn hello_world() -> Markup {
        html! {
            head {
                style { r#"html, body { font-family: system-ui, sans-serif; } body { margin: 2rem auto; max-width: 500px; }"# }
            }
            body {
                h1 { "Welcome to your new installation of the Here Now server" }
                p { "You're now looking at the self-configuration page, where we'll set up your service."}
            }
        }
    }

    pub async fn start(config: crate::config::ConfigFile) -> Result<()> {
        // build our application with a single route
        let app = Router::new().route("/", get(hello_world));

        let private_bind_address = config
            .content
            .private_bind_address
            .clone()
            .unwrap_or_else(|| "0.0.0.0:3001".to_string());

        let addr: SocketAddr = private_bind_address
            .parse()
            .with_context(|| format!("parsing private_bind_address {private_bind_address:?}"))?;

        println!("Private server starting on http://{addr}");
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() {
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
        _ = private_server::start(config_file.clone()) => {
            println!("Exited private server");
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
