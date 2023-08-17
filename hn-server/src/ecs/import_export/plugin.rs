use std::marker::PhantomData;

use crate::prelude::*;
use hn_app::_ecs_::*;
use hn_app::database_plugin::{LastImport, LocalDatabasePlugin};

#[derive(Default)]
pub struct SavePlugin(());

impl Plugin for SavePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let _span = tracing::info_span!("ecs::SavePlugin::build").entered();
        app.add_plugin(LocalDatabasePlugin::<super::DBSchema> {
            path: get_crate_path().join("../data/my-db.bonsaidb"),
            mark: PhantomData,
        });
        app.depends_on_unique::<LastImport>("to see what was initially imported");
        // initial load
        if let Err(err) = app.app.run(super::import::import_all) {
            tracing::error!(?err, "failed to import previous values");
        }
        app.add_reset_system(super::export::export_all, "save changes to disk");
    }
}
