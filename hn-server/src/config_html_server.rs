use std::{net::SocketAddr, sync::Arc};

use axum::Extension;
use axum::routing::post;
use axum::{extract::State, response::Html, routing::get, Router};
use axum_server::service::SendService;

use minijinja::context;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

use crate::config::{Configurable, Settings};
use crate::http::OrInternalError;
use crate::{config, prelude::*};

pub mod app;
mod dev_paths;
mod data_browser;
pub mod discord;
mod edit;
mod templates;

#[instrument(skip_all)]
async fn get_root_path(
    templates: templates::Templates,
    config: State<Arc<config::Settings>>,
) -> HttpResult {
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
) -> HttpResult {
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
) -> HttpResult {
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

fn setup_configurable(
    router: Router<Arc<Settings>>,
    c: Arc<Box<dyn Configurable>>,
) -> Router<Arc<Settings>> {
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
                    let resp: HttpResult = render_edit_html(&templates, &entry, None).await;
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

                    let resp: HttpResult = match fn_result {
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
                    let resp: HttpResult = render_view_html(&templates, &entry, None).await;
                    resp
                }
            }),
        )
}

pub async fn start(config: Arc<config::Settings>, app_ctx: AppCtx) -> Result<()> {
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
        .unwrap_or_else(|| "0.0.0.0:3000".to_string());

    let addr: SocketAddr = config_server_bind_address.parse().with_context(|| {
        format!("parsing config_server_bind_address {config_server_bind_address:?}")
    })?;

    let templates_dir = &{
        let path = get_crate_path().join("src/config_html_server");

        path.canonicalize()
            .with_context(|| format!("expect to find config_html_server directory at {path:?}"))?
    };

    // build our application with a single route
    let mut app = Router::<Arc<Settings>>::new().route("/", get(get_root_path));
    let templates = templates::Templates::new(templates_dir, initial_app.dev_mode.unwrap_or(true));

    #[allow(deprecated)]
    for conf in config.configurables().cloned() {
        app = setup_configurable(app, conf);
    }

    let app = app
        .fallback_service(
            ServeDir::new(templates_dir.join("./build"))
                .not_found_service(ServeFile::new(templates_dir.join("./not-found.html"))),
        )
        .layer(Extension(app_ctx.clone()))
        .layer(TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
            info_span!("config-request", method = %request.method(), uri = %request.uri())
        }))
        .layer(templates.axum_layer())
        .with_state::<()>(config.clone());

    tracing::warn!("Config server starting on http://{addr}");

    let app = Router::<Arc<Settings>>::new()
        .nest("/dev", dev_paths::create_dev_router())
        .nest("/data", data_browser::create_data_browser_router(app_ctx))
        // divided up so we don't trace the requests to the dev server
        .fallback_service(app.into_service())
        .with_state(config.clone());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
