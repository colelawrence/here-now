// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hn_app::_result_::*;
use hn_app::_tracing_::*;
use sea_orm::Database;
mod prelude {
    #![allow(unused)]
    pub use anyhow::Context;
    pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
}

mod db_gen;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod plugin {
    use tauri::{
        plugin::{Builder as PluginBuilder, TauriPlugin},
        RunEvent, Runtime,
    };

    // this command can be called in the frontend using `invoke('plugin:window|do_something')`.
    #[tauri::command]
    async fn do_something<R: Runtime>(
        _app: tauri::AppHandle<R>,
        _window: tauri::Window<R>,
    ) -> Result<(), String> {
        println!("command called");
        Ok(())
    }
    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        PluginBuilder::new("window")
            .setup(|_app| {
                // initialize the plugin here
                Ok(())
            })
            .on_event(|_app, event| match event {
                RunEvent::Ready => {
                    println!("app is ready");
                }
                RunEvent::WindowEvent { label, event, .. } => {
                    println!("window {label} received an event: {event:?}");
                }
                _ => (),
            })
            .invoke_handler(tauri::generate_handler![do_something])
            .build()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    hn_tracing::expect_init_logger("rightnow-desktop");
    static URL: &str = "sqlite:rightnow.sqlite?mode=rwc";
    async {
        let pool = sqlx::SqlitePool::connect(URL)
            .await
            .todo(f!("expect to connect to sqlite at {URL}"));
        // let db = pool.acquire().await.expect("acquired connection");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .todo(f!("expect to run migrations (from ./migrations) on {URL}"));
    }
    .instrument(info_span!("migrate sqlite", ?URL))
    .await;

    let db = Database::connect(URL).await.expect("opened database");
    use db_gen::prelude::*;
    use sea_orm::entity::prelude::*;

    let all_windows = WindowPositions::find()
        .all(&db)
        .await
        .expect("found all windows");

    println!("all_windows: {:#?}", all_windows);

    // use schemars::JsonSchema;
    // let mut gen = schemars::gen::SchemaGenerator::default();
    // let obj = tauri_utils::config::Config::json_schema(&mut gen).into_object();
    // println!("{}", serde_json::to_string_pretty(&obj).unwrap());
    tauri::Builder::default()
        // .system_tray(tray)
        // .on_system_tray_event(on_system_tray_event)
        .invoke_handler(tauri::generate_handler![greet])
        // .enable_macos_default_menu(false)
        .plugin(plugin::init())
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}
