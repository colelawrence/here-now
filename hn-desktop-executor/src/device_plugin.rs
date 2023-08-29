use std::{marker::PhantomData, path::PathBuf};

use hn_app::{
    _ecs_::*,
    _tracing_::*,
    app_ctx::{AppCtxPlugin, Command},
    database_plugin::LocalDatabasePlugin,
};

use self::windows_plugin::WindowsPlugin;

pub struct DevicePlugin(pub tokio::sync::mpsc::UnboundedSender<Command>);

mod data;
mod ecs;
mod export_data;
mod import_data;
mod windows_plugin;

impl Plugin for DevicePlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder.add_unique(Messages(Vec::<ui::ToExecutor>::new()));
        builder.add_unique(Messages(Vec::<ui::ToUI>::new()));
        builder.add_plugin(AppCtxPlugin(self.0.clone()));
        builder.add_plugin(LocalDatabasePlugin::<data::DBSchema> {
            path: PathBuf::from("./data/desktop-db.bonsaidb"),
            mark: PhantomData,
        });
        builder.add_plugin(WindowsPlugin::default());
        builder.add_system(import_data::import_data_from_database_system);
        builder.add_reset_system(
            export_data::sync_changes_to_database_system,
            "synchronize any changed data back into the database",
        );
    }
}

#[ecs_unique]
#[derive(Debug)]
pub struct Messages<T: 'static>(Vec<T>);

impl<T> Messages<T> {
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.0.drain(..)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn handle(&mut self, mut f: impl FnMut(&mut T) -> SetupResult<bool>) {
        self.0.retain_mut(|a| match f(a) {
            Ok(true) => false,
            Ok(false) => true,
            Err(err) => {
                error!(?err, "skipping failed message");
                false
            }
        })
    }
    pub fn add(&mut self, msg: T) {
        self.0.push(msg);
    }
}
