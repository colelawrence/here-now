use axum::extract::FromRequestParts;
use axum::Extension;
use http::request::Parts;
use std::path::PathBuf;
use std::sync::Arc;

use crate::prelude::*;

use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;

#[derive(Clone)]
pub(crate) struct Templates {
    debug: Cow<'static, str>,
    reloader: Arc<AutoReloader>,
}

impl Templates {
    pub(crate) fn render(&self, template_name: &str, value: impl Serialize) -> Result<String> {
        let guard = self
            .reloader
            .acquire_env()
            .with_context(|| "acquiring guard for templates")?;
        let tmpl = guard
            .get_template(template_name)
            .with_context(|| format!("finding template {template_name:?} ({})", self.debug))?;
        let result = tmpl.render(value).with_context(|| {
            format!(
                "rendering template {template_name:?} with values ({})",
                self.debug
            )
        })?;
        Ok(result)
    }

    pub(crate) fn render_block(
        &self,
        template: &HTMXPartial,
        block_name: &str,
        value: impl Serialize,
    ) -> Result<String> {
        let template_name = template.template_file;
        let guard = self
            .reloader
            .acquire_env()
            .with_context(|| "acquiring guard for templates")?;
        let tmpl = guard
            .get_template(template_name)
            .with_context(|| format!("finding template {template_name:?} ({})", self.debug))?;
        let result = tmpl
            .eval_to_state(value)
            .with_context(|| {
                format!(
                    "evaluating template {template_name:?} with values ({})",
                    self.debug
                )
            })?
            .render_block(block_name)
            .with_context(|| {
                format!(
                    "rendering template {template_name:?} block {block_name:?} with values ({})",
                    self.debug
                )
            })?;
        Ok(result)
    }

    pub(crate) fn new(template_path: impl Into<PathBuf>, is_dev: bool) -> Templates {
        let template_path = template_path.into();
        let fast_reload = is_dev;
        let watch = is_dev;
        let mut debug = format!("path: {template_path:?}");
        // The closure is invoked every time the environment is outdated to
        // recreate it.
        let reloader = AutoReloader::new(move |notifier| {
            let mut env = Environment::new();
            env.set_loader(path_loader(&template_path));

            if fast_reload {
                notifier.set_fast_reload(true);
            }

            // if watch_path is never called, no fs watcher is created
            if watch {
                notifier.watch_path(&template_path, true);
            }
            Ok(env)
        });

        if fast_reload {
            debug.push_str(", fast_reload");
        }
        if watch {
            debug.push_str(", fs watched");
        }

        Templates {
            debug: debug.into(),
            reloader: Arc::new(reloader),
        }
    }

    pub(crate) fn axum_layer(self) -> Extension<Self> {
        Extension(self)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Templates
where
    // these bounds are required by `async_trait`
    S: Send + Sync,
{
    type Rejection = http::StatusCode;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(req
            .extensions
            .get::<Templates>()
            .ok_or_else(|| http::StatusCode::INTERNAL_SERVER_ERROR)?
            .clone())
    }
}
