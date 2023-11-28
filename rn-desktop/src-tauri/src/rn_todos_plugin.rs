use std::{ops::DerefMut, sync::Arc};

use crate::ui;
use derive_codegen::{fn_codegen, i_codegen_derive::codegen};
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, RunEvent, Runtime,
};
use tauri_plugin_window_state::{StateFlags, WindowExt};

type ArcRw<T> = Arc<tokio::sync::RwLock<T>>;
mod error;
use error::Error;
use tokio::sync::RwLock;
use tracing::Instrument;

#[derive(Clone)]
pub struct AppState {
    pub todos: ArcRw<Vec<ui::Todo>>,
    pub work_state: ArcRw<ui::WorkState>,
    pub app_settings: ArcRw<ui::AppSettings>,
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
#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
async fn get_all_todos<R: Runtime>(app: tauri::AppHandle<R>) -> Result<Vec<ui::Todo>, Error> {
    Ok(app.state::<AppState>().todos.read().await.clone())
}

/// `invoke("update_todo_fields", { uid, fields, template })`
#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
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
        return Err(Error::Other(format!("Template todo ({uid:?}) not found")));
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
        return Err(Error::Other(format!("Todo ({uid:?}) not found")));
    }
}

fn now_unix() -> usize {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

/// `invoke("update_todo_completed", { uid, completed })`
#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
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
    return Err(Error::Other(format!("Todo ({uid:?}) not found")));
}

#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
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
        return Err(Error::Other(format!("Template todo ({uid:?}) not found")));
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
        return Err(Error::Other(format!("Todo ({uid:?}) not found")));
    }
}

#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
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

#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
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

#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
async fn start_session<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), Error> {
    let state = app.state::<AppState>();
    let mut current_state = state.work_state.write().await;
    let current_state = current_state.deref_mut();
    match current_state {
        ui::WorkState::Break {
            ends_at_unix: _,
            started_at_unix: _,
        } => {
            todo!("start from break");
        }
        ui::WorkState::Working {
            ends_at_unix: _,
            started_at_unix: _,
        } => {
            todo!("start from working");
        }
        ui::WorkState::Planning => {
            todo!("start from planning");
        }
    }
    // broadcast_ui_update(&app, ui::ToUIUpdate::AddTemplateTodo(template_todo))?;
    // Ok(())
}

#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
async fn stop_session<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), Error> {
    let state = app.state::<AppState>();
    let mut current_state = state.work_state.write().await;
    let current_state = current_state.deref_mut();
    match current_state {
        ui::WorkState::Break {
            ends_at_unix: _,
            started_at_unix: _,
        } => {
            todo!("stop from break");
        }
        ui::WorkState::Working {
            ends_at_unix: _,
            started_at_unix: _,
        } => {
            todo!("stop from working");
        }
        ui::WorkState::Planning => {
            // nothing to do?
            return Ok(());
        }
    }
    // broadcast_ui_update(&app, ui::ToUIUpdate::AddTemplateTodo(template_todo))?;
    // Ok(())
}

#[fn_codegen]
#[codegen(tauri_command, tauri_plugin = "right-now-state", tags = "rn-ui")]
#[tauri::command(async)]
#[tracing::instrument]
async fn take_a_break<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), Error> {
    let state = app.state::<AppState>();
    let mut current_state = state.work_state.write().await;
    let current_state = current_state.deref_mut();
    match current_state {
        ui::WorkState::Break {
            ends_at_unix: _,
            started_at_unix: _,
        } => {
            // nothing to do?
            return Ok(());
        }
        ui::WorkState::Working {
            ends_at_unix: _,
            started_at_unix: _,
        } => {
            todo!("break from working");
        }
        ui::WorkState::Planning => {
            todo!("break from planning?");
        }
    }
    // broadcast_ui_update(&app, ui::ToUIUpdate::AddTemplateTodo(template_todo))?;
    // Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("right-now-state")
        .setup(|app| {
            app.manage(AppState {
                app_settings: Arc::new(RwLock::new(ui::AppSettings {
                    working_secs: 25 * 60,
                    break_secs: 5 * 60,
                    template_todos: vec![
                        ui::TemplateTodo {
                            uid: "PLAN_DAY".to_string(),
                            fields: ui::TodoFields {
                                mvp_tags: vec![],
                                time_estimate_mins: 25,
                                title: "Plan your day".to_string(),
                            },
                            ord_in_template_list: 0.1f64,
                        },
                        ui::TemplateTodo {
                            uid: "WATER_PLANTS".to_string(),
                            fields: ui::TodoFields {
                                mvp_tags: vec![],
                                time_estimate_mins: 10,
                                title: "Water plants".to_string(),
                            },
                            ord_in_template_list: 0.5f64,
                        },
                    ],
                })),
                todos: Arc::new(RwLock::new(vec![])),
                work_state: Arc::new(RwLock::new(ui::WorkState::Planning)),
            });

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
            RunEvent::WindowEvent { label, event, .. } => match event {
                tauri::WindowEvent::Focused(_) => {
                    let app = app.clone();
                    let span = tracing::info_span!("Load todos on focus", ?label);
                    let label = label.clone();
                    tokio::task::spawn(
                        async move {
                            let state = app.state::<AppState>();
                            let todos = state.todos.read().await.clone();
                            let template_todos =
                                state.app_settings.read().await.template_todos.clone();
                            if let Err(err) = app.emit_to(
                                &label,
                                UI_UPDATE_EVENT,
                                ui::ToUIUpdate::LoadTodos {
                                    todos,
                                    template_todos,
                                },
                            ) {
                                tracing::error!(
                                    ?label,
                                    ?err,
                                    "Error loading todos for a new window"
                                )
                            };
                        }
                        .instrument(span),
                    );
                }
                _ => {}
            },
            _ => (),
        })
        .invoke_handler(tauri::generate_handler![
            add_todo,
            update_todo_completed,
            update_todo_fields,
            update_todo_ord,
            get_all_todos,
            start_session,
            stop_session,
            take_a_break,
        ])
        .build()
}
