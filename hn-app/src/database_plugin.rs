use std::collections::HashSet;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::Arc;

use bonsaidb::core::schema::Schema;
use bonsaidb::local;

use crate::_ecs_::*;
use crate::_result_::*;
use crate::app_ctx::LocalDatabase;

#[ecs_unique]
#[derive(Default)]
pub struct LastImport(pub HashSet<EntityId>);

impl LastImport {
    /// For use to skip exporting entities that were changed as a result of insert from disk
    pub fn skip_once(&mut self, entity_id: EntityId) -> bool {
        self.0.remove(&entity_id)
    }
}

/// Provides unique [LocalDatabase] and [LastImport] components for synchronization.
pub struct LocalDatabasePlugin<DB> {
    // get from configuration in the future?
    pub path: PathBuf,
    pub mark: PhantomData<DB>,
}

impl<DB: Schema> Plugin for LocalDatabasePlugin<DB> {
    fn build(&self, app: &mut AppBuilder) {
        let _span = tracing::info_span!("LocalDatabase::build").entered();
        let db: ArcResult<local::Database> = {
            let mut storage_conf = local::config::StorageConfiguration::default();
            storage_conf.path = Some(self.path.clone());
            Arc::new(local::Database::open::<DB>(storage_conf).context("creating database"))
        };
        app.add_unique(LocalDatabase(db));
        app.add_unique(LastImport::default());
    }
}
