use std::{marker::PhantomData, path::PathBuf};

use bonsaidb::core::schema;
use hn_app::{
    _ecs_::*,
    _tracing_::*,
    app_ctx::{AppCtxPlugin, Command},
    database_plugin::LocalDatabasePlugin,
};

#[ecs_unique]
pub struct SettingServerURL1(pub Option<String>);
#[ecs_unique]
pub struct SettingServerURL2(pub Option<String>);

pub struct DevicePlugin(pub tokio::sync::mpsc::UnboundedSender<Command>);

// todo: add a collection for storing device keys
#[derive(schema::Schema)]
#[schema(name = "DesktopDBSchema", collections = [])]
pub struct DBSchema;

impl Plugin for DevicePlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder.add_unique(UIMessages(Vec::new()));
        builder.add_plugin(AppCtxPlugin(self.0.clone()));
        builder.add_plugin(LocalDatabasePlugin::<DBSchema> {
            path: PathBuf::from("./data/desktop-db.bonsaidb"),
            mark: PhantomData,
        });

        builder
            .add_unique(SettingServerURL1(None))
            .add_unique(SettingServerURL2(None));
    }
}

#[ecs_unique]
pub struct UIMessages(Vec<ui::ToUI>);

impl UIMessages {
    pub fn drain(&mut self) -> impl Iterator<Item = ui::ToUI> + '_ {
        self.0.drain(..)
    }
    pub fn add(&mut self, msg: ui::ToUI) {
        self.0.push(msg);
    }
}

pub fn open_settings(
    mut uvm_ui_messages: UniqueViewMut<UIMessages>,
    uv_settings_url_1: UniqueView<SettingServerURL1>,
    uv_settings_url_2: UniqueView<SettingServerURL2>,
) {
    uvm_ui_messages.add(ui::ToUI::ShowSettings(ui::Settings {
        server_url: uv_settings_url_1
            .0
            .as_ref()
            .map(Clone::clone)
            .map(ui::Setting::Value)
            .unwrap_or(ui::Setting::NoValue),
        server_url_2: uv_settings_url_2
            .0
            .as_ref()
            .map(Clone::clone)
            .map(ui::Setting::Value)
            .unwrap_or(ui::Setting::NoValue),
    }));
}

pub(crate) trait ExecutorAction {
    fn execute(self, executor: &super::Executor);
}

impl ExecutorAction for ui::executor::AddServerByURL {
    fn execute(self, executor: &crate::Executor) {
        warn!("TODO: add server by url");
    }
}
impl ExecutorAction for ui::executor::UpdateSettings {
    fn execute(self, executor: &crate::Executor) {
        let settings = self.settings;
        executor.run(
            move |mut uvm_ui_messages: UniqueViewMut<UIMessages>,
                  mut uvm_settings_url_1: UniqueViewMut<SettingServerURL1>,
                  mut uvm_settings_url_2: UniqueViewMut<SettingServerURL2>| {
                if let Some(value) = settings.server_url.changed() {
                    tracing::info!(?value, "updated server url 1");
                    uvm_settings_url_1.as_mut().0 = value.cloned();
                }
                if let Some(value) = settings.server_url_2.changed() {
                    tracing::info!(?value, "updated server url 2");
                    uvm_settings_url_2.as_mut().0 = value.cloned();
                }
                uvm_ui_messages.add(ui::ToUI::HideSettings);
            },
        )
    }
}
