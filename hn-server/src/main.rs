use std::sync::Arc;

use tokio;

use crate::config::Settings;

mod app_ctx;
mod hmm;

mod config;
mod config_html_server;
mod logging;
mod prelude;

mod config_plugins;

#[tokio::main]
async fn main() {
    logging::expect_init_logger();

    let config_dir = crate::config::config_directory_setup::init_config_directory();
    let settings = Settings::new(config_dir)
        .with(config_html_server::app::AppSettings)
        .with(config_html_server::discord::DiscordSettings);

    tracing::info!("loaded {settings:#?}");

    tokio::select! {
        res = config_html_server::start(Arc::new(settings)) => {
            println!("Exited private server: {res:#?}");
            // sub.shutdown();
        }
        // res = hmm_handle => {
        //     println!("Hmm exited: {res:#?}");
        // }
    }
}
