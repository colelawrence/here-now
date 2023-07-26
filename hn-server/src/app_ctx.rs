use crate::prelude::*;

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

pub trait Command: Sized + Send + Sync + 'static {
    fn add(self, to: WorkloadBuilder) -> WorkloadBuilder;
}

impl Command for WorkloadSystem {
    fn add(self, to: WorkloadBuilder) -> WorkloadBuilder {
        to.with_system(self)
    }
}

// pub struct CommandsPlugin(pub tokio::sync::mpsc::UnboundedSender<Box<dyn Command>>);
pub struct AppCtxPlugin(pub tokio::sync::mpsc::UnboundedSender<WorkloadSystem>);

#[derive(Component, Clone)]
pub struct AppCtx(tokio::sync::mpsc::UnboundedSender<WorkloadSystem>);

impl AppCtx {
    /// TODO: Figure out how to allow for FnOnce
    pub fn schedule_system<B, R, S>(&self, cmd: S)
    where
        S: IntoWorkloadSystem<B, R>,
    {
        self.0
            .send(
                cmd.into_workload_system()
                    .todo(f!("expecting valid system")),
            )
            .todo(f!("attempting to schedule"));
    }
}

impl Plugin for AppCtxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_unique(AppCtx(self.0.clone()));
    }
}
