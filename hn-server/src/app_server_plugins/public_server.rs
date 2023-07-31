use std::path::PathBuf;

use axum::{response::Html, routing::get, Extension, Router};
use derive_codegen::Codegen;
use quick_js::JsValue;
use tower_http::trace::TraceLayer;

use crate::{http::OrInternalError, prelude::*, quickjs::serialize_to_js_value};

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
        .layer(Extension(SvelteTemplates {
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

async fn login_page(Extension(templates): Extension<SvelteTemplates>) -> HttpResult {
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

#[derive(Clone)]
struct SvelteTemplates {
    dev_path: Arc<PathBuf>,
}

#[derive(Default)]
struct SvelteSSR {
    html: String,
    head: String,
    css_map: Option<String>,
    css_code: String,
}

impl SvelteTemplates {
    fn render_svelte_into_html_page<S: Serialize + Send>(
        &self,
        template: &SvelteTemplate,
        props: S,
    ) -> Result<String> {
        let ssr = self
            .render_svelte_template(template, props)
            .context("for html page")?;
        let mut html = r#"<!DOCTYPE html>
    <html><head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8"/>
    <title>Here Now</title>"#
            .to_string();
        html.push_str(&ssr.head);
        html.push_str("<style>");
        html.push_str(&ssr.css_code);
        html.push_str("</style>");
        html.push_str("</head><body>");
        html.push_str(&ssr.html);
        html.push_str("</body></html>");
        Ok(html)
    }
    fn render_svelte_template<S: Serialize + Send>(
        &self,
        template: &SvelteTemplate,
        props: S,
    ) -> Result<SvelteSSR> {
        let ctx = quick_js::Context::new().context("created new context for templates")?;
        let path = self
            .dev_path
            .join(template.template_file)
            .canonicalize()
            .context("find template file")?;
        let code = template.read_cjs(&path)?;
        ctx.set_global("module", JsValue::Object(Default::default()))
            .context("setting global module")?;
        ctx.eval(&code).expect("success");
        ctx.set_global(
            "_input_",
            serialize_to_js_value(props).context("serializing props for template")?,
        )
        .context("setting global in context")?;
        match ctx
            .eval("module.exports.render(_input_)")
            .context("rendered template quick")?
        {
            JsValue::Object(obj) => {
                let mut ssr = SvelteSSR::default();
                for (key, value) in obj {
                    match (key.as_str(), value) {
                        ("head", JsValue::String(head)) => ssr.head = head,
                        ("html", JsValue::String(html)) => ssr.html = html,
                        ("css", JsValue::Object(css)) => {
                            for (key, value) in css {
                                match (key.as_str(), value) {
                                    ("map", JsValue::Null) => {}
                                    ("map", JsValue::String(css_map)) => {
                                        ssr.css_map = Some(css_map)
                                    }
                                    ("code", JsValue::String(css_code)) => ssr.css_code = css_code,
                                    other => {
                                        warn!(
                                            "found unexpected key value for css object {other:?}"
                                        );
                                    }
                                }
                            }
                        }
                        other => {
                            warn!("found unexpected key value {other:?} ");
                        }
                    }
                }
                return Ok(ssr);
            }
            other => {
                return Err(anyhow::anyhow!(
                    "expected an object returned, but received {other:?}"
                ));
            }
        }
    }
}
