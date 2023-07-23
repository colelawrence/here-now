use serde_json::json;
use toml_edit::{Formatted, Item, Value};

use super::Configurable;
use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AppConfiguration {
    /// e.g. `"0.0.0.0:8000"`
    pub public_bind_address: Option<String>,
    // /// Where should we expect all connecting services to come from?
    // /// e.g. http://localhost:8001
    // pub public_origins: Vec<String>,
    /// e.g. `"0.0.0.0:8001"`
    pub config_server_bind_address: Option<String>,
    pub dev_mode: Option<bool>,
}

#[derive(Debug)]
pub struct AppSettings;

impl Configurable for AppSettings {
    fn template(&self) -> HTMXPartial {
        htmx_partial!("app.configurable.html.j2")
    }

    fn section_name(&self) -> Cow<'static, str> {
        "app".into()
    }

    fn vars(&self, section: &Box<dyn toml_edit::TableLike>) -> Result<JSON> {
        let client_id = section.get("client_id").and_then(|item| item.as_str());
        let api_key = section.get("api_key").and_then(|item| item.as_str());
        Ok(json!({
            "client_id": client_id,
            "api_key": api_key,
        }))
    }

    fn save(
        &self,
        data: &serde_json::Value,
        toml: &mut dyn toml_edit::TableLike,
    ) -> Result<JSON, JSON> {
        let client_id = data
            .get("client_id")
            .and_then(|a| a.as_str())
            .unwrap_or_default();
        let api_key = data
            .get("api_key")
            .and_then(|a| a.as_str())
            .unwrap_or_default();

        toml.insert(
            "client_id",
            Item::Value(Value::String(Formatted::new(client_id.into()))),
        );
        toml.insert(
            "api_key",
            Item::Value(Value::String(Formatted::new(api_key.into()))),
        );

        Ok(json!({ "saved": true }))
    }
}
