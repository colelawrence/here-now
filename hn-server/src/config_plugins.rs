use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::prelude::*;
use hn_app::_ecs_::*;

use std::{
    collections::{hash_map, HashMap},
    marker::PhantomData,
    path::PathBuf,
    time::Duration,
};

#[derive(Component)]
#[track(All)]
pub struct ConfigDirectoryPath(pub Option<PathBuf>);

/// Required for your [ConfigFilePlugin] to work.
/// Update the [ConfigDirectoryPath] unique to change the directory.
///
/// See the trait [ConfigFile], which you'll need to implement on a type
/// to create a readable config file.
///
/// See also related components [ConfigFileContent] and []
#[derive(Default)]
pub struct ConfigDirectoryPlugin {
    pub default_path: Option<PathBuf>,
    /// used if applicable, if applicable, defaults to 2 seconds
    pub polling_duration: Option<Duration>,
}

/// Unique component
#[derive(Component)]
#[track(All)]
pub struct ConfigFilesDirectory {
    /// None if not yet configured
    pub path: Option<PathBuf>,
}

/// Use this with [ConfigFilePlugin] to match it with [ConfigFileContent] unique component.
pub trait ReadConfigFile: Component<Tracking = track::All> {
    /// TODO: Remove requirement on debug for content
    type Content: Debug + Sync + Sized + Send + 'static;
    type Error: Debug + Sync + Sized + Send + 'static;
    fn relative_path(&self) -> &str;
    fn load(&self, bytes: &[u8]) -> Result<Self::Content, Self::Error>;
}

pub trait WriteConfigFile: ReadConfigFile {
    fn save(&self, value: Self::Content) -> Vec<u8>;
}

mod internal {
    use super::*;

    /// Unique
    #[derive(Component)]
    #[track(All)]
    pub(super) struct ConfigDirectoryFileContent {
        pub path_to_version_and_body:
            HashMap<PathBuf, (usize, Result<Vec<u8>, Arc<std::io::Error>>)>,
    }

    /// Internal to plugin
    #[derive(Component)]
    #[track(All)]
    pub(super) struct ConfigFileWatchPath<C: ReadConfigFile> {
        /// Could be None if the config directory is not set.
        pub watch_path: Option<PathBuf>,
        _mark: PhantomData<C>,
    }

    impl<T: ReadConfigFile> Default for ConfigFileWatchPath<T> {
        fn default() -> Self {
            Self {
                watch_path: None,
                _mark: PhantomData,
            }
        }
    }

    pub(super) fn load_the_file_contents(
        paths: &[PathBuf],
        uvm_dir_file_content: &mut UniqueViewMut<internal::ConfigDirectoryFileContent>,
    ) {
        let _span = info_span!("load_the_file_contents", ?paths).entered();
        let updated_files = paths
            .par_iter()
            .map(|path| {
                let _span = info_span!("read file content", ?path).entered();
                (path.clone(), std::fs::read(path).map_err(Arc::new))
            })
            .collect::<Vec<_>>();

        for (path, read_res) in updated_files {
            match uvm_dir_file_content.path_to_version_and_body.entry(path) {
                hash_map::Entry::Occupied(mut found) => {
                    let found_mut = found.get_mut();
                    found_mut.0 += 1;
                    found_mut.1 = read_res;
                }
                hash_map::Entry::Vacant(vacant) => {
                    vacant.insert((0, read_res));
                }
            }
        }
    }
}

/// Updated by the [ConfigFilesPlugin] for every entity with the [ConfigFileTracker] component.
#[derive(Component, Debug)]
#[track(All)]
pub struct ConfigFileContent<T: ReadConfigFile> {
    content_opt: Option<ConfigFileContentInner<T::Content, T::Error>>,
}

impl<T: ReadConfigFile> Default for ConfigFileContent<T> {
    fn default() -> Self {
        Self { content_opt: None }
    }
}

impl<C: ReadConfigFile> ConfigFileContent<C> {
    pub fn get_content(&self) -> Option<&ConfigFileContentInner<C::Content, C::Error>> {
        self.content_opt.as_ref()
    }
}

#[derive(Debug)]
pub struct ConfigFileContentInner<C, E> {
    pub version: usize,
    pub is_unlinked: bool,
    pub full_path: PathBuf,
    pub content: Arc<Result<C, ConfigFileContentError<E>>>,
}

#[derive(Debug)]
pub enum ConfigFileContentError<E> {
    LoaderError(E),
    ReadError(Arc<std::io::Error>),
}

