use std::path::PathBuf;

use quick_js::JsValue;

use crate::prelude::*;

#[derive(Clone)]
pub struct SvelteTemplates {
    pub dev_path: Arc<PathBuf>,
}

#[derive(Default)]
pub struct SvelteSSR {
    pub html: String,
    pub head: String,
    pub css_map: Option<String>,
    pub css_code: String,
}

impl SvelteTemplates {
    #[instrument(skip_all)]
    pub fn render_svelte_into_html_page<S: Serialize + Send>(
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
        <meta name="viewport" content="width=device-width,initial-scale=1"/>
        <link rel="icon" type="image/png" href="/public/favicon.png"/>"#
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

    #[instrument(skip_all)]
    pub fn render_svelte_template<S: Serialize + Send>(
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
        info_span!("eval svelte template code in quickjs")
            .in_scope(|| ctx.eval(&code).context("evaluating svelte template code"))?;

        // info_span!("set props (dumb) in quickjs").in_scope(|| {
        //     ctx.set_global(
        //         "_input0_",
        //         serialize_to_js_value(&props).context("serializing props for template")?,
        //     )
        //     .context("setting global in context")
        // })?;

        info_span!("set_global_serde")
            .in_scope(|| ctx.set_global_serde("_input_", &props))
            .context("serializing props for template")?;
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
