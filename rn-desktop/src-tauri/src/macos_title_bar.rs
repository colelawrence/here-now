#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowButton};
use tauri::Window;

/// Used by windows controllers for tray window
#[cfg(target_os = "macos")]
pub fn hide_window_buttons_all<R: tauri::Runtime>(window: &Window<R>) {
    hide_window_buttons_each(window, true, true, true);
}

/// Used by windows controllers for tray window
#[cfg(target_os = "macos")]
pub fn hide_window_buttons_each<R: tauri::Runtime>(
    window: &Window<R>,
    close: bool,
    minimize: bool,
    maximize: bool,
) {
    let win = window.clone();
    window
        .run_on_main_thread(move || unsafe {
            let id = win.ns_window().unwrap() as cocoa::base::id;
            let close_button = id.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
            let min_button = id.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
            let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
            if close {
                let _: () = msg_send![close_button, setHidden: true];
            }
            if minimize {
                let _: () = msg_send![min_button, setHidden: true];
            }
            if maximize {
                let _: () = msg_send![zoom_button, setHidden: true];
            }
        })
        .unwrap();
}
