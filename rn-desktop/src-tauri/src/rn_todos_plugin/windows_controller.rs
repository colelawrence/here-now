use tauri::{Manager, Runtime, Window};

const PLANNER_WINDOW_LABEL: &str = "planner";
const TRACKER_WINDOW_LABEL: &str = "tracker";
const TRAY_WINDOW_LABEL: &str = "tray";

// in logical sizes
const TRACKER_MIN_WIDTH: f64 = 400f64;
const TRACKER_MAX_WIDTH: f64 = 1200f64;
// TODO: make configurable
const TRACKER_HEIGHT: f64 = 58f64;

pub fn get_planner_window<R: Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> tauri::Result<Option<tauri::Window<R>>> {
    Ok(app_handle.get_window(PLANNER_WINDOW_LABEL))
}

pub fn ensure_planner_window<R: tauri::Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> tauri::Result<tauri::Window<R>> {
    if let Some(existing) = app_handle.get_window(PLANNER_WINDOW_LABEL) {
        return Ok(existing);
    }

    tauri::WindowBuilder::new(
        app_handle,
        PLANNER_WINDOW_LABEL,
        tauri::WindowUrl::App("/planner-window".into()),
    )
    .title("Right Now Planner")
    .hidden_title(true)
    .title_bar_style(tauri::TitleBarStyle::Overlay)
    // .max_inner_size(1000, 4000)
    .inner_size(350f64, 350f64)
    .min_inner_size(250f64, 200f64)
    .center()
    .build()
}

pub fn get_tray_window<R: Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> tauri::Result<Option<tauri::Window<R>>> {
    Ok(app_handle.get_window(TRAY_WINDOW_LABEL))
}

pub fn ensure_tray_window<R: tauri::Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> tauri::Result<tauri::Window<R>> {
    if let Some(existing) = app_handle.get_window(TRAY_WINDOW_LABEL) {
        return Ok(existing);
    }

    // hide with macos specific controls
    #[cfg(target_os = "macos")]
    let decorations = true;
    #[cfg(not(target_os = "macos"))]
    let decorations = false;

    let window = tauri::WindowBuilder::new(
        app_handle,
        TRAY_WINDOW_LABEL,
        tauri::WindowUrl::App("/tray".into()),
    )
    .title("Right Now Planner")
    .inner_size(400f64, 400f64)
    .title_bar_style(tauri::TitleBarStyle::Transparent)
    .always_on_top(true)
    .hidden_title(true)
    .decorations(decorations)
    .resizable(false)
    .visible(false)
    .focused(false)
    .build()?;

    #[cfg(target_os = "macos")]
    crate::macos_title_bar::hide_window_buttons(&window);

    Ok(window)
}

pub fn get_tracker_window<R: Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> tauri::Result<Option<tauri::Window<R>>> {
    Ok(app_handle.get_window(TRACKER_WINDOW_LABEL))
}

/// Use the reference to position the tracker at the bottom of the reference window
pub fn ensure_tracker_window_below(
    app_handle: &tauri::AppHandle,
    reference_window: &Window,
) -> tauri::Result<Window> {
    let scale_factor = reference_window.scale_factor()?;
    // TODO: Make tacker window state positions work correctly
    let reference_pos = reference_window
        .outer_position()?
        .to_logical::<f64>(scale_factor);
    let reference_size = reference_window
        .outer_size()?
        .to_logical::<f64>(scale_factor);
    let tracker_window = ensure_planner_window(app_handle)?;
    tracker_window.set_size(tauri::LogicalSize {
        height: TRACKER_HEIGHT,
        width: reference_size
            .width
            .clamp(TRACKER_MIN_WIDTH, TRACKER_MAX_WIDTH),
    })?;
    tracker_window.set_position(tauri::LogicalPosition {
        x: reference_pos.x,
        y: reference_pos.y + reference_size.height - TRACKER_HEIGHT,
    })?;

    Ok(tracker_window)
}

pub fn ensure_tracker_window(app_handle: &tauri::AppHandle) -> tauri::Result<tauri::Window> {
    if let Some(existing) = get_tracker_window(app_handle)? {
        return Ok(existing);
    }

    tauri::WindowBuilder::new(
        app_handle,
        TRACKER_WINDOW_LABEL,
        tauri::WindowUrl::App("/tracker".into()),
    )
    .always_on_top(true)
    .closable(false)
    .decorations(false)
    .title("Right Now Tracker")
    .hidden_title(true)
    .max_inner_size(TRACKER_MAX_WIDTH, TRACKER_HEIGHT)
    .min_inner_size(TRACKER_MIN_WIDTH, TRACKER_HEIGHT)
    .build()
}
