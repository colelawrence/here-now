use std::sync::Arc;

use hn_app::{_ecs_::ecs_unique, app_ctx::AppCtx};
use shipyard::{UniqueView, UniqueViewMut};
use shipyard_app::App;
use tokio::sync::mpsc::UnboundedSender;

mod prelude {
    #![allow(unused)]
    pub use anyhow::Context;
    pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
    pub use hn_common::keys;
}

mod device;
mod device_client;
mod local_keys;

pub fn main(send_to_ui: Box<dyn ui::SendToUI>) -> Box<dyn ui::SendToExecutor> {
    // start tokio runtime
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");

    // needs to be set so the app plugins (e.g. AppCtx) can find the tokio runtime
    let _entered = rt.enter();

    let mut app = shipyard_app::App::new();
    let (sender, recv) = tokio::sync::mpsc::unbounded_channel();
    send_to_ui.send_to_ui(ui::ToUI::ShowMainWindow);
    let main_plugin = device_plugin::DevicePlugin(sender.clone());
    let workload = app.add_plugin_workload(main_plugin);
    let send_to_ui = Arc::new(send_to_ui);
    let send_to_ui_clone = send_to_ui.clone();
    let main_loop = rt.spawn(hn_app::app_ctx::start_loop(
        app,
        workload,
        recv,
        move |app: &App| {
            app.world.run(
                |mut uvm: shipyard::UniqueViewMut<device_plugin::UIMessages>| {
                    for msg in uvm.drain() {
                        send_to_ui_clone.send_to_ui(msg);
                    }
                },
            )
        },
    ));

    Box::new(Executor {
        sender,
        send_to_ui,
        _rt: rt,
        _main_loop: main_loop,
    })
}

struct Executor {
    // to own the runtime and main loop
    _rt: tokio::runtime::Runtime,
    _main_loop: tokio::task::JoinHandle<()>,
    sender: UnboundedSender<hn_app::app_ctx::Command>,
    send_to_ui: Arc<Box<dyn ui::SendToUI>>,
}

impl Executor {
    fn run<B, R, S>(&self, system: S)
    where
        S: shipyard::IntoWorkloadSystem<B, R>,
    {
        self.sender
            .send(hn_app::app_ctx::Command::new(
                "sent message to executor",
                true,
                None,
                system,
            ))
            .expect("send to executor")
    }
    // fn send_to_ui(&self, to_ui: ui::ToUI) {
    //     self.sender
    //         .send(hn_app::app_ctx::Command::new(
    //             "sent message to executor",
    //             true,
    //             None,
    //             move |mut uvm_ui_messages: UniqueViewMut<device_plugin::UIMessages>| {
    //                 uvm_ui_messages.add(to_ui.clone());
    //             },
    //         ))
    //         .expect("send to executor")
    // }
}

impl ui::SendToExecutor for Executor {
    #[tracing::instrument(skip(self))]
    fn send_to_executor(&self, msg: ui::ToExecutor) {
        use shipyard::*;
        match msg {
            ui::ToExecutor::OpenSettings => {
                tracing::info!("open settings window");
                self.run(
                    |mut uvm_ui_messages: UniqueViewMut<device_plugin::UIMessages>,
                     uv_settings_url_1: UniqueView<device_plugin::SettingServerURL1>,
                     uv_settings_url_2: UniqueView<device_plugin::SettingServerURL2>| {
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
                    },
                );
            }
            ui::ToExecutor::HidMainWindow => {
                tracing::info!("hid main window");
                let send_to_ui_clone = self.send_to_ui.clone();
                std::thread::spawn(move || {
                    // experiment to show we can re-show the main window
                    tracing::warn!("waiting a second to reshow main window");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    send_to_ui_clone.send_to_ui(ui::ToUI::ShowMainWindow);
                });
            }
            ui::ToExecutor::HidSettings => {
                tracing::info!("hid settings window");
                // to do: update the status of whether the settings window is open
            }
            ui::ToExecutor::OpenMainWindow => {
                tracing::info!("open main window");
                self.send_to_ui.send_to_ui(ui::ToUI::ShowMainWindow);
            }
            ui::ToExecutor::UpdateSettings(settings) => {
                self.run(
                    move |mut uvm_ui_messages: UniqueViewMut<device_plugin::UIMessages>,
                          mut uvm_settings_url_1: UniqueViewMut<
                        device_plugin::SettingServerURL1,
                    >,
                          mut uvm_settings_url_2: UniqueViewMut<
                        device_plugin::SettingServerURL2,
                    >| {
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
                );
            }
        }
    }
}

mod device_plugin {
    use std::{marker::PhantomData, path::PathBuf};

    use bonsaidb::core::schema;
    use hn_app::{
        _ecs_::*,
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
}
