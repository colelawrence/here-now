use futures::Future;

use tokio::sync::Mutex;

use crate::prelude::*;

pub type CommandSender = tokio::sync::mpsc::UnboundedSender<Command>;

pub struct Command {
    pub reason: &'static str,
    pub dedup: Option<String>,
    pub system: WorkloadSystem,
}

pub trait AppSenderExt {
    fn ctx(&self) -> AppCtx;
}

impl AppSenderExt for AppBuilder<'_> {
    fn ctx(&self) -> AppCtx {
        self.app
            .world
            .run(|uv_sender: UniqueView<AppCtx>| uv_sender.clone())
    }
}

pub struct AppCtxPlugin(pub CommandSender);

#[derive(Component, Clone)]
pub struct AppCtx {
    app: Option<Arc<Mutex<App>>>,
    commands: CommandSender,
    handle: tokio::runtime::Handle,
}

impl AppCtx {
    #[allow(unused)]
    /// TODO: Figure out how to allow for FnOnce
    ///
    /// Future: Create a run_system_with_data version:
    /// Allow for returning a Future with custom return value
    /// don't accept a future, but return the future so we ca wait on
    /// shipyard to become available and run the system.
    pub fn schedule_system<B, R, S>(&self, reason: &'static str, cmd: S)
    where
        S: IntoWorkloadSystem<B, R>,
    {
        self.commands
            .send(Command {
                reason,
                dedup: None,
                system: cmd
                    .into_workload_system()
                    .todo(f!("expecting valid system")),
            })
            .todo(f!("attempting to schedule"));
    }

    pub async fn get_unique<U: Component + Clone + Send + Sync>(&self, reason: &'static str) -> U {
        let all_access = self
            .app
            .as_ref()
            .context(reason)
            .expect("app mutex set")
            .lock()
            .await;
        let b = all_access
            .world
            .borrow::<UniqueView<U>>()
            .expect("unique access");
        let ret = b.as_ref();
        ret.clone()
    }

    /// See [AppCtx::schedule_system]
    pub fn schedule_system_dedup<B, R, S>(&self, reason: &'static str, dedup: String, cmd: S)
    where
        S: IntoWorkloadSystem<B, R>,
    {
        self.commands
            .send(Command {
                reason,
                dedup: Some(dedup),
                system: cmd
                    .into_workload_system()
                    .todo(f!("expecting valid system")),
            })
            .todo(f!("attempting to schedule"));
    }
    /// TODO: hook the result error into sending a command?
    #[track_caller]
    pub fn spawn<F>(&self, fut: F)
    where
        F: Send + Future<Output = Result<()>> + 'static,
    {
        self.handle.spawn(async {
            match fut.await {
                Ok(()) => {}
                Err(err) => error!(?err, "error for spawned future"),
            }
        });
    }

    pub fn set_app(&mut self, app: Arc<Mutex<App>>) {
        self.app = Some(app);
    }
}

impl Plugin for AppCtxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let handle =
            tokio::runtime::Handle::try_current().expect("expect a tokio runtime is ready");

        app.add_unique(AppCtx {
            app: Default::default(),
            commands: self.0.clone(),
            handle,
        });
    }
}
