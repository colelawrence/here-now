use std::{collections::BTreeSet, time::Duration};

use prelude::ResultExt;
use shipyard_app::AppBuilder;
use tokio::{self, sync::Mutex};
use tracing::{info_span, Instrument};

mod app_ctx;
mod hmm;

mod data;

mod logging;
mod prelude;

mod config_plugins;

pub mod ecs;
pub mod http;
pub mod quickjs;
pub mod svelte_templates;

#[tokio::main]
async fn main() {
    // can we make this configurable with reloading?
    logging::expect_init_logger();

    let mut app = shipyard_app::App::new();
    let (sender, mut recv) = tokio::sync::mpsc::unbounded_channel();
    let main_plugin = MainPlugin(sender);
    let workload = app.add_plugin_workload(main_plugin);
    let main_loop = tokio::spawn(async move {
        use crate::prelude::*;
        let app: Arc<Mutex<App>> = Arc::new(Mutex::new(app));

        {
            let app_clone = app.clone();
            let app = app.lock().await;
            // re-insert app into world so it can be referenced
            app.run(|mut uvm_app_ctx: UniqueViewMut<AppCtx>| {
                uvm_app_ctx.as_mut().set_app(app_clone);
            });

            // initial kick off
            workload.run(&app);
        }

        let mut i = 0usize;
        while let Some(app_ctx::Command {
            reason,
            system,
            dedup,
        }) = recv.recv().await
        {
            i += 1;
            // async block so we can instrument with tracing
            async {
                // channel might continue growing?
                tokio::time::sleep(Duration::from_millis(17))
                    .instrument(info_span!("sleep to wait for additional commands"))
                    .await;

                let mut seen = BTreeSet::<(String, &'static str)>::new();
                seen.extend(dedup.map(|s| (s, reason)));

                let (name, builder) = async {
                    let name = format!("command-{i}");
                    let mut builder = WorkloadBuilder::new(name.clone());
                    builder = builder.with_system(system);

                    while let Ok(app_ctx::Command {
                        reason,
                        system,
                        dedup,
                    }) = recv.try_recv()
                    {
                        if let Some(dedup_str) = dedup {
                            let val = (dedup_str, reason);
                            if seen.contains(&val) {
                                debug!(i, reason, dedup = val.0, "skipping duplicate command");
                                continue;
                            }

                            seen.insert(val);
                        }

                        debug!(?i, ?reason, "adding command");
                        builder = builder.with_system(system);
                    }

                    (name, builder)
                }
                .instrument(info_span!("collect commands into workload"))
                .await;

                {
                    let app = app
                        .lock()
                        .instrument(info_span!("lock app for commands"))
                        .await;
                    async {
                        let info = builder.add_to_world(&app.world).expect("adding workload");
                        app.world
                            .run_workload(name)
                            .todo(f!("run collected commands workload {:?}", info));
                    }
                    .instrument(info_span!("run collected commands workload"))
                    .await
                }

                {
                    let app = app
                        .lock()
                        .instrument(info_span!("lock app for update loop"))
                        .await;
                    info_span!("run update loop").in_scope(|| {
                        workload.run(&app);
                    });
                }
            }
            .instrument(tracing::info_span!("running command", ?i, ?reason))
            .await
        }

        debug!(?i, "closed");
    });

    // must await or the nested jobs get canceled with an opaque "background task failed" error.
    main_loop.await.todo(format_args!("app loop exit error"));
}

struct MainPlugin(app_ctx::CommandSender);

impl shipyard_app::Plugin for MainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let config_dir = crate::config::config_directory_setup::init_config_directory();

        app.add_plugin(app_ctx::AppCtxPlugin(self.0.clone()))
            .add_plugin(config_plugins::ConfigDirectoryPlugin {
                default_path: Some(config_dir.clone()),
                ..Default::default()
            })
            .add_plugin(app_server_plugins::AppServerPlugin::default())
            .add_plugin(ecs::SavePlugin::default())
            .add_plugin(config_html_server_plugins::ConfigHtmlServerPlugin { config_dir });
    }
}

mod app_server_plugins;
mod config;
mod config_html_server;

mod config_html_server_plugins {
    use std::path::PathBuf;

    use crate::{config::Settings, config_html_server, prelude::*};

    pub struct ConfigHtmlServerPlugin {
        pub config_dir: PathBuf,
    }

    /// Unique for other plugins to access settings.
    /// This cannot be tracked, but the underlying settings do change as there are changes
    /// to the filesystem.
    #[derive(Component)]
    pub struct ConfigSettings(Arc<Settings>);

    impl Plugin for ConfigHtmlServerPlugin {
        fn build(&self, app: &mut AppBuilder) {
            let config_dir = crate::config::config_directory_setup::init_config_directory();
            let settings = Settings::new(config_dir)
                .with(config_html_server::app::AppSettings)
                .with(config_html_server::discord::DiscordSettings);
            tracing::info!("loaded {settings:#?}");
            let ctx = app.ctx();
            let arc_settings = Arc::new(settings);
            ctx.spawn(config_html_server::start(arc_settings.clone(), ctx.clone()));
            app.add_unique(ConfigSettings(arc_settings));
        }
    }
}
