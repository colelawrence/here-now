#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowButton};
use tauri::Window;

/// Used by windows controllers for tray window
#[cfg(target_os = "macos")]
pub fn hide_window_buttons<R: tauri::Runtime>(window: &Window<R>) {
    unsafe {
        let id = window.ns_window().unwrap() as cocoa::base::id;
        let close_button = id.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        let min_button = id.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
        let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
        let _: () = msg_send![close_button, setHidden: true];
        let _: () = msg_send![min_button, setHidden: true];
        let _: () = msg_send![zoom_button, setHidden: true];
    }
}
