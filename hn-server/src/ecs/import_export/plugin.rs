use crate::prelude::bonsai_::*;
use crate::prelude::ecs_::*;
use crate::prelude::*;

#[derive(Default)]
pub struct SavePlugin(());

impl Plugin for SavePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let db: ArcResult<local::Database> = {
            let mut storage_conf = local::config::StorageConfiguration::default();
            // get from configuration in the future?
            storage_conf.path = Some(get_crate_path().join("../data/my-db.bonsaidb"));
            Arc::new(
                local::Database::open::<super::DBSchema>(storage_conf).context("creating database"),
            )
        };
        app.add_unique(LocalDatabase(db));
        app.add_reset_system(super::export::export_all, "save changes to disk");
    }
}

#[ecs_unique]
pub struct LocalDatabase(ArcResult<local::Database>);

impl AsRef<Result<local::Database>> for LocalDatabase {
    fn as_ref(&self) -> &Result<local::Database> {
        &self.0
    }
}
