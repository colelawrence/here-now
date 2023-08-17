use std::collections::BTreeSet;
use std::{sync::Arc, time::Duration};

use shipyard_app::App;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Mutex;

use super::AppCtx;
use crate::_ecs_::*;
use crate::_result_::*;
use crate::_tracing_::*;

pub async fn start_loop(
    app: App,
    workload: AppWorkload,
    mut recv: UnboundedReceiver<super::Command>,
) {
    let app: Arc<Mutex<App>> = Arc::new(Mutex::new(app));

    {
        let app_clone = app.clone();
        let app = app.lock().await;
        // re-insert app into world so it can be referenced
        app.run(|mut uvm_app_ctx: UniqueViewMut<AppCtx>| {
            uvm_app_ctx.as_mut().set_app(app_clone);
        });

        // initial kick off
        workload.run(&app);
    }

    let mut i = 0usize;
    while let Some(super::Command {
        reason,
        immediate,
        system,
        dedup,
    }) = recv.recv().await
    {
        i += 1;
        // async block so we can instrument with tracing
        async {
            if !immediate {
                // channel might continue growing?
                tokio::time::sleep(Duration::from_millis(17))
                    .instrument(info_span!("sleep to wait for additional commands"))
                    .await;
            }

            let mut seen = BTreeSet::<(String, &'static str)>::new();
            seen.extend(dedup.map(|s| (s, reason)));

            let (name, builder) = async {
                let name = format!("command-{i}");
                let mut builder = WorkloadBuilder::new(name.clone());
                builder = builder.with_system(system);

                while let Ok(super::Command {
                    reason,
                    immediate: _,
                    system,
                    dedup,
                }) = recv.try_recv()
                {
                    if let Some(dedup_str) = dedup {
                        let val = (dedup_str, reason);
                        if seen.contains(&val) {
                            debug!(i, reason, dedup = val.0, "skipping duplicate command");
                            continue;
                        }

                        seen.insert(val);
                    }

                    debug!(?i, ?reason, "adding command");
                    builder = builder.with_system(system);
                }

                (name, builder)
            }
            .instrument(info_span!("collect commands into workload"))
            .await;

            {
                let app = app
                    .lock()
                    .instrument(info_span!("lock app for commands"))
                    .await;
                async {
                    let info = builder.add_to_world(&app.world).expect("adding workload");
                    app.world
                        .run_workload(name)
                        .todo(f!("run collected commands workload {:?}", info));
                }
                .instrument(info_span!("run collected commands workload"))
                .await
            }

            {
                let app = app
                    .lock()
                    .instrument(info_span!("lock app for update loop"))
                    .await;
                info_span!("run update loop").in_scope(|| {
                    workload.run(&app);
                });
            }
        }
        .instrument(tracing::info_span!("running command", ?i, ?reason))
        .await
    }

    debug!(?i, "closed");
}
