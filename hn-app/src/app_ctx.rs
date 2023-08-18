use std::sync::Arc;

use anyhow::anyhow;
use bonsaidb::local;
use futures::Future;

use shipyard::{Component, IntoWorkloadSystem, UniqueView, WorkloadSystem};
use shipyard_app::{App, AppBuilder};
use tokio::sync::Mutex;

use crate::_ecs_::*;
use crate::_result_::*;
use crate::_tracing_::*;

pub type CommandSender = tokio::sync::mpsc::UnboundedSender<Command>;

pub use cmd_loop::start_loop;

mod cmd_loop;

pub struct Command {
    pub reason: &'static str,
    pub immediate: bool,
    pub dedup: Option<String>,
    pub system: WorkloadSystem,
    pub span: tracing::Span,
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
    db: Arc<Mutex<ArcResult<bonsaidb::local::Database>>>,
    commands: CommandSender,
    handle: tokio::runtime::Handle,
}

impl Command {
    /// TODO: Figure out how to allow for FnOnce
    ///
    /// Future: Create a run_system_with_data version:
    /// Allow for returning a Future with custom return value
    /// don't accept a future, but return the future so we ca wait on
    /// shipyard to become available and run the system.
    pub fn new<B, R, S>(
        reason: &'static str,
        immediate: bool,
        dedup: Option<String>,
        cmd: S,
    ) -> Self
    where
        S: IntoWorkloadSystem<B, R>,
    {
        Command {
            reason,
            immediate,
            dedup,
            system: cmd
                .into_workload_system()
                .todo(f!("expecting valid system")),
            span: debug_span!("create command", reason, immediate).or_current(),
        }
    }
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
            .send(Command::new(reason, false, None, cmd))
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
            .send(Command::new(reason, false, Some(dedup), cmd))
            .todo(f!("attempting to schedule"));
    }

    /// See [AppCtx::schedule_system]
    pub fn run_system<B, R, S>(&self, reason: &'static str, cmd: S)
    where
        S: IntoWorkloadSystem<B, R>,
    {
        self.commands
            .send(Command::new(reason, true, None, cmd))
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

    pub async fn get_database(&self) -> ArcResult<bonsaidb::local::Database> {
        self.db.lock().await.clone()
    }
}

impl Plugin for AppCtxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let handle =
            tokio::runtime::Handle::try_current().expect("expect a tokio runtime is ready");

        app.add_unique(AppCtx {
            app: Default::default(),
            // lovely...
            db: Arc::new(Mutex::new(Arc::new(Err(anyhow!("not ready yet"))))),
            commands: self.0.clone(),
            handle,
        });

        app.add_reset_system(
            keep_app_ctx_db_up_to_date,
            "update database reference when database is created",
        );
    }
}

fn keep_app_ctx_db_up_to_date(
    uv_database: UniqueView<LocalDatabase>,
    mut uvm_app_ctx: UniqueViewMut<AppCtx>,
) {
    if uv_database.is_inserted_or_modified() {
        let mut db_lock = uvm_app_ctx
            .as_mut()
            .db
            .try_lock()
            .expect("TODO: lucky lock?");
        *db_lock = uv_database.get_database();
    }
}

#[ecs_unique]
pub struct LocalDatabase(pub(crate) ArcResult<local::Database>);

impl LocalDatabase {
    pub fn get_database(&self) -> ArcResult<local::Database> {
        self.0.clone()
    }
}

impl AsRef<Result<local::Database>> for LocalDatabase {
    fn as_ref(&self) -> &Result<local::Database> {
        &self.0
    }
}
