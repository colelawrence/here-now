#![allow(unused)]
use anyhow::Error;
use std::fmt::{format, Display};

pub(crate) use super::ecs;
pub(crate) use async_trait::async_trait;
pub(crate) use hn_app::_result_::*;
pub(crate) use hn_app::_tracing_::*;
pub(crate) use hn_app::app_ctx::AppCtx;
pub(crate) use std::borrow::Cow;
pub(crate) use std::fmt::Debug;
pub(crate) use std::hash::Hash;
pub(crate) use std::sync::Arc;

pub mod bonsai_ {
    pub use bonsaidb::core::connection::Connection;
    pub use bonsaidb::core::document::{CollectionDocument, Document};
    pub use bonsaidb::core::schema;
    pub use bonsaidb::core::schema::{SerializedCollection, SerializedView};
    pub use bonsaidb::local;
}

pub(crate) use serde::{Deserialize, Serialize};

// http
pub type HttpResult<T = axum::response::Html<String>> =
    core::result::Result<T, (http::StatusCode, String)>;

#[cfg(test)]
pub(crate) use hn_app::logging::test_logger;

pub type JSON = serde_json::Value;
pub type Cowstr = Cow<'static, str>;

// Customizing the context behavior for Here Now app specific needs?
// pub(crate) trait HereNowErrorContextualizer {
//     fn with_hn_context(self, f: impl FnOnce() -> String) -> ;
// }
// impl <C: anyhow::Context> HereNowErrorContextualizer for C {
//     fn with_hn_context(self, f: impl FnOnce() -> String) -> Result<Ok>;
// }

macro_rules! svelte_template {
    ($name: expr) => {
        SvelteTemplate {
            template_file: $name,
        }
    };
}
pub(crate) use svelte_template;

/// Dev version with auto reloading from disk
/// Future: use macro to replace with static versions
#[derive(Copy, Clone)]
pub struct SvelteTemplate {
    pub(crate) template_file: &'static str,
}

impl SvelteTemplate {
    #[tracing::instrument(skip(self))]
    pub fn read_cjs(&self, file_path: &std::path::Path) -> Result<String> {
        std::fs::read_to_string(&file_path).with_context(|| format!("reading {file_path:?}"))
    }
}

macro_rules! htmx_partial {
    ($name: expr) => {
        HTMXPartial {
            template_file: $name,
        }
    };
}
pub(crate) use htmx_partial;

/// Dev version with auto reloading from disk
/// Future: use macro to replace with static versions
#[derive(Copy, Clone)]
pub struct HTMXPartial {
    pub(crate) template_file: &'static str,
}

#[cfg(test)]
pub(crate) mod test_ecs {
    use hn_app::_ecs_::*;
    /// Create an [App] with 0 plugins.
    #[allow(unused)]
    pub(crate) fn test_app0() -> App {
        let app = App::new();
        let mut builder = AppBuilder::new(&app);
        builder.finish();
        app
    }
    /// Create an [App] with 1 plugin.
    pub(crate) fn test_app1<T>(z: T) -> App
    where
        T: Plugin,
    {
        test_app(move |builder| {
            builder.add_plugin(z);
        })
    }
    /// Create an [App] with 2 plugins.
    pub(crate) fn test_app2<T, U>(y: T, z: U) -> App
    where
        T: Plugin,
        U: Plugin,
    {
        test_app(move |builder| {
            builder.add_plugin(y).add_plugin(z);
        })
    }
    /// Create an [App] with 3 plugins.
    #[allow(unused)]
    pub(crate) fn test_app3<T, U, V>(x: T, y: U, z: V) -> App
    where
        T: Plugin,
        U: Plugin,
        V: Plugin,
    {
        test_app(move |builder| {
            builder.add_plugin(x).add_plugin(y).add_plugin(z);
        })
    }
    /// Create an [App] with 4 plugins.
    #[allow(unused)]
    pub(crate) fn test_app4<T, U, V, W>(x: T, y: U, z: V, q: W) -> App
    where
        T: Plugin,
        U: Plugin,
        V: Plugin,
        W: Plugin,
    {
        test_app(move |builder| {
            builder
                .add_plugin(x)
                .add_plugin(y)
                .add_plugin(z)
                .add_plugin(q);
        })
    }
    /// Create an [App] with a provided function.
    fn test_app<F>(builder_fn: F) -> App
    where
        F: FnOnce(&mut AppBuilder),
    {
        let app = App::new();
        let mut builder = AppBuilder::new(&app);
        builder_fn(&mut builder);
        builder.finish();
        app
    }
}

pub fn get_crate_path() -> std::path::PathBuf {
    let dir_from_env = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        std::env::var("HERE_NOW_SERVER_SRC_PATH").expect(
            "CARGO_MANIFEST_DIR or HERE_NOW_SERVER_SRC_PATH env var pointing at hn-server folder",
        )
    });

    return std::path::PathBuf::from(dir_from_env)
        .canonicalize()
        .expect("find ");
}
