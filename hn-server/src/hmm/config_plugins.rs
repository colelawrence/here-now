#[derive(Component)]
#[track(All)]
pub struct ConfigDirectoryPath(pub Option<PathBuf>);

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::prelude::*;

use std::{
    collections::{hash_map, HashMap},
    marker::PhantomData,
    path::PathBuf,
    time::Duration,
};

mod commands;

/// Required for your [ConfigFilePlugin] to work.
/// Update the [ConfigDirectoryPath] unique to change the directory.
#[derive(Default)]
pub struct ConfigDirectoryPlugin {
    pub default_path: Option<PathBuf>,
    /// used if applicable, if applicable, defaults to 2 seconds
    pub polling_duration: Option<Duration>,
}

/// Wraps as a new plugin, requires one [ConfigDirectoryPlugin].
pub struct ConfigFilePlugin<T>(pub T);

/// Unique component
#[derive(Component)]
#[track(All)]
pub struct ConfigFilesDirectory {
    /// None if not yet configured
    path: Option<PathBuf>,
}

/// Unique
#[derive(Component)]
#[track(All)]
struct ConfigFileContentTracker {
    path_to_version_and_body: HashMap<PathBuf, (usize, Result<Vec<u8>, Arc<std::io::Error>>)>,
}

/// Use this with [ConfigFilePlugin] to match it with [ConfigFileContent] component.
pub trait ConfigFile: Component<Tracking = track::All> {
    type Content: Sync + Sized + Send + 'static;
    type Error: Debug + Sync + Sized + Send + 'static;
    fn relative_path(&self) -> &str;
    fn load(&self, bytes: &[u8]) -> Result<Self::Content, Self::Error>;
}

#[derive(Component)]
#[track(All)]
struct ConfigFileWatchPath<C: ConfigFile> {
    /// Could be None if the config directory is not set.
    watch_path: Option<PathBuf>,
    _mark: PhantomData<C>,
}

impl<T: ConfigFile> Default for ConfigFileWatchPath<T> {
    fn default() -> Self {
        Self {
            watch_path: None,
            _mark: PhantomData,
        }
    }
}

/// Updated by the [ConfigFilesPlugin] for every entity with the [ConfigFileTracker] component.
#[derive(Component, Debug)]
#[track(All)]
pub struct ConfigFileContent<T: ConfigFile> {
    content_opt: Option<ConfigFileContentInner<T::Content, T::Error>>,
}

impl<T: ConfigFile> Default for ConfigFileContent<T> {
    fn default() -> Self {
        Self { content_opt: None }
    }
}

impl<C: ConfigFile> ConfigFileContent<C> {
    pub fn get_content(&self) -> Option<&ConfigFileContentInner<C::Content, C::Error>> {
        self.content_opt.as_ref()
    }
}

pub struct ConfigFileContentInner<C, E> {
    pub version: usize,
    pub is_unlinked: bool,
    pub full_path: PathBuf,
    pub content: Result<C, ConfigFileContentError<E>>,
}

#[derive(Debug)]
pub enum ConfigFileContentError<E> {
    LoaderError(E),
    ReadError(Arc<std::io::Error>),
}

impl<C, E> Debug for ConfigFileContentInner<C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigFileContent")
            .field("version", &self.version)
            .field("full_path", &self.full_path)
            .field(
                "content",
                match &self.content {
                    Ok(_) => &"ok",
                    Err(_) => &"err",
                },
            )
            .finish()
    }
}

