use error::Error;
use std::{ops::DerefMut, path::Path, sync::Arc};
use stored_json::StoredJSON;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    AppHandle, Manager, RunEvent, Runtime, SystemTrayEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};
use tokio::sync::Mutex;
use tracing::Instrument;

use crate::ui;
use i_rn_desktop_proc::ui_command;

type ArcRw<T> = Arc<tokio::sync::RwLock<T>>;

mod audio_controller;
mod error;
mod stored_json;
pub mod tray_controller;
pub mod windows_controller;

#[derive(Clone)]
pub struct AppState {
    pub todos: ArcRw<Vec<ui::Todo>>,
    pub todos_stored: StoredJSON<Vec<ui::Todo>>,
    pub work_state: ArcRw<ui::WorkState>,
    pub work_state_stored: StoredJSON<ui::WorkState>,
    /// Keep track of the current active task that will switch the mode automatically,
    /// so we can cancel that task when switching
    pub work_state_timer: Arc<Mutex<Option<tokio::task::AbortHandle>>>,
    pub app_settings: ArcRw<ui::AppSettings>,
    pub app_settings_stored: StoredJSON<ui::AppSettings>,
}

const UI_UPDATE_EVENT: &str = "ui_update";

fn broadcast_ui_update<R: Runtime>(
    app: &tauri::AppHandle<R>,
    upd: ui::ToUIUpdate,
) -> Result<(), Error> {
    app.emit_all(UI_UPDATE_EVENT, upd)?;
    Ok(())
}

/// `invoke("get_all_todos", {})`

#[ui_command]
async fn get_all_todos<R: Runtime>(app: tauri::AppHandle<R>) -> Result<Vec<ui::Todo>, Error> {
    Ok(app.state::<AppState>().todos.read().await.clone())
}

/// `invoke("update_todo_fields", { uid, fields, template })`

