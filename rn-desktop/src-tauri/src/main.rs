// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod window_state_restore_plugin;

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

    let db_url = format!(
        "sqlite:{}?mode=rwc",
        app_dir
            .join("rightnow.db")
            .to_str()
            .expect("app dir is utf8 compatible")
    );

    let _db = db_setup(&db_url)
        .await
        .with_context(|| format!("setup with db at {db_url:?}"))?;

    tauri::Builder::default()
        // .setup({
        //     let app_dir = app_dir.clone();
        //     move |app| {
        //         let salt_path = app_dir.join("rnsalt.txt");
        //         app.handle()
        //             .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
        //         Ok(())
        //     }
        // })
        // Check out https://github.com/tauri-apps/awesome-tauri#plugins
        // .system_tray(tray)
        // .on_system_tray_event(on_system_tray_event)
        .invoke_handler(tauri::generate_handler![greet])
        // .enable_macos_default_menu(false)
        .plugin(window_state_restore_plugin::init())
        .run(context)
        .context("error while running tauri application")
}
