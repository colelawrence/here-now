use std::str::FromStr;

use serde::de::DeserializeOwned;
use toml_edit::TableLike;

use crate::{config_html_server::discord, prelude::*};

#[derive(Deserialize, Debug)]
pub struct ConfigContent {
    pub app: Option<AppConfiguration>,
    pub discord: Option<discord::DiscordConfiguration>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl Configurable for AppConfiguration {
    type Saving = AppConfiguration;

    fn template() -> HTMXPartial {
        htmx_partial!("config/config-app.configurable.html.j2")
    }

    fn section_name() -> Cow<'static, str> {
        "app".into()
    }

    fn view(section: &Option<Box<dyn toml_edit::TableLike>>) -> Result<Self> {
        Err(anyhow::anyhow!("unimplemented view"))
    }

    fn save(saving: &Self::Saving, section: &mut Box<dyn toml_edit::TableLike>) -> Result<()> {
        Err(anyhow::anyhow!("unimplemented save"))
    }
}

#[derive(Clone, Debug)]
pub struct ConfigFile {
    // TODO: I'd like each kind of configuration to be managed in a separate file.
    pub config_file_path: std::path::PathBuf,
    // pub configurables: BTreeMap<String, Box<dyn Configurable>>,
    /// Initially loaded configuration
    pub initial_app: Arc<AppConfiguration>,
}

pub(crate) trait Configurable: Sized + DeserializeOwned + Serialize + Debug + Clone {
    /// Used for registering the post endpoint
    type Saving: DeserializeOwned + Send + Debug;
    // TODO: some way to embed into binaries automatically?
    fn template() -> HTMXPartial;
    /// The section in the configuration that the values get placed into
    fn section_name() -> Cow<'static, str>;
    // maybe a separate type for viewing?
    fn view(section: &Option<Box<dyn toml_edit::TableLike>>) -> Result<Self>;
    fn save(saving: &Self::Saving, section: &mut Box<dyn toml_edit::TableLike>) -> Result<()>;
}

impl ConfigFile {
    pub async fn get_section_impulsively(
        &self,
        _section_name: &str,
    ) -> Result<Option<Box<dyn toml_edit::TableLike>>> {
        let file_path = &self.config_file_path;
        let file = tokio::fs::read_to_string(file_path)
            .await
            .with_context(|| format!("reading config file from {file_path:?}"))?;
        let doc = toml_edit::Document::from_str(&file)
            .with_context(|| format!("parsing toml at {file_path:?}"))?;

        // TODO: break the configurations out into sections
        // TODO: caching & reloading?
        Ok(Some(Box::new(doc.as_table().clone())))
    }
    pub async fn update_section_impulsively(
        &self,
        _section_name: &str,
        f: impl FnOnce(Box<&mut dyn toml_edit::TableLike>) -> Result<()>,
    ) -> Result<bool> {
        let file_path = &self.config_file_path;
        let file = tokio::fs::read_to_string(file_path)
            .await
            .with_context(|| format!("reading config file from {file_path:?}"))?;
        let mut doc = toml_edit::Document::from_str(&file)
            .with_context(|| format!("parsing toml at {file_path:?}"))?;

        f(Box::new(doc.as_table_mut()))?;

        let updated_content = doc.to_string();
        if file != updated_content {
            // TODO: break the configurations out into sections
            // TODO: caching & reloading?
            tokio::fs::write(file_path, updated_content)
                .await
                .with_context(|| format!("writing updated config file to {file_path:?}"))?;
            return Ok(true);
        }

        Ok(false)
    }
    // pub fn get_value_str(&self, name: &str) -> Result<Option<String>> {
    //     Ok(match name {
    //         "public_bind_address" => self.content.public_bind_address.clone(),
    //         "config_server_bind_address" => self.content.config_server_bind_address.clone(),
    //         _ => {
    //             return Err(anyhow::format_err!(
    //                 "{name:?} not found as a valid string variable in the configuration"
    //             ))
    //         }
    //     })
    // }
    // pub async fn set_value_str(&self, name: &str, value: &str) -> Result<bool> {
    //     let file_path = &self.config_file_path;
    //     let file = tokio::fs::read_to_string(file_path)
    //         .await
    //         .with_context(|| format!("reading config file from {file_path:?}"))?;
    //     let mut doc = toml_edit::Document::from_str(&file)
    //         .with_context(|| format!("parsing toml at {file_path:?}"))?;

    //     match name {
    //         "config_server_bind_address" => {
    //             doc.insert(
    //                 name,
    //                 toml_edit::value(Value::String(Formatted::new(value.to_string()))),
    //             )
    //             .with_context(|| format!("inserting new value for {name:?}"))?;
    //         }
    //         // "config_server_bind_address" => self.content.config_server_bind_address.clone(),
    //         _ => {
    //             return Err(anyhow::format_err!(
    //                 "{name:?} not found as a valid string variable in the configuration"
    //             ))
    //         }
    //     }

    //     tokio::fs::write(file_path, doc.to_string())
    //         .await
    //         .with_context(|| format!("writing updated config file to {file_path:?}"))?;

    //     tracing::info!(
    //         config = ?file_path,
    //         updated = name,
    //         with = value,
    //         "updated config file value"
    //     );

    //     Ok(true)
    // }
    // pub fn get_value_bool(&self, name: &str) -> Result<Option<bool>> {
    //     Ok(match name {
    //         "dev_mode" => self.content.dev_mode.clone(),
    //         _ => {
    //             return Err(anyhow::format_err!(
    //                 "{name:?} not found as a valid boolean variable in the configuration"
    //             ))
    //         }
    //     })
    // }
}

/// Progress 0/10: Ewww... I'm not sure how configurables versus initial server launch should interact.
/// We could make it so starting a server anywhere opens a magical tunnel through something like Cloudflare
/// for the initial configuration server and troubleshooting.
pub async fn load_config_from_path(path: impl Into<std::path::PathBuf>) -> Result<ConfigFile> {
    let path_buf: std::path::PathBuf = path.into();
    let path = &path_buf
        .canonicalize()
        .with_context(|| format!("finding configuration file"))?;
    let content = tokio::fs::read(path)
        .await
        .with_context(|| format!("opening config file at {path:?}"))?;
    // just double check that the configurations are well formed
    let app_conf: AppConfiguration = toml_edit::de::from_slice(&content)
        .with_context(|| format!("loading config from file at {path:?}"))?;
    return Ok(ConfigFile {
        config_file_path: path_buf,
        initial_app: Arc::new(app_conf),
    });
}
