use std::process::Command;
use std::sync::Arc;

use axum::extract::Query;
use axum::{extract::State, response::Html, routing::get, Router};
use axum_server::service::SendService;
use http::header::LOCATION;
use http::{HeaderValue, StatusCode};
use tower_http::services::ServeDir;

use crate::config::Settings;
use crate::http::OrInternalError;
use crate::{config, prelude::*};

mod dev_jaeger_proxy {
    use axum::Router;
    use reverse_proxy_service::AppendPrefix;
    use tower_http::trace::TraceLayer;

    use crate::prelude::{f, ResultExt};

    pub(super) fn make_router() -> Router {
        // get from config?
        let jaeger_host = "127.0.0.1:16686";
        let jaeger_ui_host = reverse_proxy_service::builder_http(jaeger_host)
            .todo(f!("ensure jaeger host ({jaeger_host}) is valid"));

        Router::new()
        .route_service("/*path", jaeger_ui_host.build(AppendPrefix("/dev/traces")))
        .layer(TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
            tracing::trace_span!("proxy-request", method = %request.method(), uri = %request.uri())
        }))
    }
}

pub(super) fn create_dev_router() -> Router<Arc<Settings>> {
    let router = Router::<Arc<Settings>>::new();
    let current_platform = current_platform::CURRENT_PLATFORM;
    // TODO: make this configurable
    let doc_path = get_crate_path().join(format!("../target/{current_platform}/doc"));

    router
        .nest_service("/docs", ServeDir::new(doc_path))
        // Equivalent of https://github.com/yyx990803/launch-editor
        // /__open-in-editor?file=src/main.js:13:24
        .route("/open", get(dev_open))
        .route("/redir", get(dev_redir))
        // TODO: make this based on the config
        .nest_service("/traces", dev_jaeger_proxy::make_router().into_service())
}

#[derive(Debug, Deserialize)]
struct DevRedirectParams {
    uri: String,
}

/// `/dev/redir?uri=#{uri}`
// #[instrument(skip_all)]
async fn dev_redir(
    Query(DevRedirectParams { uri }): Query<DevRedirectParams>,
    state: State<Arc<config::Settings>>,
) -> HttpResult<axum::response::Response<String>> {
    let public_url = state
        .0
        .get_entry("here-now-app")
        .context("get app settings")
        .err_500()?
        .get_toml_or_empty()
        .await
        .context("get toml")
        .err_500()?
        .get("public_host_base_url")
        .context("has key")
        .err_500()?
        .as_str()
        .context("is string")
        .err_500()?
        .to_string();

    let mut resp = axum::response::Response::new(String::from("Redirecting..."));

    *resp.status_mut() = StatusCode::TEMPORARY_REDIRECT;

    let headers = resp.headers_mut();
    headers.append(
        LOCATION,
        HeaderValue::from_str(&format!("{public_url}{uri}")).unwrap(),
    );

    Ok(resp)
}

#[derive(Debug, Deserialize)]
struct DevOpenEditorParams {
    file: String,
}

/// The middleware factory function accepts the following arguments (all optional, the callback can be in any position as long as it's the last argument):
/// A specific editor bin to try first. Defaults to inferring from running processes, then fallback to env variables like EDITOR and VISUAL.
/// The root directory of source files, in case the file path is relative. Defaults to process.cwd().
/// a callback when it fails to launch the editor.
/// To launch files, send requests to the server like the following:
#[instrument]
async fn dev_open(Query(DevOpenEditorParams { file }): Query<DevOpenEditorParams>) -> HttpResult {
    let crate_path = get_crate_path();
    let root_dir = crate_path.parent().unwrap();
    let file_path = root_dir.join(if file.starts_with('"') {
        file.trim_matches('"')
    } else {
        file.as_str()
    });

    let output = info_span!("Opening file with launch-in-editor.cjs", ?file_path)
        .in_scope(|| {
            Command::new("node")
                .current_dir(&crate_path)
                .arg("./dev/open/launch-in-editor.cjs")
                .arg(file_path)
                .output()
                .context("launch editor")
        })
        .err_500()?;

    let stdout = output
        .status
        .success()
        .then(|| &output.stdout)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "launch editor failed: {err}\n{out}",
                err = String::from_utf8_lossy(&output.stderr),
                out = String::from_utf8_lossy(&output.stdout)
            )
        })
        .err_500()?;

    // html that immediately closes itself
    Ok(Html(format!(
        "<!DOCTYPE html>
    <html>
    <pre>{}</pre>
    <script>window.close()</script>
    </html>",
        String::from_utf8_lossy(&stdout).to_string()
    )))
}
