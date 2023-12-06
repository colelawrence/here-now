use std::time::Instant;

use hn_app::HintedID;
use sea_orm::ActiveValue;
use sea_orm::DatabaseConnection;

pub struct RNTracking {
    work_session_id: i32,
    work_session_start: Instant,
}

impl RNTracking {
    pub fn work_session_start(&self) -> Instant {
        self.work_session_start
    }

    pub fn work_session_id(&self) -> i32 {
        self.work_session_id
    }
}

pub enum RNMode {
    Planning,
    Tracking(RNTracking),
}

pub struct RNState {
    db: DatabaseConnection,
    mode: RNMode,
}

impl RNState {
    pub fn mode(&self) -> &RNMode {
        &self.mode
    }
}

use crate::db_gen::prelude::*;
use crate::db_gen::todo;
use crate::prelude::*;
use sea_orm::prelude::*;

impl RNState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            mode: RNMode::Planning,
        }
    }

    // async fn emit_update<S: serde::ser::Serialize>(&self, kind: &str, id: i32, value: S) -> Result<()> {

    // }

    pub async fn create_todo(&self, initial_content: ActiveValue<String>) -> Result<HintedID> {
        let new_todo = todo::ActiveModel {
            title: initial_content,
            ..Default::default()
        };
        let todo: todo::Model = new_todo.insert(&self.db).await?;
        Ok(HintedID::try_from(todo.id.as_str())?)
    }

    pub async fn delete_todo(&self, id: HintedID) -> Result<Option<HintedID>> {
        let delete_result = Todo::delete_by_id(id.to_id_string()).exec(&self.db).await?;
        Ok(if delete_result.rows_affected > 0 {
            Some(id)
        } else {
            None
        })
    }

    pub async fn get_all_todos(&self) -> Result<Vec<todo::Model>> {
        let select: Vec<todo::Model> = Todo::find().all(&self.db).await?;
        Ok(select)
    }

    pub async fn update_todo<F>(&self, id: HintedID, mut f: F) -> Result<()>
    where
        F: FnMut(&mut todo::ActiveModel),
    {
        let todo: Option<todo::Model> = Todo::find_by_id(id.to_id_string()).one(&self.db).await?;
        // Into ActiveModel
        let mut todo: todo::ActiveModel = todo.context("expected to find Todo")?.into();
        f(&mut todo);
        let _updated_todo = todo.update(&self.db).await?;
        // self.emit_update("todo", updated_todo.id, updated_todo).await?;
        Ok(())
    }
}
