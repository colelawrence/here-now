use crate::prelude::*;

pub trait Command: Sized + Send + Sync + 'static {
    fn add(self, to: WorkloadBuilder) -> WorkloadBuilder;
}

impl Command for WorkloadSystem {
    fn add(self, to: WorkloadBuilder) -> WorkloadBuilder {
        to.with_system(self)
    }
}

// pub struct CommandsPlugin(pub tokio::sync::mpsc::UnboundedSender<Box<dyn Command>>);
pub struct CommandsPlugin(pub tokio::sync::mpsc::UnboundedSender<WorkloadSystem>);

#[derive(Component, Clone)]
pub struct SendCommands(tokio::sync::mpsc::UnboundedSender<WorkloadSystem>);

impl SendCommands {
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

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_unique(SendCommands(self.0.clone()));
    }
}
