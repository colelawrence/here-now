use derive_codegen::Codegen;
use serde::{Deserialize, Serialize};

pub type UID = String;

/// Future: Store this as the only state stored to disk for this app
#[derive(Serialize, Deserialize, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub struct AppSettings {
    /// How long should breaks last (5m in pomodoro)
    pub break_secs: usize,
    /// How long should work sessions last (25m in pomodoro)
    pub working_secs: usize,
    /// Todo items that can be re-used
    pub template_todos: Vec<TemplateTodo>,
}

#[derive(Serialize, Deserialize, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub enum ToUITodoUpdate {
    UpdateFields(TodoFields),
    UpdateCompletedAt(Option<usize>),
    AddWorkDuration(TodoWorkDuration),
    UpdateOrd(f64),
}

impl ToUITodoUpdate {
    pub fn into_update(self, uid: UID) -> ToUIUpdate {
        ToUIUpdate::UpdateTodo(uid, self)
    }
}

#[derive(Serialize, Deserialize, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub enum ToUITemplateTodoUpdate {
    UpdateFields(TodoFields),
    UpdateOrd(f64),
}

impl ToUITemplateTodoUpdate {
    pub fn into_update(self, uid: UID) -> ToUIUpdate {
        ToUIUpdate::UpdateTemplateTodo(uid, self)
    }
}

#[derive(Serialize, Deserialize, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub enum ToUIUpdate {
    /// Initial load
    LoadTodos {
        todos: Vec<Todo>,
        template_todos: Vec<TemplateTodo>,
    },
    UpdateWorkState(WorkState),
    AddTodo(Todo),
    UpdateTodo(UID, ToUITodoUpdate),
    RemoveTodo(UID),
    AddTemplateTodo(TemplateTodo),
    UpdateTemplateTodo(UID, ToUITemplateTodoUpdate),
    RemoveTemplateTodo(UID),
}

#[derive(Serialize, Deserialize, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub enum WorkState {
    Planning,
    Break {
        /// Time the break is over
        ends_at_unix: usize,
        /// Time the break started
        started_at_unix: usize,
    },
    Working {
        /// Time the work session is over
        ends_at_unix: usize,
        /// Time the work session started
        started_at_unix: usize,
    },
}

#[derive(Serialize, Deserialize, Debug, Codegen, Clone, PartialEq)]
#[codegen(tags = "rn-ui")]
pub struct TodoFields {
    /// Future: Full text content of the todo (first new line separates the title from description)
    /// Future: Can link to media IDs
    pub title: String,
    /// In minutes
    pub time_estimate_mins: usize,
    /// Tags for categorization and quick organization
    /// e.g. `["user:Passport", "when:later", "user:Important"]`
    pub mvp_tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub struct Todo {
    pub uid: UID,
    /// What order is this todo item in the universal ordering
    pub ord: f64,
    /// Seconds since Unix epoch
    pub completed_at: Option<usize>,
    /// Segments of work performed
    pub worked: Vec<TodoWorkDuration>,
    pub fields: TodoFields,
}

#[derive(Serialize, Deserialize, Debug, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub struct TodoWorkDuration {
    pub started_at_unix: usize,
    pub stopped_at_unix: usize,
}

#[derive(Serialize, Deserialize, Codegen, Clone)]
#[codegen(tags = "rn-ui")]
pub struct TemplateTodo {
    pub uid: UID,
    /// What order is this in the template list
    pub ord_in_template_list: f64,
    pub fields: TodoFields,
}

#[test]
#[ignore]
fn generate_ui_typescript() {
    derive_codegen::Generation::for_tag("rn-ui")
        .as_arg_of(
            std::process::Command::new("deno")
                .args("run ./dev-codegen/generate-typescript.ts".split(' '))
                .current_dir(get_crate_path()),
        )
        .write()
        .print();

    fn get_crate_path() -> std::path::PathBuf {
        let dir_from_env = std::env::var("RIGHT_NOW_SRC_TAURI_PATH").unwrap_or_else(|_| {
            std::env::var("CARGO_MANIFEST_DIR").expect(
                "RIGHT_NOW_SRC_TAURI_PATH or CARGO_MANIFEST_DIR env var pointing at rn-desktop/src-tauri folder",
            )
        });

        std::path::PathBuf::from(dir_from_env)
            .canonicalize()
            .expect("find crate path")
    }
}
