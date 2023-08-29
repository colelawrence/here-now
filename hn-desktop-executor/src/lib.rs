use std::sync::Arc;

use shipyard::UniqueViewMut;
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
mod device_plugin;
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
    let main_plugin = device_plugin::DevicePlugin(sender.clone());
    let workload = app.add_plugin_workload(main_plugin);
    let send_to_ui = Arc::new(send_to_ui);
    let send_to_ui_clone = send_to_ui.clone();
    let main_loop =
        rt.spawn(hn_app::app_ctx::start_loop(
            app,
            workload,
            recv,
            move |app: &App| {
                app.world.run(
                    |mut uvm: shipyard::UniqueViewMut<device_plugin::Messages<ui::ToUI>>,
                     uv_unhandled: shipyard::UniqueViewMut<
                        device_plugin::Messages<ui::ToExecutor>,
                    >| {
                        tracing::info!("draining messages from executor: {:?}", uvm);
                        send_to_ui_clone.send_all_to_ui(uvm.drain().collect());
                        if uv_unhandled.as_ref().len() > 0 {
                            tracing::error!(to_executor = ?uv_unhandled, "unhandled messages from executor");
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

impl ui::SendToExecutor for Executor {
    #[tracing::instrument(skip(self))]
    fn send_all_to_executor(&self, msgs: Vec<ui::ToExecutor>) {
        eprintln!("send_all_to_executor:");
        self.run(
            move |mut uvm: UniqueViewMut<device_plugin::Messages<ui::ToExecutor>>| {
                for msg in msgs.clone() {
                    eprintln!(" - {msg:?}");
                    uvm.add(msg)
                }
            },
        );
    }
}
