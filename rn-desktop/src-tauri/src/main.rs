// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use std::sync::Arc;
use tauri::Manager;

use tauri_plugin_autostart::MacosLauncher;

use hn_app::_result_::*;
use hn_app::_tracing_::*;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
mod prelude {
    #![allow(unused)]
    pub use anyhow::Context;
    pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
}

mod db_gen;
mod macos_title_bar;
mod rn_todos_plugin;
mod state;
mod ui;

const TRAY_WINDOW_LABEL: &str = "tray";
const SYSTEM_TRAY_ID: &str = "tray";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[instrument]
async fn run_migrations(db_url: &str) -> Result<()> {
    let pool = sqlx::SqlitePool::connect(db_url)
        .await
        .context("connect to sqlite")?;

    // let db = pool.acquire().await.expect("acquired connection");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("migrate (from ./migrations)")?;

    Ok(())
}

#[instrument]
async fn db_setup(db_url: &str) -> Result<DatabaseConnection> {
    run_migrations(db_url).await.context("run migrations")?;
    Database::connect(db_url)
        .await
        .context("connect to database")
}

#[tauri::command]
fn report_error(window: tauri::Window, error: serde_json::Value) {
    let window_label = window.label();
    error!(?window_label, ?error, "error reported")
}

#[tokio::main]
async fn main() -> Result<()> {
    hn_tracing::expect_init_logger("rightnow-desktop");
    let context = tauri::generate_context!();
    let app_dir = Arc::new(
        std::env::var("RIGHTNOW_APP_DATA_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                tauri::api::path::app_data_dir(context.config())
                    // .unwrap_or_else(|| tauri::api::path::app_local_data_dir(context.config()))
                    .expect("found a directory to store data in")
            }),
    );

    std::fs::create_dir_all(app_dir.as_path()).expect("ensured app dir exists");

    info!("app dir: {:?}", app_dir);

    // let db_url = format!(
    //     "sqlite:{}?mode=rwc",
    //     app_dir
    //         .join("rightnow.db")
    //         .to_str()
    //         .expect("app dir is utf8 compatible")
    // );

    // let _db = db_setup(&db_url)
    //     .await
    //     .with_context(|| format!("setup with db at {db_url:?}"))?;

    eprintln!("Starting \"Right Now\" with data from {app_dir:?}");

    // // Checks if current instance is the primary instance
    // tauri_plugin_deep_link::prepare("com.colelawrence.rightnow");

    tauri::Builder::default()
        .setup({
            // let app_dir = app_dir.clone();
            move |app| {
                // tauri_plugin_deep_link::register("rightnow", move |request| {
                //     // TODO: Actually intercept
                //     handle.emit_all("scheme-request", request).unwrap();
                // })
                // .unwrap();

                // // Called the binary with a URL, so we need to handle that as a request
                // #[cfg(not(target_os = "macos"))]
                // if let Some(url) = std::env::args().nth(1) {
                //     app.emit_all("scheme-request", url).unwrap();
                // }

                let handle = app.handle();
                rn_todos_plugin::windows_controller::ensure_planner_window(&handle)
                    .unwrap()
                    .show()
                    .unwrap();

                // #[cfg(not(target_os = "macos"))]
                // tray_window.set_decorations(false).unwrap();

                // let salt_path = app_dir.join("rnsalt.txt");
                // app.handle()
                //     .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
                Ok(())
            }
        })
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![""]),
        ))
        .plugin(tauri_plugin_positioner::init())
        .plugin(rn_todos_plugin::tray_controller::init())
        .plugin(rn_todos_plugin::init())
        .invoke_handler(tauri::generate_handler![greet, report_error,])
        // Probably based on gitlight, I don't know what this does for us, though.
        // .enable_macos_default_menu(false)
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            rn_todos_plugin::on_tray_event(app, &event);
        })
        .build(context)
        .context("error while running tauri application")?
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });

    eprintln!("Exiting \"Right Now\"");

    Ok(())
}
