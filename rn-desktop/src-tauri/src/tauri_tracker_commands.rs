use std::time::Duration;

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

#[tauri::command(async)]
pub async fn start_work_session(
    app_handle: tauri::AppHandle,
    source_window: tauri::Window,
) -> tauri::Result<()> {
    let tray = get_tray(&app_handle).await?;
    // TODO: Actually set soemthing in the app state to say "please update my title to current task"
    #[cfg(target_os = "macos")]
    tray.set_title("Task")?;
    tray.set_icon(tauri::Icon::Raw(
        include_bytes!("../icons/tray-0width.png").to_vec(),
    ))?;
    tokio::time::sleep(Duration::from_millis(50)).await;
    let window = crate::rn_tracker_window::create_tracker_window(&app_handle, &source_window)?;
    window.show()?;
    Ok(())
}

#[tauri::command(async)]
pub async fn stop_work_session(app_handle: tauri::AppHandle) -> tauri::Result<()> {
    let tray = get_tray(&app_handle).await?;
    #[cfg(target_os = "macos")]
    tray.set_title("")?;
    tray.set_icon_as_template(true)?;
    tray.set_icon(tauri::Icon::Raw(
        include_bytes!("../icons/tray-base.png").to_vec(),
    ))?;
    tokio::time::sleep(Duration::from_millis(50)).await;
    crate::rn_planner_window::ensure_planner_window(&app_handle)?.show()?;
    Ok(())
}

async fn get_tray(app_handle: &tauri::AppHandle) -> tauri::Result<tauri::SystemTrayHandle> {
    if let Some(tray) = app_handle.tray_handle_by_id(crate::SYSTEM_TRAY_ID) {
        Ok(tray)
    } else {
        SystemTray::new()
            .with_id(crate::SYSTEM_TRAY_ID)
            .with_menu(
                SystemTrayMenu::new()
                    // .add_item(CustomMenuItem::new("dashboard".to_string(), "Dashboard..."))
                    // .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
            )
            .build(app_handle)
    }
}
