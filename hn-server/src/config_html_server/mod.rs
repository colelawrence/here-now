use std::path::PathBuf;
use std::{net::SocketAddr, sync::Arc};

use axum::routing::post;
use axum::{extract::State, response::Html, routing::get, Router};
use futures::Future;
use http::StatusCode;
use minijinja::context;
use tower_http::services::{ServeDir, ServeFile};

use crate::config::{AppConfiguration, ConfigContent, ConfigFile, Configurable};
use crate::config_html_server::discord::DiscordConfiguration;
use crate::{config, prelude::*};

use self::templates::Templates;

trait OrInternalError<T> {
    fn err_500(self) -> Result<T, (StatusCode, String)>;
    fn err_400(self) -> Result<T, (StatusCode, String)>;
}

impl<T> OrInternalError<T> for Result<T> {
    fn err_500(self) -> Result<T, (StatusCode, String)> {
        self.map_err(|err| {
            // redundant?
            tracing::error!(err=?err, "internal service error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("INTERNAL ERROR:\n{:#?}", err),
            )
        })
    }
    fn err_400(self) -> Result<T, (StatusCode, String)> {
        self.map_err(|err| (StatusCode::BAD_REQUEST, format!("BAD REQUEST:\n{}", err)))
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

pub(crate) mod discord;
mod templates;

type Response = Result<Html<String>, (StatusCode, String)>;

async fn hello_world(
    templates: templates::Templates,
    config: State<Arc<config::ConfigFile>>,
) -> Response {
    let config_server_bind_address = &config.initial_app.config_server_bind_address;

    templates
        .render("home.html.j2", context!(config_server_bind_address))
        .err_500()
        .map(Html::from)
}

#[derive(Deserialize)]
struct GetEditPartialParams {
    name: String,
}

fn setup<C: Configurable>(
    router: Router<Arc<ConfigFile>>,
    templates: &Templates,
) -> Router<Arc<ConfigFile>> {
    let section_name = C::section_name();
    let template = C::template();
    async fn get_handler(
        templates: templates::Templates,
        config: State<Arc<config::ConfigFile>>,
    ) -> Response {
        // template.render_block(templates, block_name, value);
        templates
            .render("home.html.j2", context!(value => "wwww"))
            .err_500()
            .map(Html::from)
        // Ok(Html::from("Hello!".to_string()))
    }

    router
        .route(
            &format!("/;conf-{section_name}"),
            get({
                let section_name = section_name.clone();
                move |templates: templates::Templates, config: State<Arc<config::ConfigFile>>| async move {
                    let section = config
                        .get_section_impulsively(&section_name)
                        .await
                        .err_500()?;
                    let context = C::view(&section)
                        .with_context(|| format!("creating view for section {section_name:?} data"))
                        .err_500()?;

                    let html = templates
                        .render_block(&template, "edit", context)
                        .with_context(|| {
                            format!("rendering view block for section {section_name:?} data")
                        })
                        .err_500()?;

                    let resp: Response = Ok(Html(
                        format!("<form action=\";conf-{section_name}\" method=\"post\" hx-swap=\"outerHTML\">{html}<button>Submit</button></form>")
                    ));
                    resp
                }
            }),
        )
        .route(
            &format!("/;conf-{section_name}"),
            post({
                let section_name = section_name.clone();
                // TODO: make this configurable based on C
                move |templates: templates::Templates, config: State<Arc<config::ConfigFile>>, params: axum::Form<serde_json::Value>| async move {
                    let value = serde_json::from_value::<C::Saving>(params.0).with_context(|| format!("parsing update for {section_name} update")).err_400()?;
                    tracing::debug!(?value, "submitting conf for update");
                    let section = config
                        .get_section_impulsively(&section_name)
                        .await
                        .err_500()?;
                    let context = C::view(&section)
                        .with_context(|| format!("creating view for section {section_name:?} data"))
                        .err_500()?;

                    let html = templates
                        .render_block(&template, "view", context)
                        .with_context(|| {
                            format!("rendering view block for section {section_name:?} data")
                        })
                        .err_500()?;

                    let resp: Response = Ok(Html(html));
                    resp
                }
            }),
        )
}

// async fn get_edit_partial(
//     templates: templates::Templates,
//     config: State<Arc<config::AppConfiguration>>,
//     params: axum::Form<GetEditPartialParams>,
// ) -> Response {

//     // let value = config
//     //     .get_value_str(&params.name)
//     //     .err_400()?
//     //     .unwrap_or_default();

//     templates
//         .render(
//             "edit-toml.html.j2",
//             context!(
//                 name => params.name,
//                 value => value,
//             ),
//         )
//         .err_500()
//         .map(Html::from)
// }

// #[derive(Deserialize)]
// struct SaveEditPartialParams {
//     name: String,
//     value: String,
// }

// async fn save_edit_partial(
//     templates: templates::Templates,
//     config: State<Arc<config::AppConfiguration>>,
//     params: axum::Form<SaveEditPartialParams>,
// ) -> Response {
//     config
//         .set_value_str(&params.name, &params.value)
//         .await
//         .err_500()?;

//     templates
//         .render(
//             "edit-button-toml.html.j2",
//             context!(
//                 name => params.name,
//                 value => params.value,
//             ),
//         )
//         .err_500()
//         .map(Html::from)
// }

pub async fn start(config: Arc<crate::config::ConfigFile>) -> Result<()> {
    let config_server_bind_address = config
        .initial_app
        .config_server_bind_address
        .clone()
        .unwrap_or_else(|| "0.0.0.0:3001".to_string());

    let addr: SocketAddr = config_server_bind_address.parse().with_context(|| {
        format!("parsing config_server_bind_address {config_server_bind_address:?}")
    })?;

    let templates_dir = &{
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/config_html_server");

        path.canonicalize()
            .with_context(|| format!("expect to find config_html_server directory at {path:?}"))?
    };

    // build our application with a single route
    let app = Router::new().route("/", get(hello_world));
    let templates =
        templates::Templates::new(templates_dir, config.initial_app.dev_mode.unwrap_or(true));

    let app = setup::<DiscordConfiguration>(app, &templates)
        // .route("/;edit", get(get_edit_partial))
        // .route("/;save", post(save_edit_partial))
        .fallback_service(
            ServeDir::new(templates_dir.join("./build"))
                .not_found_service(ServeFile::new(templates_dir.join("./not-found.html"))),
        )
        .layer(templates.axum_layer())
        .with_state(config);

    tracing::info!("Config server starting on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
