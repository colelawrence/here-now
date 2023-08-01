use std::str::FromStr;

use crate::{
    config_plugins::{self, ReadConfigFile},
    prelude::*,
};

#[derive(Default)]
pub struct DiscordSettingsPlugin(());

impl Plugin for DiscordSettingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_tracked_value(DiscordClientID(Arc::new(Err(anyhow::anyhow!(
            "Discord settings not set, yet"
        )))));
        app.add_tracked_value(DiscordClientSecret(Arc::new(Err(anyhow::anyhow!(
            "Discord settings not set, yet"
        )))));
        app.add_plugin(config_plugins::ConfigFilePlugin(DiscordConfigFile(())));
        app.add_system(index_discord_settings_system);
    }
}

/// Unique
#[derive(Debug, Component, Clone)]
#[track(All)]
pub struct DiscordClientID(pub ArcResult<String>);

/// Unique
#[derive(Debug, Component, Clone)]
#[track(All)]
pub struct DiscordClientSecret(pub ArcResult<String>);

#[derive(Component, Clone)]
#[track(All)]
pub struct DiscordConfigFile(());

impl ReadConfigFile for DiscordConfigFile {
    type Content = toml_edit::Document;
    type Error = anyhow::Error;

    fn relative_path(&self) -> &str {
        "discord.toml"
    }

    fn load(&self, bytes: &[u8]) -> Result<Self::Content, Self::Error> {
        let str = String::from_utf8(bytes.to_vec()).with_context(|| "loading toml config")?;
        let doc = toml_edit::Document::from_str(&str).with_context(|| "parsing toml as toml")?;
        Ok(doc)
    }
}

fn index_discord_settings_system(
    uv_config: UniqueView<config_plugins::ConfigFileContent<DiscordConfigFile>>,
    mut uvm_client_id: UniqueViewMut<DiscordClientID>,
    mut uvm_client_secret: UniqueViewMut<DiscordClientSecret>,
) {
    if uv_config.is_inserted_or_modified() {
        let new_client_id_res = uv_config
            .get_content()
            .context("expected config to have content")
            .and_then(|inner| inner.content.as_err_arc_ref())
            .and_then(|doc| {
                doc.get("client_id")
                    .context("Toml has client_id key defined")
            })
            .and_then(|item| {
                item.as_str()
                    .context("expected client_id to be a string")
                    .map(String::from)
            });
        let new_client_secret_res = uv_config
            .get_content()
            .context("expected config to have content")
            .and_then(|inner| inner.content.as_err_arc_ref())
            .and_then(|doc| {
                doc.get("client_secret")
                    .context("Toml has client_secret key defined")
            })
            .and_then(|item| {
                item.as_str()
                    .context("expected client_secret to be a string")
                    .map(String::from)
            });

        if uvm_client_id.0.as_ref().as_ref().ok() != new_client_id_res.as_ref().ok() {
            // as_mut marks it for modified
            uvm_client_id.as_mut().0 = Arc::new(new_client_id_res);
            info!("updated Discord client_id");
        }

        if uvm_client_secret.0.as_ref().as_ref().ok() != new_client_secret_res.as_ref().ok() {
            // as_mut marks it for modified
            uvm_client_secret.as_mut().0 = Arc::new(new_client_secret_res);
            info!("updated Discord client_secret");
        }
    }
}
