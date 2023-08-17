use hn_app::_result_::ResultExt;
use hn_app::app_ctx;
use shipyard_app::AppBuilder;
use tokio::{self};

mod data;
mod hmm;
mod prelude;

mod config_plugins;

pub mod ecs;
pub mod http;
pub mod quickjs;
pub mod svelte_templates;

#[tokio::main]
async fn main() {
    // can we make this configurable with reloading?
    hn_app::logging::expect_init_logger("hn-server");

    let mut app = shipyard_app::App::new();
    let (sender, recv) = tokio::sync::mpsc::unbounded_channel();
    let main_plugin = MainPlugin(sender);
    let workload = app.add_plugin_workload(main_plugin);
    let main_loop = tokio::spawn(app_ctx::start_loop(app, workload, recv));
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
    use hn_app::_ecs_::*;

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
