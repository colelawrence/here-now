use std::sync::Arc;

use hn_app::app_ctx::AppCtx;
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

impl ui::SendToExecutor for Executor {
    #[tracing::instrument(skip(self))]
    fn send_to_executor(&self, msg: ui::ToExecutor) {
        use shipyard::*;
        let send_to_ui_clone = self.send_to_ui.clone();
        self.sender
            .send(hn_app::app_ctx::Command::new(
                "sent message to executor",
                true,
                None,
                move |uv_app_ctx: UniqueView<AppCtx>, mut uvm_ui_messages: UniqueViewMut<device_plugin::UIMessages>| {
                    let _span = tracing::warn_span!("send to executor").entered();
                    let send_to_ui_clone = send_to_ui_clone.clone();
                    match &msg {
                        ui::ToExecutor::OpenSettings => {
                            tracing::info!("start screen share");
                            uvm_ui_messages.add(ui::ToUI::ShowSettings(ui::Settings {
                                server_url: ui::Setting::Value("http://localhost:9000".to_string()),
                                server_url_2: ui::Setting::NoValue,
                            }));
                        }
                        ui::ToExecutor::HidMainWindow => {
                            tracing::info!("hid main window");
                            // immediately show it again
                            uv_app_ctx.spawn(async move {
                                tracing::warn!("waiting a second to reshow main window");
                                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                                send_to_ui_clone.send_to_ui(ui::ToUI::ShowMainWindow);
                                Ok(())
                            });
                            uvm_ui_messages.add(ui::ToUI::ShowMainWindow);
                        }
                        other => {
                            tracing::warn!(?other, "unimplemented message");
                        }
                    }
                },
            ))
            .expect("send to executor");
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
