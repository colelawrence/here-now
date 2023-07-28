use futures::Future;

use crate::prelude::*;

pub type CommandSender = tokio::sync::mpsc::UnboundedSender<Command>;

pub struct Command {
    pub reason: &'static str,
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
    commands: CommandSender,
}

impl AppCtx {
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
                system: cmd
                    .into_workload_system()
                    .todo(f!("expecting valid system")),
            })
            .todo(f!("attempting to schedule"));
    }
    /// TODO: hook the result error into sending a command?
    pub fn spawn<F>(&self, fut: F)
    where
        F: Send + Future<Output = Result<()>> + 'static,
    {
        tokio::spawn(async {
            match fut.await {
                Ok(()) => {}
                Err(err) => error!(?err, "error for spawned future"),
            }
        });
    }
}

impl Plugin for AppCtxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_unique(AppCtx {
            commands: self.0.clone(),
        });
    }
}
