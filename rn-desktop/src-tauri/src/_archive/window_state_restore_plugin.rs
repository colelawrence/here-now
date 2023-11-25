//! Tauri tauri_plugin_window_state should automatically manage this
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    RunEvent, Runtime,
};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

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
    PluginBuilder::new("window state restoration")
        .setup(|app| {
            app.plugin(tauri_plugin_window_state::Builder::default().build())?;
            // initialize the plugin here
            Ok(())
        })
        .on_webview_ready(|w| {
            let mut state_flags = StateFlags::all();
            state_flags.remove(StateFlags::VISIBLE);
            state_flags.remove(StateFlags::DECORATIONS);
            // TODO: Try to put this somewhere better
            w.restore_state(StateFlags::POSITION)
                .expect("restored window state")
        })
        .on_event(|app, event| match event {
            RunEvent::Ready => {
                println!("app is ready");
            }
            RunEvent::WindowEvent { label, event, .. } => {
                println!("window {label} received an event: {event:?}");
                match event {
                    // ignored events
                    tauri::WindowEvent::Focused(_)
                    | tauri::WindowEvent::ThemeChanged(_)
                    | tauri::WindowEvent::CloseRequested { .. }
                    | tauri::WindowEvent::Destroyed => {
                        return;
                    }
                    _ => {}
                }
                app.save_window_state(StateFlags::all())
                    .expect("saved window state");
            }
            _ => (),
        })
        .invoke_handler(tauri::generate_handler![do_something])
        .build()
}
