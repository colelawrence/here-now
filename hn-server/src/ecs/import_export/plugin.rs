use crate::prelude::bonsai_::*;
use crate::prelude::ecs_::*;
use crate::prelude::*;

#[derive(Default)]
pub struct SavePlugin(());

impl Plugin for SavePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let _span = tracing::info_span!("ecs::SavePlugin::build").entered();
        let db: ArcResult<local::Database> = {
            let mut storage_conf = local::config::StorageConfiguration::default();
            // get from configuration in the future?
            storage_conf.path = Some(get_crate_path().join("../data/my-db.bonsaidb"));
            Arc::new(
                local::Database::open::<super::DBSchema>(storage_conf).context("creating database"),
            )
        };
        app.add_unique(LocalDatabase(db));
        app.add_unique(super::import::LastImport::default());
        // initial load
        if let Err(err) = app.app.run(super::import::import_all) {
            error!(?err, "failed to import previous values");
        }
        // there's probably an extra write to bonsai happening
        app.add_reset_system(super::export::export_all, "save changes to disk");
    }
}

#[ecs_unique]
pub struct LocalDatabase(ArcResult<local::Database>);

impl LocalDatabase {
    pub fn get_database(&self) -> ArcResult<local::Database> {
        self.0.clone()
    }
}

impl AsRef<Result<local::Database>> for LocalDatabase {
    fn as_ref(&self) -> &Result<local::Database> {
        &self.0
    }
}
