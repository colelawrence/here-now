use std::sync::Arc;

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
    pub(crate) use std::borrow::Cow;
    use std::fmt::format;
    pub(crate) use std::fmt::Debug;
    pub(crate) use std::sync::Arc;

    pub(crate) use anyhow::{Context, Error, Result};
    pub(crate) use serde::{Deserialize, Serialize};

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

    // /// Dev version with auto reloading from disk
    // /// Future: use macro to replace with static versions
    // pub(crate) struct HTMXRenderer {
    //     pub(crate) reloader: Arc<minijinja_autoreload::AutoReloader>,
    // }

    // impl HTMXPartial {
    //     pub fn render_block<T: Serialize>(
    //         &self,
    //         renderer: &HTMXRenderer,
    //         block_name: &'static str,
    //         value: T,
    //     ) -> Result<String> {
    //         let env = renderer
    //             .reloader
    //             .acquire_env()
    //             .with_context(|| format!("acquiring reloader guard for HTMXPartial",))?;
    //         let tmpl = env.get_template(self.template_file).with_context(|| {
    //             format!(
    //                 "getting template file {:?} for HTMXPartial",
    //                 self.template_file
    //             )
    //         })?;
    //         tmpl.eval_to_state(value)
    //             .with_context(|| {
    //                 format!(
    //                     "evaluating state with value for template {:?}",
    //                     self.template_file
    //                 )
    //             })?
    //             .render_block(block_name)
    //             .with_context(|| format!("rendering block {block_name:?}"))
    //     }
    // }
}

mod config;
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

    tracing::info!("loaded {config_file:#?}");

    tokio::select! {
        res = config_html_server::start(Arc::new(config_file)) => {
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
