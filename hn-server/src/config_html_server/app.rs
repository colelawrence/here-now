use serde_json::json;
use toml_edit::{Decor, Formatted, Item, Value};

use super::{edit, Configurable};
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
        "here-now-app".into()
    }

    fn vars(&self, toml: &Box<dyn toml_edit::TableLike>) -> Result<JSON> {
        let public_bind_address = toml
            .get("public_bind_address")
            .map(|item| item.as_str())
            .unwrap_or_default();
        let public_host_base_url = toml
            .get("public_host_base_url")
            .map(|item| item.as_str())
            .unwrap_or_default();
        let config_server_bind_address = toml
            .get("config_server_bind_address")
            .map(|item| item.as_str())
            .unwrap_or_default();
        let dev_mode = toml
            .get("dev_mode")
            .map(|item| item.as_bool())
            .unwrap_or_default();
        // How to get the comments surrounding the code:
        // toml.key_decor("dev_mode")
        // dbg!(&dev_mode);

        Ok(json!({
            "public_bind_address": public_bind_address,
            "public_host_base_url": public_host_base_url,
            "config_server_bind_address": config_server_bind_address,
            "dev_mode": dev_mode,
        }))
    }

    fn save(
        &self,
        json: &serde_json::Value,
        toml: &mut dyn toml_edit::TableLike,
    ) -> Result<JSON, JSON> {
        let public_bind_address = json
            .get("public_bind_address")
            .and_then(|a| a.as_str())
            .unwrap_or_default();
        let public_host_base_url = json
            .get("public_host_base_url")
            .and_then(|a| a.as_str())
            .unwrap_or_default();
        let config_server_bind_address = json
            .get("config_server_bind_address")
            .and_then(|a| a.as_str())
            .unwrap_or_default();
        let dev_mode = json
            .get("dev_mode")
            .and_then(|a| a.as_str())
            .map(|str| str == "on")
            .unwrap_or_default();

        edit::update_toml_key(
            toml,
            "public_bind_address",
            Item::Value(Value::String(Formatted::new(public_bind_address.into()))),
            Some("Where the public interface binds like `127.0.0.1`.".to_string()),
            false,
        );
        edit::update_toml_key(
            toml,
            "public_host_base_url",
            Item::Value(Value::String(Formatted::new(public_host_base_url.into()))),
            Some("Where the public interface will be accessed on the internet.".to_string()),
            false,
        );
        edit::update_toml_key(
            toml,
            "config_server_bind_address",
            Item::Value(Value::String(Formatted::new(
                config_server_bind_address.into(),
            ))),
            Some("Where this interface binds.".to_string()),
            false,
        );
        edit::update_toml_key(
            toml,
            "dev_mode",
            Item::Value(Value::Boolean(Formatted::new(dev_mode.into()))),
            Some("Whether or not to enable dev features of the server.".to_string()),
            false,
        );

        Ok(json!({ "saved": true }))
    }
}
