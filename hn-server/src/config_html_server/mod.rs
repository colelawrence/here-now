use std::path::PathBuf;
use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, response::Html, routing::get, Router};
use http::StatusCode;
use minijinja::context;
use tower_http::services::{ServeDir, ServeFile};

use crate::{config, prelude::*};

impl templates::Templates {
    fn home(&self, config: &config::ConfigContent) -> Result<String> {
        let dev_server_bind_address = &config.config_server_bind_address;
        self.render("home.html.j2", context!(dev_server_bind_address))
    }
}

async fn hello_world(
    templates: templates::Templates,
    config: State<Arc<config::ConfigContent>>,
) -> Result<Html<String>, (StatusCode, String)> {
    Ok(Html::from(templates.home(&config).err_500()?))
}
trait OrInternalError<T> {
    fn err_500(self) -> Result<T, (StatusCode, String)>;
}

impl<T> OrInternalError<T> for Result<T> {
    fn err_500(self) -> Result<T, (StatusCode, String)> {
        self.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

// maud example
// use maud::{html, Markup};
// async fn hello_world2(
//     templates: templates::Templates,
//     config: State<config::ConfigContent>,
// ) -> Markup {
//     // templates.home(&config);
//     html! {
//         head {
//             style { r#"html, body { font-family: system-ui, sans-serif; } body { margin: 2rem auto; max-width: 500px; }"# }
//         }
//         body {
//             h1 { "Welcome to your new installation of the Here Now server" }
//             p { "You're now looking at the self-configuration page, where we'll set up your service."}
//         }
//     }
// }

mod templates;

pub async fn start(config: crate::config::ConfigFile) -> Result<()> {
    let config_server_bind_address = config
        .content
        .config_server_bind_address
        .clone()
        .unwrap_or_else(|| "0.0.0.0:3001".to_string());

    let addr: SocketAddr = config_server_bind_address.parse().with_context(|| {
        format!("parsing config_server_bind_address {config_server_bind_address:?}")
    })?;

    let templates_dir = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/config_html_server");

        path.canonicalize()
            .with_context(|| format!("expect to find config_html_server directory at {path:?}"))?
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_world))
        .fallback_service(
            ServeDir::new(templates_dir.join("./build"))
                .not_found_service(ServeFile::new(templates_dir.join("./not-found.html"))),
        )
        .layer(templates::Templates::provide(
            templates_dir,
            config.content.dev_mode.unwrap_or(true),
        ))
        .with_state(config.content.clone());

    println!("Config server starting on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
