use axum::{response::Html, routing::get, Extension, Router};
use derive_codegen::Codegen;

use tower_http::trace::TraceLayer;

use crate::{http::OrInternalError, prelude::*};

mod svelte_templates;

pub fn start_server_from_tcp_listener(
    listener: std::net::TcpListener,
    addr: &std::net::SocketAddr,
    app_ctx: AppCtx,
) -> axum_server::Handle {
    info!(?addr, "starting public server");
    let handle = axum_server::Handle::new();
    let server = axum_server::from_tcp(listener).handle(handle.clone());

    let app = Router::new()
        .route("/", get(login_page))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(svelte_templates::SvelteTemplates {
            dev_path: Arc::new(
                get_crate_path()
                    .join("templates")
                    .canonicalize()
                    .expect("templates path exists"),
            ),
        }));

    app_ctx.spawn(async {
        server
            .serve(app.into_make_service())
            .await
            .context("serving public app")
    });

    handle
}

#[derive(Serialize, Codegen)]
#[codegen(tags = "login-page")]
#[allow(non_snake_case)]
pub struct LoginProps {
    loginURLs: Vec<LoginURL>,
}

/// What kind of login URL?
#[derive(Serialize, Codegen)]
#[codegen(tags = "login-page")]
#[codegen(svelte_enum)]
pub struct LoginURL {
    label: String,
    url: String,
}

#[test]
fn generate_svelte_templates() {
    derive_codegen::Generation::for_tag("login-page")
        .as_arg_of(
            std::process::Command::new("deno")
                .args("run ./generator/generate-typescript.ts".split(' '))
                .args("--sharedFileName=login-page.ts".split(' '))
                .current_dir(get_crate_path().join("templates")),
        )
        .write()
        .print();
}

async fn login_page(
    Extension(templates): Extension<svelte_templates::SvelteTemplates>,
) -> HttpResult {
    let props = LoginProps {
        loginURLs: vec![
            LoginURL {
                label: "Discord".to_string(),
                url: "login-discord".to_string(),
            },
            LoginURL {
                label: "Slack".to_string(),
                url: "login-slack".to_string(),
            },
            LoginURL {
                label: "Google Workspace".to_string(),
                url: "login-google-workspace".to_string(),
            },
        ],
    };
    let template = svelte_template!("login.template.compiled.cjs");
    templates
        .render_svelte_into_html_page(&template, props)
        .context("rendering login page")
        .err_500()
        .map(Html)
}
