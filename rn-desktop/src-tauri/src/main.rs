// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use std::sync::Arc;
use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_positioner::{Position, WindowExt as _};

use hn_app::_result_::*;
use hn_app::_tracing_::*;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use tauri_plugin_window_state::StateFlags;
mod prelude {
    #![allow(unused)]
    pub use anyhow::Context;
    pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
}

mod db_gen;
mod macos_title_bar;
mod tauri_tracker_commands;
mod tauri_tray_commands;

pub(crate) mod rn_tracker_window {
    use tauri::{Manager, Window};

    /// Use the reference to position the tracker at the bottom of the reference window
    pub fn create_tracker_window(
        app_handle: &tauri::AppHandle,
        reference_window: &Window,
    ) -> tauri::Result<tauri::Window> {
        let scale_factor = reference_window.scale_factor()?;
        // in logical sizes
        let tracker_min_width = 200f64;
        let tracker_max_width = 1200f64;
        // TODO: make configurable
        let tracker_height = 58f64;

        let reference_pos = reference_window
            .outer_position()?
            .to_logical::<f64>(scale_factor);
        let reference_size = reference_window
            .outer_size()?
            .to_logical::<f64>(scale_factor);

        if let Some(existing) = app_handle.get_window(crate::TRACKER_WINDOW_LABEL) {
            existing.close()?;
        }

        tauri::WindowBuilder::new(
            app_handle,
            crate::TRACKER_WINDOW_LABEL,
            tauri::WindowUrl::App("/tracker".into()),
        )
        .always_on_top(true)
        .closable(false)
        .decorations(false)
        .title("Right Now Tracker")
        .hidden_title(true)
        .max_inner_size(tracker_max_width, tracker_height)
        .inner_size(
            reference_size
                .width
                .clamp(tracker_min_width, tracker_max_width),
            tracker_height,
        )
        .min_inner_size(tracker_min_width, tracker_height)
        .position(
            reference_pos.x,
            reference_pos.y + reference_size.height - tracker_height,
        )
        .build()
    }
}

pub(crate) mod rn_planner_window {
    use tauri::Manager;

    /// Use the reference to position the tracker at the bottom of the reference window
    pub fn create_planner_window(app_handle: &tauri::AppHandle) -> tauri::Result<tauri::Window> {
        if let Some(existing) = app_handle.get_window(crate::MAIN_WINDOW_LABEL) {
            return Ok(existing);
        }

        tauri::WindowBuilder::new(
            app_handle,
            crate::MAIN_WINDOW_LABEL,
            tauri::WindowUrl::App("/planner-window".into()),
        )
        .title("Right Now Planner")
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        // .max_inner_size(1000, 4000)
        .min_inner_size(250f64, 200f64)
        .center()
        .build()
    }
}

pub const MAIN_WINDOW_LABEL: &str = "planner";
pub const TRACKER_WINDOW_LABEL: &str = "tracker";
pub const TRAY_WINDOW_LABEL: &str = "tray";
pub const SYSTEM_TRAY_ID: &str = "tray";

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

pub enum RNMode {
    Planning,
    Tracking,
}

pub struct RNState {
    db: DatabaseConnection,
    mode: RNMode,
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

    // Checks if current instance is the primary instance
    tauri_plugin_deep_link::prepare("com.colelawrence.rightnow");

    tauri::Builder::default()
        .setup({
            // let app_dir = app_dir.clone();
            move |app| {
                let handle = app.handle();
                tauri_plugin_deep_link::register("rightnow", move |request| {
                    // TODO: Actually intercept
                    handle.emit_all("scheme-request", request).unwrap();
                })
                .unwrap();

                // Called the binary with a URL, so we need to handle that as a request
                #[cfg(not(target_os = "macos"))]
                if let Some(url) = std::env::args().nth(1) {
                    app.emit_all("scheme-request", url).unwrap();
                }

                // hide tray initially
                let tray_window = app.get_window(TRAY_WINDOW_LABEL).unwrap();
                tray_window.hide().unwrap();

                #[cfg(target_os = "macos")]
                macos_title_bar::hide_window_buttons(tray_window);

                crate::rn_planner_window::create_planner_window(&app.handle())
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
        .invoke_handler(tauri::generate_handler![
            greet,
            tauri_tracker_commands::start_work_session,
            tauri_tracker_commands::stop_work_session,
            tauri_tray_commands::update_tray,
            tauri_tray_commands::toggle_tray,
        ])
        // .enable_macos_default_menu(false)
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .skip_initial_state(crate::TRACKER_WINDOW_LABEL)
                .skip_initial_state(crate::TRAY_WINDOW_LABEL)
                .with_state_flags(StateFlags::POSITION | StateFlags::SIZE)
                .build(),
        )
        .system_tray(SystemTray::new().with_id(SYSTEM_TRAY_ID))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick { .. } => {
                    if let Some(main_window) = app.get_window(MAIN_WINDOW_LABEL) {
                        // we're in planning mode, so we should focus on that window.
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        return;
                    }

                    let tray_window = app.get_window(TRAY_WINDOW_LABEL).unwrap();
                    if tray_window.is_visible().unwrap() {
                        // tray is already open, so shift focus to tracker window
                        tray_window.hide().unwrap();
                        if let Some(tracker_window) = app.get_window(TRACKER_WINDOW_LABEL) {
                            tracker_window.show().unwrap();
                            tracker_window.set_focus().unwrap();
                        }
                    } else {
                        tray_window.move_window(Position::TrayCenter).unwrap();
                        tray_window.show().unwrap();
                        tray_window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .build(context)
        .context("error while running tauri application")?
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });

    Ok(())
}
