use serde_json::json;
use toml_edit::{Formatted, Item, Value};

use crate::prelude::*;

use super::Configurable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordConfiguration {
    client_id: String,
    api_key: String,
}

#[derive(Debug, Clone)]
pub struct DiscordSettings;

impl Configurable for DiscordSettings {
    fn template(&self) -> HTMXPartial {
        htmx_partial!("discord.configurable.html.j2")
    }

    fn section_name(&self) -> Cow<'static, str> {
        "discord".into()
    }

    fn vars(&self, section: &Box<dyn toml_edit::TableLike>) -> Result<JSON> {
        let api_key = section
            .get("api_key")
            .and_then(|i| i.as_str())
            .unwrap_or_default();
        let client_id = section
            .get("client_id")
            .and_then(|i| i.as_str())
            .unwrap_or_default();

        Ok(json!({
            "api_key": api_key,
            "client_id": client_id,
        }))
    }

    fn save(&self, data: &JSON, toml: &mut dyn toml_edit::TableLike) -> Result<JSON, JSON> {
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

        Ok(json!({}))
    }
}
