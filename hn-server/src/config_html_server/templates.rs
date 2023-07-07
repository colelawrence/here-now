use axum::extract::FromRequestParts;
use axum::Extension;
use http::request::Parts;
use std::path::PathBuf;
use std::sync::Arc;

use crate::prelude::*;

use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;

pub(crate) struct Templates {
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
            .with_context(|| format!("finding template {template_name:?}"))?;
        let result = tmpl
            .render(value)
            .with_context(|| format!("rendering template {template_name:?} with values"))?;
        Ok(result)
    }

    pub(crate) fn provide(
        template_path: impl Into<PathBuf>,
        is_dev: bool,
    ) -> Extension<Arc<AutoReloader>> {
        let template_path = template_path.into();
        // The closure is invoked every time the environment is outdated to
        // recreate it.
        let reloader = AutoReloader::new(move |notifier| {
            let mut env = Environment::new();
            env.set_loader(path_loader(&template_path));

            if is_dev {
                notifier.set_fast_reload(true);
            }

            // if watch_path is never called, no fs watcher is created
            if is_dev {
                notifier.watch_path(&template_path, true);
            }
            Ok(env)
        });

        Extension(Arc::new(reloader))
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
        Ok(Templates {
            reloader: req
                .extensions
                .get::<Arc<AutoReloader>>()
                .ok_or_else(|| http::StatusCode::INTERNAL_SERVER_ERROR)?
                .clone(),
        })
    }
}