impl Plugin for ConfigDirectoryPlugin {
    fn build(&self, app: &mut shipyard_app::AppBuilder) {
        let sender = app
            .app
            .run(|uv_sender: UniqueView<commands::SendCommands>| uv_sender.clone());
        app.add_tracked_value(ConfigFilesDirectory {
            path: self.default_path.clone(),
        });
        app.add_tracked_value(ConfigFileContentTracker {
            path_to_version_and_body: Default::default(),
        });
        use notify::Watcher;
        let mut watcher = notify::recommended_watcher(
            move |a: std::result::Result<notify::Event, notify::Error>| match a {
                Ok(event) => {
                    warn!(?event.paths, "TODO: watcher got an okay event");
                    sender.schedule_system(
                        move |mut uvm_tracker: UniqueViewMut<ConfigFileContentTracker>| {
                            let updated_files = event
                                .paths
                                .par_iter()
                                .map(|path| ((path.clone(), std::fs::read(path).map_err(Arc::new))))
                                .collect::<Vec<_>>();

                            for (path, read_res) in updated_files {
                                match uvm_tracker.path_to_version_and_body.entry(path) {
                                    hash_map::Entry::Occupied(mut found) => {
                                        let mut found_mut = found.get_mut();
                                        found_mut.0 += 1;
                                        found_mut.1 = read_res;
                                    }
                                    hash_map::Entry::Vacant(vacant) => {
                                        vacant.insert((0, read_res));
                                    }
                                }
                            }
                        },
                    );
                }
                Err(err) => {
                    error!(?err, "TODO: watcher got an error event");
                }
            },
        )
        .todo(f!("creating recommended watcher"));

        watcher
            .configure(
                notify::Config::default()
                    .with_poll_interval(
                        self.polling_duration
                            .clone()
                            .unwrap_or_else(|| Duration::from_secs(2)),
                    )
                    .with_compare_contents(true),
            )
            .todo(f!("configuring watcher (dur: {:?})", self.polling_duration));

        app.add_unique(ConfigFilesWatcher {
            watcher,
            watch_count: Default::default(),
        });
    }
}

#[derive(Component)]
struct ConfigFilesWatcher {
    // watch_list: HashMap<PathBuf, HashSet<EntityId>>,
    watch_count: HashMap<PathBuf, usize>,
    watcher: notify::RecommendedWatcher,
}

impl<T: ConfigFile + Sync + Send + Clone> Plugin for ConfigFilePlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_tracked_value(self.0.clone());
        app.add_tracked_value(ConfigFileContent::<T>::default());
        app.add_tracked_value(ConfigFileWatchPath::<T>::default());
        app.depends_on_plugin::<ConfigDirectoryPlugin>("provides the directory and watching pool");
        app.add_system(watch_system::<T>);
        app.add_system(load_system::<T>);
    }
}

fn load_system<C: ConfigFile>(
    uv_config: UniqueView<C>,
    uv_file_path: UniqueView<ConfigFileWatchPath<C>>,
    mut uvm_file_content: UniqueViewMut<ConfigFileContent<C>>,
    uv_content_tracker: UniqueView<ConfigFileContentTracker>,
) {
    let _ = info_span!("load_system").entered();
    if uv_content_tracker.is_inserted_or_modified() {
        let existing_opt = uvm_file_content
            .content_opt
            .as_ref()
            .map(|inner| (inner.full_path.clone(), inner.version));

        let update = if let Some(ref path) = uv_file_path.watch_path {
            match (
                uv_content_tracker.path_to_version_and_body.get(path),
                existing_opt,
            ) {
                (Some((version, read_res)), Some((existing_path, existing_version))) => {
                    if path != &existing_path || *version != existing_version {
                        Some(Some((*version, path.clone(), read_res)))
                    } else {
                        None
                    }
                }
                (Some((version, read_res)), None) => Some(Some((*version, path.clone(), read_res))),
                (None, Some(_)) => Some(None),
                (None, None) => None,
            }
        } else if existing_opt.is_some() {
            Some(None)
        } else {
            warn!(?uv_file_path.watch_path, "no existing watch path");
            None
        };

        if let Some(update_with) = update {
            let mut target = uvm_file_content.as_mut();
            target.content_opt =
                update_with.map(|(version, full_path, read_res)| ConfigFileContentInner {
                    is_unlinked: false,
                    version,
                    content: read_res
                        .as_ref()
                        .map_err(|e| ConfigFileContentError::ReadError(e.clone()))
                        .and_then(|bytes| {
                            debug!("loading bytes from content tracker");
                            uv_config
                                .load(&bytes)
                                .map_err(ConfigFileContentError::LoaderError)
                        }),
                    full_path,
                });
            debug!(?target.content_opt, "load updated");
        }
    }
}

