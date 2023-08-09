use serde::de::DeserializeOwned;

use crate::prelude::*;
use std::{path::PathBuf, str::FromStr};

/// The root settings container for all configurable things in the app.
#[derive(Clone, Debug)]
pub struct Settings {
    config_files_directory: std::path::PathBuf,
    configurables: Vec<Arc<Box<dyn Configurable>>>,
}

impl Settings {
    pub fn new(config_files_directory: PathBuf) -> Self {
        Settings {
            config_files_directory,
            configurables: Vec::new(),
        }
    }

    pub fn with<C: Configurable + 'static>(mut self, configurable: C) -> Self {
        self.configurables.push(Arc::new(Box::new(configurable)));
        self
    }

    #[deprecated = "use .entries()"]
    pub fn configurables(&self) -> impl Iterator<Item = &Arc<Box<dyn Configurable>>> {
        self.configurables.iter()
    }
    pub fn entries<'a>(&'a self) -> impl Iterator<Item = SettingEntry<'a, 'a>> {
        self.configurables.iter().map(|c| SettingEntry {
            settings: self,
            section_name: c.section_name(),
            configurable: c.as_ref().as_ref(),
        })
    }
}

pub mod config_directory_setup;

pub trait Configurable: Send + Sync + Debug {
    // TODO: some way to embed into binaries automatically?
    fn template(&self) -> HTMXPartial;
    /// Should this be separated out?
    /// The section in the configuration that the values get placed into
    fn section_name(&self) -> Cow<'static, str>;
    /// variables passed into the template
    fn vars(&self, section: &Box<dyn toml_edit::TableLike>) -> Result<serde_json::Value>;
    /// Returned JSON will be passed back into the "view" as `ok` and into "edit" as `err`.
    fn save(
        &self,
        fields: &serde_json::Value,
        section: &mut dyn toml_edit::TableLike,
    ) -> Result<JSON, JSON>;
}

pub struct SettingEntry<'a, 'b> {
    settings: &'a Settings,
    section_name: Cow<'b, str>,
    pub configurable: &'a dyn Configurable,
}

impl<'a, 'b> SettingEntry<'a, 'b> {
    /// Private
    async fn read_toml_and_file(&self) -> Result<Option<(toml_edit::Document, String)>> {
        let mut file_path = self
            .settings
            .config_files_directory
            .join(self.section_name.as_ref());
        file_path.set_extension("toml");
        let file_path = &file_path;
        let file_res = tokio::fs::read_to_string(file_path).await;
        let file = match file_res {
            Ok(file) => file,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => return Ok(None),
                other => {
                    return Err(err).with_context(|| {
                        format!("reading config file from {file_path:?} ({other:?})")
                    })
                }
            },
        };

        let doc = toml_edit::Document::from_str(&file)
            .with_context(|| format!("parsing toml at {file_path:?}"))?;

        // TODO: break the configurations out into sections
        // TODO: caching & reloading?
        Ok(Some((doc, file)))
    }

    async fn write_file(&self, content: &str) -> Result<()> {
        let mut file_path = self
            .settings
            .config_files_directory
            .join(self.section_name.as_ref());
        file_path.set_extension("toml");
        let file_path = &file_path;
        tokio::fs::write(file_path, content)
            .await
            .with_context(|| format!("writing to {file_path:?}"))?;
        // TODO: break the configurations out into sections
        // TODO: caching & reloading?
        Ok(())
    }

    pub async fn get_toml_and_parse<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        let (document, _str) = match self.read_toml_and_file().await? {
            Some(a) => a,
            None => return Ok(None),
        };
        Ok(Some(
            toml_edit::de::from_document::<T>(document).with_context(|| "parsing document")?,
        ))
    }

    pub async fn get_toml_or_empty(&self) -> Result<Box<dyn toml_edit::TableLike>> {
        Ok(self
            .read_toml_and_file()
            .await?
            .map(|a| a.0.as_table().to_owned())
            .map(|a| Box::new(a) as Box<dyn toml_edit::TableLike>)
            .unwrap_or_else(|| Box::new(toml_edit::Table::new()) as Box<dyn toml_edit::TableLike>))
    }

    pub async fn get_view_json(&self) -> Result<JSON> {
        let toml = self.get_toml_or_empty().await?;
        Ok(self.configurable.vars(&toml)?)
    }

    pub async fn save_with(&self, json: &JSON) -> Result<Result<(JSON, bool), JSON>> {
        self.update_toml_with(|table_like| self.configurable.save(json, table_like))
            .await
    }

    /// E would usually be a JSON passed up
    pub async fn update_toml_with<T, E>(
        &self,
        f: impl FnOnce(&mut dyn toml_edit::TableLike) -> Result<T, E>,
    ) -> Result<Result<(T, bool), E>> {
        let (mut doc, file) = self
            .read_toml_and_file()
            .await
            .with_context(|| format!("reading toml for updating"))?
            .unwrap_or_default();

        let _original = doc.clone();
        let ok = match f(doc.as_table_mut()) {
            Err(inner) => return Ok(Err(inner)),
            Ok(ok) => ok,
        };

        let updated_doc_str = doc.to_string();
        if file != updated_doc_str {
            // TODO: break the configurations out into sections
            // TODO: caching & reloading?
            self.write_file(&updated_doc_str)
                .await
                .with_context(|| format!("writing updated settings"))?;
            return Ok(Ok((ok, true)));
        }

        Ok(Ok((ok, false)))
    }
}

impl Settings {
    pub fn get_entry<'a, 'b: 'a>(&'a self, section_name: &'b str) -> Option<SettingEntry> {
        self.configurables
            .iter()
            .find(|a| &a.section_name() == &section_name)
            .map(|configurable| SettingEntry {
                configurable: configurable.as_ref().as_ref(),
                section_name: Cow::Borrowed(section_name),
                settings: self,
            })
    }
}
