use serde_json::json;
use toml_edit::{Formatted, Item, Value};

use crate::prelude::*;

use super::{edit, Configurable};

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
        let client_id = section
            .get("client_id")
            .and_then(|i| i.as_str())
            .unwrap_or_default();
        let client_secret = section
            .get("client_secret")
            .and_then(|i| i.as_str())
            .unwrap_or_default()
            .chars()
            .map(|_| 'X')
            .collect::<String>();
        let bot_token = section
            .get("bot_token")
            .and_then(|i| i.as_str())
            .unwrap_or_default()
            .chars()
            .map(|_| 'X')
            .collect::<String>();

        Ok(json!({
            "client_id": client_id,
            "client_secret": client_secret,
            "bot_token": bot_token,
        }))
    }

    fn save(&self, data: &JSON, toml: &mut dyn toml_edit::TableLike) -> Result<JSON, JSON> {
        let client_id = data
            .get("client_id")
            .and_then(|a| a.as_str())
            .unwrap_or_default();
        let client_secret = data
            .get("client_secret")
            .and_then(|a| a.as_str())
            .unwrap_or_default();
        let bot_token = data
            .get("bot_token")
            .and_then(|a| a.as_str())
            .unwrap_or_default();

        edit::update_toml_key(
            toml,
            "client_id",
            Item::Value(Value::String(Formatted::new(client_id.into()))),
            Some(
                "Client ID of your application. It may look something like `1122223333333000000`."
                    .to_string(),
            ),
            false,
        );

        if !client_secret.is_empty()
            && client_secret != "none"
            && client_secret.chars().any(|a| a != 'X')
        {
            // changed client_secret
            edit::update_toml_key(
                toml,
                "client_secret",
                Item::Value(Value::String(Formatted::new(client_secret.into()))),
                Some("Client Secret of your application. It may look something like `-eHs9Lp-3XzR-BVq5r8HWEXodJNNKGtx`.".to_string()),
                false,
            );
        }

        if !bot_token.is_empty()
            && bot_token != "none"
            && bot_token.chars().any(|a| a != 'X')
        {
            // changed bot_token
            edit::update_toml_key(
                toml,
                "bot_token",
                Item::Value(Value::String(Formatted::new(bot_token.into()))),
                Some("Bot Token of your application. It may look something like `MTEzMjc3MzE2MTk4NTkwODc4Nw.G3rB1Z.P01iwhnwt6R-JtqixNb6nEp1bQr8ljzid6jiXc`.".to_string()),
                false,
            );
        }

        Ok(json!({}))
    }
}
