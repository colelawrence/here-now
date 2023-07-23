use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    path::PathBuf,
    str::FromStr,
};

use once_cell::sync::{Lazy, OnceCell};
use serde::de::DeserializeOwned;

use crate::{config_html_server::discord, prelude::*};

#[derive(Deserialize, Debug)]
pub struct ConfigContent {
    pub app: Option<AppConfiguration>,
    pub discord: Option<discord::DiscordConfiguration>,
}

static CONFIG_FOLDER_ENV_VAR: &'static str = "HERE_NOW_CONFIG_FOLDER";

static CONFIG_FOLDER: Lazy<AppConfigFolder> = Lazy::new(|| {
    match std::env::var(CONFIG_FOLDER_ENV_VAR) {
        Ok(found) => {
            return expect_config_directory_at(PathBuf::from(found));
        }
        Err(std::env::VarError::NotPresent) => {}
        Err(std::env::VarError::NotUnicode(err)) => {
            panic!("{CONFIG_FOLDER_ENV_VAR} env variable was not valid unicode: {err:?}");
        }
    }
    eprintln!("No config folder passed in environment");
    let user = directories::UserDirs::new().expect("no user directories found");
    let found = user
        .desktop_dir()
        .expect("finding desktop directory")
        .join("here-now-config");
    return expect_config_directory_at(found);
});

fn expect_config_directory_at(found: PathBuf) -> AppConfigFolder {
    match found.canonicalize() {
        Ok(path) => {
            if !path.is_dir() {
                panic!("{path:?} is not a directory.")
            }
            return AppConfigFolder(path);
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Directory not found for configuration files at {found:?}.");
                let create = inquire::Select::new(
                    "Would you like to create this directory?",
                    vec![true, false],
                )
                .prompt_skippable()
                .expect("selecting option for creating the directory");
                if let Some(true) = create {
                    std::fs::create_dir_all(found).expect("creating directory for config files");
                    return AppConfigFolder(
                        found
                            .canonicalize()
                            .expect("finding config directory after creation"),
                    );
                } else {
                    panic!("Directory does not exist.")
                }
            }
            _ => panic!("Failed to find config folder at {found:?}: {err}"),
        },
    }
}

/// The folder where all the configuration files can be stored for
/// different parts of the server's functionality
struct AppConfigFolder(PathBuf);

pub struct AppConfig {
    file_name: &'static str,
    once: Cell<Option<SectionConfiguration>>,
}

impl AppConfig {
    fn get_lines(&self) -> &SectionConfiguration {
        let mut_opt = self.once.get_mut();
        if mut_opt.is_none() {
            return self.update();
        } else {
            &mut_opt.unwrap()
        }
    }

    fn update(&self) -> &SectionConfiguration {
        std::fs::read_to_string(self.get_full_file_path());
        let prev_version = self
            .once
            .get_mut()
            .map(|prev| prev.version)
            .unwrap_or_default();
        *self.once.get_mut() = Some(SectionConfiguration {
            version: prev_version + 1,
            lines: Vec::new(),
        });
        &self.once.get_mut().unwrap()
    }

    fn get_full_file_path(&self) -> PathBuf {
        return CONFIG_FOLDER.0.join(self.file_name);
    }
}

/// References into the configuration
struct AppConfigValue<T> {
    origin: &'static AppConfig,
    name: &'static str,
    version: usize,
    _mark: PhantomData<T>,
}

impl<T: DeserializeOwned> AppConfigValue<T> {
    pub async fn read_json(&self) -> Result<T> {
        let prefix = &format!("{}=", self.name);
        match self
            .origin
            .get_lines()
            .lines
            .iter()
            .filter_map(|line| line.strip_prefix(prefix))
            .last()
        {
            Some(str) => serde_json::from_str::<T>(str).with_context(|| {
                format!(
                    "value with prefix ({prefix:?}) in {:?} has unexpected type",
                    self.origin.file_name
                )
            }),
            None => Err(anyhow::anyhow!(
                "value with prefix ({prefix:?}) not found in ({})",
                self.origin.file_name
            )),
        }
    }
}

impl AppConfig {
    pub const fn new(file_name: &'static str) -> AppConfig {
        AppConfig {
            file_name,
            once: Cell::new(None),
        }
    }

    fn get<T>(&'static self, name: &'static str) -> AppConfigValue<T> {
        AppConfigValue {
            name,
            origin: self,
            version: 0,
            _mark: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct SectionConfiguration {
    version: usize,
    // lines: Vec<ConfigurationLine>,
    lines: Vec<String>,
}

#[derive(Debug)]
pub enum ConfigurationLine {
    Empty,
    Unknown(String),
    /// Comment (full line content)
    /// e.g.`#Comment contents`
    Ignored(String),
    /// e.g.`KEY=VALUE`
    KeyValue {
        key: String,
        value: String,
    },
}

// pub struct ValueAddress<'a, T> {
//     // shared: SharedAppConfiguration,
//     /// e.g. `"discord.keys.secret"`
//     path: Cowstr,
//     copy: Result<&'a T, ValueAddressError>,
//     _mark: PhantomData<T>,
// }

// pub enum ValueAddressError {
//     Missing,
//     WrongType { found: serde_json::Value },
// }

// impl SharedAppConfiguration {
//     pub async fn get<T: DeserializeOwned>(&self, json_path: Cowstr) -> Result<T> {
//         let mut found = &self.0.read().await.value;
//         for (idx, index) in json_path.split('.').enumerate() {
//             let map = found.as_object().ok_or_else(|| {
//                 let missing_at = json_path.split('.').take(idx).collect::<Vec<_>>();
//                 anyhow::anyhow!("when looking up {json_path:?} in config, we expected to find an object value at {missing_at:?}")
//             })?;
//             found = found.get(index).ok_or_else(|| {
//                 let missing_at = json_path.split('.').take(idx + 1).collect::<Vec<_>>();
//                 anyhow::anyhow!("when looking up {json_path:?} in config, we expected to find a value at {missing_at:?}")
//             })?;
//         }

//         Ok(todo!())
//     }
// }
