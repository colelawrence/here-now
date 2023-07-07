use crate::prelude::*;

use super::Configurable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordConfiguration {
    client_id: String,
    api_key: String,
}

impl Configurable for DiscordConfiguration {
    type Saving = DiscordConfiguration;

    fn template() -> HTMXPartial {
        htmx_partial!("discord.configurable.html.j2")
    }

    fn section_name() -> Cow<'static, str> {
        "discord".into()
    }

    fn view(section: &Option<Box<dyn toml_edit::TableLike>>) -> Result<Self> {
        Ok(DiscordConfiguration {
            api_key: "not-implemented".to_string(),
            client_id: "not-implemented".to_string(),
        })
    }

    fn save(data: &Self::Saving, section: &mut Box<dyn toml_edit::TableLike>) -> Result<()> {
        Err(anyhow::anyhow!("unimplemented save"))
    }
}
