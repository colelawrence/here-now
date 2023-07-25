use std::{path::PathBuf, time::Duration};

use crate::prelude::*;
use shipyard::{EntityId, World};
use shipyard_app::{App, Plugin};
use watchable::{Watchable, Watcher};

struct Settings {
    config_files_w: Watchable<usize>,
    world: shipyard::World,
}

pub async fn start(root: Watcher<()>) {
    let config_files_w = Watchable::new(0usize);
    let world = shipyard::World::new();
    let settings = Arc::new(Settings {
        config_files_w,
        world,
    });

    // world.add_unique(component);

    // let other_h = tokio::spawn(other(settings.clone()));

    for () in root {
        // wait for shutdown
        return;
    }
}

mod config_plugins {
    #[derive(Component)]
    #[track(All)]
    pub struct ConfigDirectoryPath(pub Option<PathBuf>);

    use crate::prelude::*;
    use std::{
        collections::{HashMap, HashSet},
        path::{Path, PathBuf},
        time::Duration,
    };

    /// Hmmm: Could this be a generic "WatchedFilesPlugin" ?
    #[derive(Default)]
    pub struct ConfigFilesPlugin {
        /// used if applicable, if applicable, defaults to 2 seconds
        pub polling_duration: Option<Duration>,
    }

    /// Unique component
    #[derive(Component)]
    #[track(All)]
    pub struct ConfigFilesDirectory {
        /// None if not yet configured
        path: Option<PathBuf>,
    }

    /// Attach this to any entity you have, and the [ConfigFilesPlugin] will watch it and update this directly on changes!
    #[derive(Component, Debug)]
    #[track(All)]
    pub struct ConfigFileTracker {
        relative_path: PathBuf,
        // // should this be on a different component?
        // content_opt: Option<ConfigFileContent>,
    }

    /// Updated by the [ConfigFilesPlugin] for every entity with the [ConfigFileTracker] component.
    #[derive(Component, Debug)]
    #[track(All)]
    pub struct ConfigFileContent {
        // should this be on a different component?
        content_opt: Option<ConfigFileContentInner>,
    }

    impl ConfigFileTracker {
        pub fn for_path<P: Into<PathBuf>>(relative_path: P) -> Self {
            ConfigFileTracker {
                relative_path: relative_path.into(),
                // content_opt: None,
            }
        }
        // pub fn get_content(&self) -> Option<&ConfigFileContent> {
        //     self.content_opt.as_ref()
        // }
        pub fn get_relative_path(&self) -> &Path {
            &self.relative_path
        }
    }

    pub struct ConfigFileContentInner {
        pub version: usize,
        pub is_unlinked: bool,
        pub full_path: PathBuf,
        pub text_content: String,
    }

    impl Debug for ConfigFileContentInner {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ConfigFileContent")
                .field("version", &self.version)
                .field("full_path", &self.full_path)
                .field("text_content(len)", &self.text_content.len())
                .finish()
        }
    }

    impl Plugin for ConfigFilesPlugin {
        fn build(&self, app: &mut shipyard_app::AppBuilder) {
            app.add_tracked_value(ConfigFilesDirectory { path: None });
            use notify::Watcher;
            let mut watcher = notify::recommended_watcher(|a| match a {
                Ok(event) => {
                    warn!(?event, "TODO: watcher got an okay event");
                    // TODO: Send an event to the systems?
                    // app.ctx().queue_system(|vm_ViewMut<>| {
                    // })
                }
                Err(err) => {
                    error!(?err, "TODO: watcher got an error event");
                }
            })
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

            app.add_system(watch_system);
            app.tracks::<ConfigFileTracker>("to register file watchers");
            app.add_unique(ConfigFilesWatcher {
                watcher,
                watch_list: Default::default(),
            });
        }
    }

    #[derive(Component)]
    struct ConfigFilesWatcher {
        watch_list: HashMap<PathBuf, HashSet<EntityId>>,
        watcher: notify::RecommendedWatcher,
    }

    fn watch_system(
        uv_dir: UniqueView<ConfigFilesDirectory>,
        mut uvm_watcher: UniqueViewMut<ConfigFilesWatcher>,
        v_file_tracker: View<ConfigFileTracker>,
        mut vm_file_content: ViewMut<ConfigFileContent>,
    ) {
        let _ = info_span!("watch_and_load_system").entered();
        use notify::Watcher;
        if uv_dir.is_modified() {
            {
                let ConfigFilesWatcher {
                    ref mut watch_list,
                    ref mut watcher,
                } = uvm_watcher.as_mut();
                for (path, _entities) in watch_list.drain() {
                    watcher.unwatch(&path).expect("unwatching");
                }
            }

            if let Some(ref base_dir) = uv_dir.path {
                for (entity, tracker) in v_file_tracker.iter().with_id() {
                    let watch_path = base_dir
                        .join(&tracker.relative_path)
                        .canonicalize()
                        // hmmm
                        .expect("canonicaling path to config file");

                    if !uvm_watcher.watch_list.contains_key(&watch_path) {
                        uvm_watcher
                            .watcher
                            .watch(&watch_path, notify::RecursiveMode::NonRecursive)
                            .todo(f!("watching file"));
                    }

                    uvm_watcher
                        .watch_list
                        .entry(watch_path)
                        .or_default()
                        .insert(entity);
                }
            } else {
                // no linked path
                for content in (&mut vm_file_content).iter() {
                    // content.is_unlinked = true;
                }
            }
        }

        for changed in v_file_tracker.inserted_or_modified().iter() {
            warn!(?changed, "changed file tracker");
        }
    }
}