impl<E: std::fmt::Display> std::fmt::Display for ConfigFileContentError<E> {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigFileContentError::LoaderError(err) => {
                write!(&mut f, "Error while loading: {err}")
            }
            ConfigFileContentError::ReadError(err) => {
                write!(&mut f, "Error while reading file: {err}")
            }
        }
    }
}

impl<E: std::fmt::Display + std::fmt::Debug> std::error::Error for ConfigFileContentError<E> {}

impl Plugin for ConfigDirectoryPlugin {
    fn build(&self, app: &mut shipyard_app::AppBuilder) {
        let ctx = app.ctx();
        app.add_tracked_value(ConfigFilesDirectory {
            path: self.default_path.clone(),
        });
        app.add_tracked_value(internal::ConfigDirectoryFileContent {
            path_to_version_and_body: Default::default(),
        });
        use notify::Watcher;
        let mut watcher = notify::recommended_watcher(
            move |a: std::result::Result<notify::Event, notify::Error>| match a {
                Ok(event) => {
                    debug!(?event.paths, "watcher got an event, so we're scheduling a system");
                    ctx.schedule_system_dedup(
                        "load file contents for directory",
                        format!("{:?}", event.paths),
                        move |mut uvm_tracker: UniqueViewMut<
                            internal::ConfigDirectoryFileContent,
                        >| {
                            internal::load_the_file_contents(&event.paths, &mut uvm_tracker);
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

pub use config_file_plugin::ConfigFilePlugin;

mod config_file_plugin {
    use std::any::type_name;

    use crate::prelude::*;
    use hn_app::_ecs_::*;

    use super::{
        internal, ConfigDirectoryPlugin, ConfigFileContent, ConfigFileContentError,
        ConfigFileContentInner, ConfigFilesDirectory, ConfigFilesWatcher, ReadConfigFile,
    };

    /// Wraps as a new plugin, requires one [ConfigDirectoryPlugin].
    pub struct ConfigFilePlugin<T>(pub T);

    impl<T: ReadConfigFile + Sync + Send + Clone> Plugin for ConfigFilePlugin<T> {
        fn build(&self, app: &mut AppBuilder) {
            app.add_tracked_value(self.0.clone());
            app.add_tracked_value(ConfigFileContent::<T>::default());
            app.add_tracked_value(internal::ConfigFileWatchPath::<T>::default());
            app.depends_on_plugin::<ConfigDirectoryPlugin>(
                "provides the directory and watching pool",
            );
            app.add_system(watch_system::<T>);
            app.add_system(load_system::<T>);
        }
    }

    fn load_system<C: ReadConfigFile>(
        uv_config: UniqueView<C>,
        uv_file_path: UniqueView<internal::ConfigFileWatchPath<C>>,
        uv_dir_file_content: UniqueView<internal::ConfigDirectoryFileContent>,
        mut uvm_file_content: UniqueViewMut<ConfigFileContent<C>>,
    ) {
        let name = type_name::<C>();
        let _span = info_span!("load_system", ?name).entered();
        if uv_dir_file_content.is_inserted_or_modified() {
            let existing_opt = uvm_file_content
                .content_opt
                .as_ref()
                .map(|inner| (inner.full_path.clone(), inner.version));

            let update = if let Some(ref path) = uv_file_path.watch_path {
                match (
                    uv_dir_file_content.path_to_version_and_body.get(path),
                    existing_opt,
                ) {
                    (Some((version, read_res)), Some((existing_path, existing_version))) => {
                        if path != &existing_path || *version != existing_version {
                            Some(Some((*version, path.clone(), read_res)))
                        } else {
                            None
                        }
                    }
                    (Some((version, read_res)), None) => {
                        Some(Some((*version, path.clone(), read_res)))
                    }
                    (None, Some(_)) => Some(None),
                    (None, None) => None,
                }
            } else if existing_opt.is_some() {
                Some(None)
            } else {
                warn!(?uv_file_path.watch_path, ?name, "no existing watch path");
                None
            };

            if let Some(update_with) = update {
                let target = uvm_file_content.as_mut();
                target.content_opt =
                    update_with.map(|(version, full_path, read_res)| ConfigFileContentInner {
                        is_unlinked: false,
                        version,
                        content: Arc::new(
                            read_res
                                .as_ref()
                                .map_err(|e| ConfigFileContentError::ReadError(e.clone()))
                                .and_then(|bytes| {
                                    debug!(?name, "loading bytes from content tracker");
                                    uv_config
                                        .load(&bytes)
                                        .map_err(ConfigFileContentError::LoaderError)
                                }),
                        ),
                        full_path,
                    });

                debug!(?name, "load updated");
            }
        }
    }

    fn watch_system<C: ReadConfigFile>(
        uv_dir: UniqueView<ConfigFilesDirectory>,
        mut uvm_watcher: UniqueViewMut<ConfigFilesWatcher>,
        uv_config: UniqueView<C>,
        mut uvm_file_path: UniqueViewMut<internal::ConfigFileWatchPath<C>>,
        mut uvm_dir_file_content: UniqueViewMut<internal::ConfigDirectoryFileContent>,
    ) {
        let for_confg_file = type_name::<C>();
        let _span = info_span!("watch_system", ?for_confg_file).entered();
        use notify::Watcher;
        if uv_dir.is_inserted_or_modified() || uv_config.is_inserted_or_modified() {
            debug!(?for_confg_file, "detected config change");
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

            // we must update watchers and count

            let ConfigFilesWatcher {
                ref mut watcher,
                ref mut watch_count,
            } = uvm_watcher.as_mut();

            if let Some(watch_path) = uvm_file_path.watch_path.as_ref() {
                let _span = debug_span!("removing old watch path", file_path=?watch_path).entered();
                if let Some(1) = watch_count.get(watch_path) {
                    watch_count.remove(watch_path);
                    let _ = watcher.unwatch(watch_path);
                } else {
                    watch_count.entry(watch_path.clone()).and_modify(|c| {
                        if *c > 0 {
                            *c -= 1
                        }
                    });
                }
            }

            if let Some(ref base_dir) = uv_dir.path {
                debug!(?base_dir, "loading config file paths via directory");
                let watch_path = base_dir.join(uv_config.relative_path());
                let watch_path = watch_path
                    .canonicalize()
                    .todo(f!("canonicalizing path ({watch_path:?}) to config file"));

                debug!(file_path=?watch_path, "adding path to watch");

                let cur = watch_count.entry(watch_path.clone()).or_default();
                if *cur == 0 {
                    debug_span!("watcher add watched file", file_path=?watch_path).in_scope(|| {
                        watcher
                            .watch(&watch_path, notify::RecursiveMode::NonRecursive)
                            .todo(f!("watching file"))
                    });

                    // notify on first load
                    internal::load_the_file_contents(&[watch_path], &mut uvm_dir_file_content)
                }

                *cur += 1;
            } else {
                // no linked path
                // hmm: this will happen for every config file plugin, but maybe that's fine...
                uvm_dir_file_content.path_to_version_and_body.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{app_ctx, prelude::*};
    use hn_app::{_ecs_::*, app_ctx::AppCtxPlugin};

    #[derive(Component, Clone)]
    #[track(All)]
    struct TestConfig;

    impl super::ReadConfigFile for TestConfig {
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

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.tracks::<super::ConfigFileContent<TestConfig>>("track changes to my configuration");
            app.add_system(update_config_system);
        }
    }

    #[tokio::test]
    #[ignore = "this test doesn't stop, it's just for testing"]
    async fn test() {
        test_logger();
        let default_conf_folder = get_crate_path().join("../conf");
        let (sender, mut recv) = tokio::sync::mpsc::unbounded_channel();
        let app = test_ecs::test_app4(
            AppCtxPlugin(sender),
            super::ConfigDirectoryPlugin {
                default_path: Some(default_conf_folder),
                polling_duration: None,
            },
            super::ConfigFilePlugin(TestConfig),
            TestPlugin,
        );

        app.run(
            |uv_text_toml: UniqueView<super::ConfigFileContent<TestConfig>>| {
                if uv_text_toml.is_inserted_or_modified() {
                    match uv_text_toml.get_content() {
                        Some(content) => match content.content.as_ref() {
                            Ok(ref doc) => {
                                info!(?doc, "found doc");
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
                if let Some(app_ctx::Command {
                    reason: _,
                    dedup: _,
                    immediate: _,
                    span: _,
                    system,
                }) = recv.recv().await
                {
                    // let mut systems = Vec::new();
                    // recv.try_recv()
                    i += 1;

                    let name = format!("command-{i}");
                    let info = WorkloadBuilder::new(name.clone())
                        .with_system(system)
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
    }
}