#[ui_command]
async fn update_todo_fields<R: Runtime>(
    app: tauri::AppHandle<R>,
    uid: ui::UID,
    fields: ui::TodoFields,
    template: bool,
) -> Result<(), Error> {
    if template {
        for todo in app
            .state::<AppState>()
            .app_settings
            .write()
            .await
            .template_todos
            .iter_mut()
        {
            if todo.uid == uid {
                if todo.fields != fields {
                    todo.fields = fields;
                    broadcast_ui_update(
                        &app,
                        ui::ToUITemplateTodoUpdate::UpdateFields(todo.fields.clone())
                            .into_update(uid.clone()),
                    )?;
                }
                return Ok(());
            }
        }
        return Err(Error::App(format!("Template todo ({uid:?}) not found")));
    } else {
        for todo in app.state::<AppState>().todos.write().await.iter_mut() {
            if todo.uid == uid {
                if todo.fields != fields {
                    todo.fields = fields;
                    broadcast_ui_update(
                        &app,
                        ui::ToUITodoUpdate::UpdateFields(todo.fields.clone())
                            .into_update(uid.clone()),
                    )?;
                }
                return Ok(());
            }
        }
        return Err(Error::App(format!("Todo ({uid:?}) not found")));
    }
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// `invoke("update_todo_completed", { uid, completed })`
#[ui_command]
async fn update_todo_completed<R: Runtime>(
    app: tauri::AppHandle<R>,
    uid: ui::UID,
    completed: bool,
) -> Result<(), Error> {
    for todo in app.state::<AppState>().todos.write().await.iter_mut() {
        if todo.uid == uid {
            let is_marked_completed = todo.completed_at.is_some();
            if completed != is_marked_completed {
                todo.completed_at = if completed { Some(now_unix()) } else { None };
                broadcast_ui_update(
                    &app,
                    ui::ToUITodoUpdate::UpdateCompletedAt(todo.completed_at)
                        .into_update(uid.clone()),
                )?;
            }
            return Ok(());
        }
    }
    return Err(Error::App(format!("Todo ({uid:?}) not found")));
}

#[ui_command]
async fn update_todo_ord<R: Runtime>(
    app: tauri::AppHandle<R>,
    uid: ui::UID,
    ord: f64,
    template: bool,
) -> Result<(), Error> {
    if template {
        for todo in app
            .state::<AppState>()
            .app_settings
            .write()
            .await
            .template_todos
            .iter_mut()
        {
            if todo.uid == uid {
                if todo.ord_in_template_list != ord {
                    todo.ord_in_template_list = ord;
                    broadcast_ui_update(
                        &app,
                        ui::ToUITemplateTodoUpdate::UpdateOrd(todo.ord_in_template_list)
                            .into_update(uid.clone()),
                    )?;
                }
                return Ok(());
            }
        }
        return Err(Error::App(format!("Template todo ({uid:?}) not found")));
    } else {
        for todo in app.state::<AppState>().todos.write().await.iter_mut() {
            if todo.uid == uid {
                if todo.ord != ord {
                    todo.ord = ord;
                    broadcast_ui_update(
                        &app,
                        ui::ToUITodoUpdate::UpdateOrd(todo.ord).into_update(uid.clone()),
                    )?;
                }
                return Ok(());
            }
        }
        return Err(Error::App(format!("Todo ({uid:?}) not found")));
    }
}

#[ui_command]
async fn add_todo<R: Runtime>(
    app: tauri::AppHandle<R>,
    uid: ui::UID,
    ord: f64,
    fields: ui::TodoFields,
    template: bool,
) -> Result<(), Error> {
    if template {
        let template_todo = ui::TemplateTodo {
            fields,
            ord_in_template_list: ord,
            uid,
        };
        app.state::<AppState>()
            .app_settings
            .write()
            .await
            .template_todos
            .push(template_todo.clone());
        broadcast_ui_update(&app, ui::ToUIUpdate::AddTemplateTodo(template_todo))?;
        Ok(())
    } else {
        let todo = ui::Todo {
            completed_at: None,
            worked: Vec::new(),
            fields,
            ord,
            uid,
        };
        app.state::<AppState>()
            .todos
            .write()
            .await
            .push(todo.clone());
        broadcast_ui_update(&app, ui::ToUIUpdate::AddTodo(todo))?;
        Ok(())
    }
}

#[ui_command]
async fn delete_todo<R: Runtime>(
    app: tauri::AppHandle<R>,
    uid: ui::UID,
    template: bool,
) -> Result<(), Error> {
    if template {
        app.state::<AppState>()
            .app_settings
            .write()
            .await
            .template_todos
            .retain(|a| a.uid != uid);
        broadcast_ui_update(&app, ui::ToUIUpdate::RemoveTemplateTodo(uid))?;
        Ok(())
    } else {
        app.state::<AppState>()
            .todos
            .write()
            .await
            .retain(|a| a.uid != uid);
        broadcast_ui_update(&app, ui::ToUIUpdate::RemoveTodo(uid))?;
        Ok(())
    }
}

#[ui_command]
async fn start_session(app: tauri::AppHandle) -> Result<(), Error> {
    let working_secs = {
        let state = app.state::<AppState>();
        let settings = state.app_settings.read().await;
        settings.working_secs
    };

    let started_at_unix = now_unix();
    set_work_state(
        &app,
        ui::WorkState::Working {
            ends_at_unix: started_at_unix + working_secs,
            started_at_unix,
        },
    )
    .await?;

    Ok(())
}

#[ui_command]
async fn continue_working(app: tauri::AppHandle) -> Result<(), Error> {
    start_session(app).await?;

    Ok(())
}

#[ui_command]
async fn take_a_break(app: tauri::AppHandle) -> Result<(), Error> {
    let break_secs = {
        let state = app.state::<AppState>();
        let settings = state.app_settings.read().await;
        settings.break_secs
    };
    let started_at_unix = now_unix();
    set_work_state(
        &app,
        ui::WorkState::Break {
            ends_at_unix: started_at_unix + break_secs,
            started_at_unix,
        },
    )
    .await?;
    Ok(())
}

#[ui_command]
async fn toggle_size(app: tauri::AppHandle, big: bool) -> Result<(), Error> {
    // match app.state::<AppState>().work_state.read().await.deref() {
    //     ui::WorkState::Working { .. } => {}
    //     other => {
    //         return Err(Error::App(format!(
    //             "Can only toggle size during work\n{other:?}"
    //         )))
    //     }
    // }

    if big {
        windows_controller::ensure_planner_window(&app)?.show()?;
        if let Some(tracker) = windows_controller::get_tracker_window(&app)? {
            tracker.close()?;
        }
        tracing::info!("toggle_size: showing planner, closed tracker");
    } else {
        if let Some(planner) = windows_controller::get_planner_window(&app)? {
            windows_controller::ensure_tracker_window_below(&app, &planner)?.show()?;
            planner.close()?;
        } else {
            windows_controller::ensure_tracker_window(&app)?.show()?;
        };
        tracing::info!("toggle_size: hid planner, showed tracker");
    }

    Ok(())
}

#[ui_command]
async fn stop_session(app: tauri::AppHandle) -> Result<(), Error> {
    // Future: Some "summary" view?
    set_work_state(&app, ui::WorkState::Planning).await?;
    toggle_size(app, false).await?;
    Ok(())
}

/// Will return the current instant if unix is in the past.
fn unix_to_instant(unix: u64) -> tokio::time::Instant {
    let now = now_unix();
    if now > unix {
        tracing::error!(?now, ?unix, "Unexpected creation of a time before now");
        tokio::time::Instant::now()
    } else {
        let dur_secs = unix - now;
        tokio::time::Instant::now()
            .checked_add(std::time::Duration::from_secs(dur_secs))
            .unwrap()
    }
}

async fn set_work_state(app: &tauri::AppHandle, update: ui::WorkState) -> Result<(), Error> {
    const WARN_SECS_BEFORE_BREAK_TO_WORKING: u64 = 60;
    const WARN_SECS_MIN_DUR_TO_NOTIFY: u64 = 60;
    let state = app.state::<AppState>();
    let mut current_state = state.work_state.write().await;
    let current_state = current_state.deref_mut();
    if current_state == &update {
        // no update necessary
        return Ok(());
    }
    use ui::WorkState::*;
    // The main goal here is to start scheduling tasks
    match (&current_state, &update) {
        (Break { .. }, Break { .. }) | (Working { .. }, Working { .. }) | (Planning, Planning) => {
            // no window changes
        }
        (
            Planning | Working { .. },
            Break {
                ends_at_unix,
                started_at_unix: _,
            },
        ) => {
            let ends_at_unix = *ends_at_unix;
            let app = app.app_handle();
            // start a break
            let mut prev_timer_opt = state.work_state_timer.lock().await.replace(
                tokio::spawn(async move {
                    let start_working_at = unix_to_instant(ends_at_unix);
                    let notify_to_start_working_again =
                        unix_to_instant(ends_at_unix - WARN_SECS_BEFORE_BREAK_TO_WORKING);

                    let should_notify_return_to_work_warning = notify_to_start_working_again
                        .checked_duration_since(start_working_at)
                        .map(|dur| WARN_SECS_MIN_DUR_TO_NOTIFY < dur.as_secs())
                        .unwrap_or(false);

                    if should_notify_return_to_work_warning {
                        tokio::time::sleep_until(notify_to_start_working_again).await;
                        tracing::warn!("TODO: Make a sound to return to working");
                    }

                    tokio::time::sleep_until(start_working_at).await;
                    tracing::warn!("TODO: wait to know that the user is active");
                    app
                })
                .abort_handle(),
            );
            if let Some(timer) = prev_timer_opt.take() {
                timer.abort();
            }
        }
        (
            Planning | Break { .. },
            Working {
                ends_at_unix: _,
                started_at_unix: _,
            },
        ) => {
            // start working
        }
        (Break { .. } | Working { .. }, Planning) => {
            // start planning
        }
    }

    state.work_state_stored.write(&update).await?;

    *current_state = update;

    Ok(())
}

pub fn on_tray_event<R: Runtime>(app_handle: &AppHandle<R>, event: &SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            if let Some(_tracker) = windows_controller::get_tracker_window(app_handle).unwrap() {
                // tracker is open, so we should show the tray todos list
                let tray_window = windows_controller::ensure_tray_window(app_handle).unwrap();
                if tray_window.is_visible().unwrap() {
                    // tray is already open, so shift focus to tracker window
                    tray_window.hide().unwrap();
                    if let Some(tracker_window) =
                        windows_controller::get_tracker_window(app_handle).unwrap()
                    {
                        tracker_window.show().unwrap();
                        tracker_window.set_focus().unwrap();
                    }
                } else {
                    tray_window.move_window(Position::TrayCenter).unwrap();
                    tray_window.show().unwrap();
                    tray_window.set_focus().unwrap();
                }
            } else {
                let planner_w = windows_controller::ensure_planner_window(app_handle).unwrap();
                // we're in planning mode, so we should focus on that window.
                planner_w.show().unwrap();
                planner_w.set_focus().unwrap();
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
}

#[ui_command]
async fn load_self(app: tauri::AppHandle, window: tauri::Window) -> Result<(), Error> {
    load_for_window(&app, window.label()).await;
    Ok(())
}

async fn save_everything<R: Runtime>(app_handle: &AppHandle<R>) -> Result<(), Error> {
    let app_state = app_handle.state::<AppState>();
    let state_rguard = app_state.app_settings.read().await;
    app_state.app_settings_stored.write(&state_rguard).await?;

    let todos_rguard = app_state.todos.read().await;
    app_state.todos_stored.write(&todos_rguard).await?;

    Ok(())
}

pub async fn init(save_dir: &Path) -> TauriPlugin<tauri::Wry> {
    let save_dir = save_dir.to_path_buf();
    PluginBuilder::new("RightNowTodos")
        .setup(move |app| {
            tracing::info!("Setting up RightNowTodos");
            app.manage(AppState {
                app_settings: Default::default(),
                app_settings_stored: StoredJSON::<ui::AppSettings>::new(
                    save_dir.join("settings.json"),
                ),
                todos: Default::default(),
                todos_stored: StoredJSON::<Vec<ui::Todo>>::new(save_dir.join("todos.json")),
                work_state: Default::default(),
                work_state_stored: StoredJSON::<ui::WorkState>::new(
                    save_dir.join("work-state.json"),
                ),
                work_state_timer: Arc::default(),
            });

            Ok(())
        })
        // .on_webview_ready(|w| {
        //     let mut state_flags = StateFlags::all();
        //     state_flags.remove(StateFlags::VISIBLE);
        //     state_flags.remove(StateFlags::DECORATIONS);
        //     // TODO: Try to put this somewhere better
        //     w.restore_state(state_flags)
        //         .expect("restored window state")
        // })
        .on_event(|app, event| match event {
            RunEvent::Ready => {
                // In a separate thread, because I think we need to create windows off of main thread for Windows.
                tokio::spawn({
                    let app = app.clone();
                    async move {
                        let app_state = app.state::<AppState>();
                        if let Some(work_state) = app_state.work_state_stored.read().await.unwrap()
                        {
                            // sets up timers for automatic mode switch and such
                            set_work_state(&app, work_state.0).await.unwrap();
                        }
                        if let Some(todos) = app_state.todos_stored.read().await.unwrap() {
                            *app_state.todos.write().await = todos.0;
                        }
                        if let Some(settings) = app_state.app_settings_stored.read().await.unwrap()
                        {
                            *app_state.app_settings.write().await = settings.0;
                        }

                        let planner_w = windows_controller::ensure_planner_window(&app).unwrap();
                        planner_w.show().unwrap();
                        planner_w.set_focus().unwrap();
                        planner_w.center().unwrap();
                    }
                });
            }
            RunEvent::WindowEvent { label, event, .. } => match event {
                tauri::WindowEvent::Focused(focused) => {
                    if *focused {
                        reload_window(app, label.as_str(), "focused");
                    } else {
                        // store all data
                        let app = app.clone();
                        tokio::spawn(async move {
                            match save_everything(&app).await {
                                Ok(_) => {}
                                Err(error) => {
                                    tracing::error!(?error, "failed to save everything");
                                }
                            }
                        });
                    }
                }
                _ => {}
            },
            _ => (),
        })
        .invoke_handler(tauri::generate_handler![
            add_todo,
            delete_todo,
            load_self,
            update_todo_completed,
            update_todo_fields,
            update_todo_ord,
            get_all_todos,
            start_session,
            stop_session,
            take_a_break,
            continue_working,
            toggle_size,
        ])
        .build()
}

fn reload_window(app: &tauri::AppHandle, label: &str, reason: &str) {
    let app = app.clone();
    let span = tracing::info_span!("Initial load", ?label, ?reason);
    let label = label.to_string();
    tokio::task::spawn(async move { load_for_window(&app, &label).await }.instrument(span));
}

async fn load_for_window(app: &tauri::AppHandle, label: &str) {
    let app = app.clone();
    let state = app.state::<AppState>();
    let todos = state.todos.read().await.clone();
    let template_todos = state.app_settings.read().await.template_todos.clone();
    if let Err(err) = app.emit_to(
        label,
        UI_UPDATE_EVENT,
        ui::ToUIUpdate::LoadTodos {
            todos,
            template_todos,
        },
    ) {
        tracing::error!(?label, ?err, "Error loading todos for a new window")
    };
    let work_state = state.work_state.read().await.clone();
    if let Err(err) = app.emit_to(
        label,
        UI_UPDATE_EVENT,
        ui::ToUIUpdate::UpdateWorkState(work_state),
    ) {
        tracing::error!(?label, ?err, "Error loading work state for a new window")
    };
}
