use std::path::PathBuf;
static CONFIG_FOLDER_ENV_VAR: &'static str = "HERE_NOW_CONFIG_FOLDER";

pub fn init_config_directory() -> PathBuf {
    match std::env::var(CONFIG_FOLDER_ENV_VAR) {
        Ok(found) => {
            return expect_config_directory_at(PathBuf::from(found)).0;
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
    return expect_config_directory_at(found).0;
}
struct AppConfigFolder(PathBuf);

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
                    std::fs::create_dir_all(&found).expect("creating directory for config files");
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
