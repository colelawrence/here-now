use prelude::ResultExt;
use shipyard_app::AppBuilder;
use tokio;

mod app_ctx;
mod hmm;

mod logging;
mod prelude;

mod config_plugins;

#[tokio::main]
async fn main() {
    // can we make this configurable with reloading?
    logging::expect_init_logger();

    let mut app = shipyard_app::App::new();
    let (sender, mut recv) = tokio::sync::mpsc::unbounded_channel();
    let main_plugin = MainPlugin(sender);
    let (_, workload_info) = app.add_plugin_workload_with_info(main_plugin);
    let main_loop = tokio::spawn(async move {
        use crate::prelude::*;

        let mut i = 0usize;
        loop {
            if let Some(app_ctx::Command { reason, system }) = recv.recv().await {
                i += 1;

                debug!(?reason, "running command");

                let name = format!("command-{i}");
                let info = WorkloadBuilder::new(name.clone())
                    .with_system(system)
                    .add_to_world(&app.world)
                    .expect("adding workload");

                app.world
                    .run_workload(name)
                    .todo(f!("run workload {:?}", info));

                app.world
                    .run_workload(workload_info.name.clone())
                    .todo(f!("run update workload {:?}", workload_info.name));
            } else {
                debug!("closed");
            }
        }
    });

    // must await or the nested jobs get canceled with an opaque "background task failed" error.
    main_loop.await.todo(format_args!("app loop exit error"));
}

struct MainPlugin(app_ctx::CommandSender);

impl shipyard_app::Plugin for MainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(app_ctx::AppCtxPlugin(self.0.clone()))
            .add_plugin(config_plugins::ConfigDirectoryPlugin::default())
            .add_plugin(app_server_plugins::AppServerPlugin::default())
            .add_plugin(config_html_server_plugins::ConfigHtmlServerPlugin::default());
    }
}

mod config;
mod config_html_server;

mod app_server_plugins {
    use crate::prelude::*;

    #[derive(Default)]
    pub struct AppServerPlugin(());

    impl Plugin for AppServerPlugin {
        fn build(&self, app: &mut AppBuilder) {
            info!("Setting up app server plugin");
        }
    }
}

mod config_html_server_plugins {
    use crate::{config::Settings, config_html_server, prelude::*};

    #[derive(Default)]
    pub struct ConfigHtmlServerPlugin(());

    /// Unique for other plugins to access settings.
    /// This cannot be tracked, but the underlying settings do change as there are changes
    /// to the filesystem.
    #[derive(Component)]
    pub(crate) struct ConfigSettings(Arc<Settings>);

    impl ConfigSettings {
        pub fn settings(&self) -> &Arc<Settings> {
            &self.0
        }
    }

    impl Plugin for ConfigHtmlServerPlugin {
        fn build(&self, app: &mut AppBuilder) {
            let config_dir = crate::config::config_directory_setup::init_config_directory();
            let settings = Settings::new(config_dir)
                .with(config_html_server::app::AppSettings)
                .with(config_html_server::discord::DiscordSettings);
            tracing::info!("loaded {settings:#?}");
            let ctx = app.ctx();
            let arc_settings = Arc::new(settings);
            ctx.spawn(config_html_server::start(arc_settings.clone()));
            app.add_unique(ConfigSettings(arc_settings));
        }
    }
}
