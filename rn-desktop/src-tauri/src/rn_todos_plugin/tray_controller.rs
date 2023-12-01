use std::{ops::DerefMut, sync::Arc};

use tauri::{
    plugin::{Builder, TauriPlugin},
    CustomMenuItem, Manager, SystemTray, SystemTrayMenu,
};
use tokio::sync::Mutex;

type TrayState = Arc<Mutex<TrayIconState>>;

const SYSTEM_TRAY_ID: &str = "tray";

pub fn init() -> TauriPlugin<tauri::Wry> {
    Builder::new("RightNowTray")
        .setup(|app| {
            app.manage::<TrayState>(Arc::new(Mutex::new(TrayIconState::DefaultIcon)));
            get_tray_handle(app)?.set_icon(get_base_icon())?;
            Ok(())
        })
        .build()
}

#[derive(Debug)]
pub enum TrayIconState {
    DefaultIcon,
    Text(String),
}

fn get_zero_width_icon() -> tauri::Icon {
    tauri::Icon::Raw(include_bytes!("../../icons/tray-0width.png").to_vec())
}
fn get_base_icon() -> tauri::Icon {
    tauri::Icon::Raw(include_bytes!("../../icons/tray-base.png").to_vec())
}

#[tracing::instrument]
pub async fn update_tray(app: &tauri::AppHandle, update: TrayIconState) -> tauri::Result<()> {
    let curr_state = app.state::<TrayState>();
    let mut curr_state = curr_state.lock().await;
    let curr_state = curr_state.deref_mut();
    match (&curr_state, &update) {
        (TrayIconState::DefaultIcon, TrayIconState::Text(add_text)) => {
            let tray = get_tray_handle(app)?;
            // add titled todo
            tray.set_icon(get_zero_width_icon())?;
            #[cfg(target_os = "macos")]
            tray.set_title(add_text)?;
        }
        (TrayIconState::Text(_), TrayIconState::DefaultIcon) => {
            get_tray_handle(app)?.set_icon(get_base_icon())?;
        }
        (TrayIconState::Text(title), TrayIconState::Text(upd_title)) => {
            if title != upd_title {
                #[cfg(target_os = "macos")]
                get_tray_handle(app)?.set_title(upd_title)?;
            }
        }
        (TrayIconState::DefaultIcon, TrayIconState::DefaultIcon) => {
            // the same
        }
    }
    *curr_state = update;
    Ok(())
}

fn get_tray_handle(app_handle: &tauri::AppHandle) -> tauri::Result<tauri::SystemTrayHandle> {
    if let Some(tray) = app_handle.tray_handle_by_id(SYSTEM_TRAY_ID) {
        Ok(tray)
    } else {
        SystemTray::new()
            .with_id(SYSTEM_TRAY_ID)
            .with_menu(
                SystemTrayMenu::new()
                    // .add_item(CustomMenuItem::new("dashboard".to_string(), "Dashboard..."))
                    // .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
            )
            .build(app_handle)
    }
}
