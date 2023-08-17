use hn_app::_result_::*;

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

#[tokio::main]
async fn main() {
    hn_app::logging::expect_init_logger("hn-desktop");

    // device::start().await;

    let mut app = shipyard_app::App::new();
    let (sender, recv) = tokio::sync::mpsc::unbounded_channel();
    let main_plugin = device_plugin::DevicePlugin(sender);
    let workload = app.add_plugin_workload(main_plugin);
    let main_loop = tokio::spawn(hn_app::app_ctx::start_loop(app, workload, recv));

    // must be launched on the main thread for winit to work on macOS
    slint_main::slint_main();

    // must await or the nested jobs get canceled with an opaque "background task failed" error.
    main_loop.await.todo(f!("desktop loop exit error"));
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