fn watch_system<C: ConfigFile>(
    uv_dir: UniqueView<ConfigFilesDirectory>,
    mut uvm_watcher: UniqueViewMut<ConfigFilesWatcher>,
    uv_config: UniqueView<C>,
    mut uvm_file_path: UniqueViewMut<ConfigFileWatchPath<C>>,
    mut uvm_file_content: UniqueViewMut<ConfigFileContent<C>>,
) {
    let _ = info_span!("watch_system").entered();
    use notify::Watcher;
    if uv_dir.is_inserted_or_modified() || uv_config.is_inserted_or_modified() {
        match &uv_dir.path {
            Some(dir) => {
                let joined = dir.join(uv_config.relative_path());
                let new_watch_path = joined
                    .canonicalize()
                    .todo(f!("expect canonicalization of {joined:?}"));
                if let Some(ref watched_path) = uvm_file_path.watch_path {
                    if &new_watch_path == watched_path {
                        // still same path
                        return;
                    }
                } else {
                    // update with new watch path
                    uvm_file_path.watch_path = Some(new_watch_path);
                }
            }
            None => {
                if uvm_file_path.watch_path.is_none() {
                    // nothing to do
                    return;
                }
            }
        }

        let ConfigFilesWatcher {
            ref mut watcher,
            ref mut watch_count,
        } = uvm_watcher.as_mut();

        if let Some(watch_path) = uvm_file_path.watch_path.as_ref() {
            let watchers = watch_count
                .entry(watch_path.clone())
                .and_modify(|c| *c -= 1)
                .or_default();
            // TODO: unwatch
            // if *watchers == 0 {
            //     watcher.unwatch(&watch_path).expect("unwatching");
            // }
        }

        if let Some(ref base_dir) = uv_dir.path {
            let watch_path = base_dir.join(uv_config.relative_path());
            let watch_path = watch_path
                .canonicalize()
                .todo(f!("canonicalizing path ({watch_path:?}) to config file"));

            debug!(?watch_path, "adding path to watch");

            let cur = watch_count.entry(watch_path.clone()).or_default();
            if *cur == 0 {
                watcher
                    .watch(&watch_path, notify::RecursiveMode::NonRecursive)
                    .todo(f!("watching file"));
            }

            *cur += 1;
        } else {
            // no linked path
            uvm_file_content.content_opt = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};

    use crate::prelude::*;

    #[derive(Component, Clone)]
    #[track(All)]
    struct TestConfig;

    impl super::ConfigFile for TestConfig {
        type Content = toml_edit::Document;
        type Error = anyhow::Error;

        fn relative_path(&self) -> &str {
            "./test.toml"
        }

        fn load(&self, bytes: &[u8]) -> Result<Self::Content, Self::Error> {
            let str =
                String::from_utf8(bytes.to_vec()).with_context(|| "loading test.toml config")?;
            let doc =
                toml_edit::Document::from_str(&str).with_context(|| "parsing test.toml as toml")?;
            Ok(doc)
        }
    }

    fn update_config_system(uv_content: UniqueView<super::ConfigFileContent<TestConfig>>) {
        if uv_content.is_inserted_or_modified() {
            info!("test config was modified");
            if let Some(ref content) = uv_content.content_opt {
                if content.is_unlinked {
                    // ?
                    info!("test config was unlinked and configuration values are cleared");
                    // uvm_mine.loaded = None;
                } else {
                    info!(?content.content, "test config was loaded");
                    // uvm_mine.loaded = Some(toml_edit::Document::from_str(&content.text_content));
                }
            } else {
                // uvm_mine.loaded = None;
            }
        }
    }

    #[tokio::test]
    async fn test() {
        crate::test_logger();
        let default_conf_folder = get_crate_path().join("../conf");
        let (sender, mut recv) = tokio::sync::mpsc::unbounded_channel();
        let app = test_app3(
            super::commands::CommandsPlugin(sender),
            super::ConfigDirectoryPlugin {
                default_path: Some(default_conf_folder),
                polling_duration: None,
            },
            super::ConfigFilePlugin(TestConfig),
        );

        app.run(
            |uv_text_toml: UniqueView<super::ConfigFileContent<TestConfig>>| {
                if uv_text_toml.is_inserted_or_modified() {
                    match uv_text_toml.get_content() {
                        Some(content) => match content.content {
                            Ok(ref doc) => {
                                dbg!(doc);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            },
        );

        let app_arc = Arc::new(app);
        let app = app_arc.clone();

        app.update();
        tokio::spawn(async move {
            let mut i = 0usize;
            loop {
                if let Some(cmd) = recv.recv().await {
                    // let mut systems = Vec::new();
                    // recv.try_recv()
                    i += 1;

                    let name = format!("command-{i}");
                    let info = WorkloadBuilder::new(name.clone())
                        .with_system(cmd)
                        .add_to_world(&app_arc.world)
                        .expect("adding workload");

                    app_arc
                        .world
                        .run_workload(name)
                        .todo(f!("run workload {:?}", info));

                    app_arc.update();
                } else {
                    debug!("closed");
                }
            }
        })
        .await
        .expect("...");

        // loop {
        //     app.update();
        //     std::thread::sleep(Duration::from_secs(1));
        // }
    }
}
