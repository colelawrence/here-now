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
mod device_plugin;

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
}

use device_plugin::ExecutorAction;

impl ui::SendToExecutor for Executor {
    #[tracing::instrument(skip(self))]
    fn send_to_executor(&self, msg: ui::ToExecutor) {
        match msg {
            ui::ToExecutor::OpenSettings => {
                self.run(device_plugin::open_settings);
            }
            ui::ToExecutor::HidMainWindow => {
                tracing::info!("hid main window");
                let send_to_ui_clone = self.send_to_ui.clone();
                std::thread::spawn(move || {
                    // experiment to show we can re-show the main window
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    tracing::warn!("waited a second to reshow main window");
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
                settings.execute(self);
            }
            ui::ToExecutor::AddServerByURL(add_server_by_url) => {
                add_server_by_url.execute(self);
            }
        }
    }
}
