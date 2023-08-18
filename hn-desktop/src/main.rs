use hn_app::_result_::*;
use slint::run_event_loop;

mod prelude {
    #![allow(unused)]
    pub use anyhow::Context;
    pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
    pub use hn_common::keys;
}

mod device;
mod device_client;
mod local_keys;
mod slint_main;

fn main() {
    let tokio_handle = std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let join = rt.spawn(main_tokio());

        rt.block_on(async {
            join.await.unwrap();
        });
    });

    run_event_loop().expect("able to enter slint event loop");
    tokio_handle.join().expect("tokio thread to join");
}

async fn main_tokio() {
    hn_app::logging::expect_init_logger("hn-desktop");

    // device::start().await;

    let mut app = shipyard_app::App::new();
    let (sender, recv) = tokio::sync::mpsc::unbounded_channel();
    let main_plugin = device_plugin::DevicePlugin(sender);
    let workload = app.add_plugin_workload(main_plugin);
    let main_loop = tokio::spawn(hn_app::app_ctx::start_loop(app, workload, recv));

    let slint = tokio::task::spawn_blocking(slint_main::slint_main);
    // // must be launched on the main thread for winit to work on macOS
    // slint::run_event_loop().expect("able to enter slint event loop");

    // must await or the nested jobs get canceled with an opaque "background task failed" error.
    main_loop.await.todo(f!("desktop loop exit error"));
    slint.await.todo(f!("slint exit error"));
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
            builder.add_plugin(AppCtxPlugin(self.0.clone()));
            builder.add_plugin(LocalDatabasePlugin::<DBSchema> {
                path: PathBuf::from("./data/desktop-db.bonsaidb"),
                mark: PhantomData,
            });
        }
    }
}
