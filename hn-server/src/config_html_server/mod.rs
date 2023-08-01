use std::path::PathBuf;
use std::{net::SocketAddr, sync::Arc};

use axum::routing::post;
use axum::{extract::State, response::Html, routing::get, Router};
use minijinja::context;
use tower_http::services::{ServeDir, ServeFile};

use crate::config::{Configurable, Settings};
use crate::http::{OrInternalError, Response};
use crate::{config, prelude::*};

pub(crate) mod app;
pub(crate) mod discord;
mod edit;
mod templates;

async fn hello_world(
    templates: templates::Templates,
    config: State<Arc<config::Settings>>,
) -> Response {
    // let config_server_bind_address = "home(config_server_bind_address) value"; // &initial_app.config_server_bind_address;
    let mut confs = Vec::<minijinja::value::Value>::new();
    for entry in config.entries() {
        let section_name = entry.configurable.section_name();
        let view_html = render_view_html(&templates, &entry, None).await?.0;

        confs.push(context! {
            section_name,
            view_html,
        });
    }

    templates
        .render("home.html.j2", context!(confs))
        .err_500()
        .map(Html::from)
}

async fn render_view_html(
    templates: &templates::Templates,
    entry: &config::SettingEntry<'_, '_>,
    ok_updated: Option<(JSON, bool)>,
) -> Response {
    let section_name = entry.configurable.section_name();
    let mut view_json = entry.get_view_json().await.err_500()?;

    if let Some((ok_data, updated)) = ok_updated {
        let obj = view_json.as_object_mut().expect("object");
        obj.insert("ok".to_string(), ok_data);
        obj.insert("updated".to_string(), serde_json::Value::Bool(updated));
    }

    let configurable_html = templates
        .render_block(&entry.configurable.template(), "view", view_json)
        .with_context(|| format!("rendering view block for section {section_name:?} data"))
        .err_500()?;

    Ok(Html(
        templates
            .render(
                "setting-section-view.html.j2",
                context! {
                    section_name,
                    configurable_html,
                },
            )
            .err_500()?,
    ))
}

async fn render_edit_html(
    templates: &templates::Templates,
    entry: &config::SettingEntry<'_, '_>,
    err_data: Option<JSON>,
) -> Response {
    let section_name = entry.configurable.section_name();
    let mut view_json = entry.get_view_json().await.err_500()?;
    if let Some(err) = err_data {
        view_json
            .as_object_mut()
            .expect("view json must be an object")
            .insert("err".to_string(), err);
    }

    let configurable_html = templates
        .render_block(&entry.configurable.template(), "edit", view_json)
        .with_context(|| format!("rendering view block for section {section_name:?} data"))
        .err_500()?;

    Ok(Html(
        templates
            .render(
                "setting-section-edit.html.j2",
                context! {
                    section_name,
                    configurable_html,
                },
            )
            .err_500()?,
    ))
}

fn setup(router: Router<Arc<Settings>>, c: Arc<Box<dyn Configurable>>) -> Router<Arc<Settings>> {
    let section_name = c.section_name();
    let edit_path = &format!("/;edit-{section_name}");
    let view_path = &format!("/;view-{section_name}");

    router
        // GET edit form
        .route(
            edit_path,
            get({
                let section_name = section_name.clone();
                move |templates: templates::Templates, config: State<Arc<config::Settings>>| async move {
                    let entry = config.get_entry(&section_name)
                        .with_context(|| format!("section {section_name:?} not found"))
                        .err_400()?;
                    let resp: Response = render_edit_html(&templates, &entry, None).await;
                    resp
                }
            }),
        )
        // POST save values
        .route(
            edit_path,
            post({
                let section_name = section_name.clone();
                // TODO: make this configurable based on C
                move |templates: templates::Templates, config: State<Arc<config::Settings>>, params: axum::Form<serde_json::Value>| async move {
                    // let value = params.0; // .with_context(|| format!("parsing update for {section_name} update")).err_400()?;
                    // tracing::debug!(?value, "submitting conf for update");
                    let entry = config.get_entry(&section_name)
                        .with_context(|| format!("section {section_name:?} not found"))
                        .err_400()?;
                    let fn_result = entry.save_with(&params.0).await.with_context(|| format!("proper error handling not supported for configurables")).err_500()?;

                    let resp: Response = match fn_result {
                        Ok(ok_updated) => render_view_html(&templates, &entry, Some(ok_updated)).await,
                        Err(err) => render_edit_html(&templates, &entry, Some(err)).await,
                    };

                    resp
                }
            }),
        )
        // GET updated values
        .route(
            view_path,
            get({
                let section_name = section_name.clone();
                // TODO: make this configurable based on C
                move |templates: templates::Templates, config: State<Arc<config::Settings>>, _params: axum::Form<serde_json::Value>| async move {
                    let entry = config.get_entry(&section_name)
                        .with_context(|| format!("section {section_name:?} not found"))
                        .err_400()?;
                    let resp: Response = render_view_html(&templates, &entry, None).await;
                    resp
                }
            }),
        )
}

pub(crate) async fn start(config: Arc<config::Settings>) -> Result<()> {
    // TODO: Ask to set-up new configurations if not present?
    let initial_app = &config
        .get_entry("here-now-app")
        .expect("required app settings")
        .get_toml_and_parse::<app::AppConfiguration>()
        .await
        .expect("found app settings")
        .unwrap_or_default();

    let config_server_bind_address = initial_app
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
    let mut app = Router::new().route("/", get(hello_world));
    let templates = templates::Templates::new(templates_dir, initial_app.dev_mode.unwrap_or(true));

    #[allow(deprecated)]
    for conf in config.configurables().cloned() {
        app = setup(app, conf);
    }

    let app = app
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
