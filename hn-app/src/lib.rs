pub mod _tracing_ {
    pub use tracing::{
        debug, debug_span, error, error_span, info, info_span, instrument, warn, warn_span,
        Instrument,
    };
}

pub mod _result_ {
    pub use anyhow::{Context as AnyhowContext, Error, Result};
    use std::{
        fmt::{Debug, Display},
        sync::Arc,
    };

    pub trait ResultExt<T, E> {
        /// Use when you're not sure if we need to unwrap or ignore the error
        /// ```ignore
        /// // for example
        /// .todo(f!("configuring watcher (dur: {:?})", self.polling_duration));
        /// ```
        fn todo<'a>(self, f: std::fmt::Arguments<'a>) -> T;
    }

    pub use std::format_args as f;

    impl<T, E> ResultExt<T, E> for Result<T, E>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        #[track_caller]
        fn todo<'a>(self, f: std::fmt::Arguments<'a>) -> T {
            self.with_context(|| format!("{}", f))
                .expect("todo: handle error")
        }
    }

    pub trait AsErrArcRefExt<T, E> {
        /// Use when you need to send an owned error around
        /// ```ignore
        /// // for example
        /// .todo(f!("configuring watcher (dur: {:?})", self.polling_duration));
        /// ```
        fn as_err_arc_ref(&self) -> Result<&T, Error>;
    }

    impl<T: 'static, E: Debug + Display + Send + Sync + 'static> AsErrArcRefExt<T, E>
        for Arc<Result<T, E>>
    {
        fn as_err_arc_ref(&self) -> Result<&T, Error> {
            let arc = self.clone();
            match self.as_ref() {
                Ok(val) => Ok(val),
                Err(_) => Err(Error::new(ArcError(arc))),
            }
        }
    }

    #[derive(Clone)]
    struct ArcError<T, E>(pub Arc<Result<T, E>>);

    pub type ArcResult<T, E = Error> = Arc<Result<T, E>>;

    unsafe impl<E: Sync, T> Sync for ArcError<T, E> {}
    unsafe impl<E: Send, T> Send for ArcError<T, E> {}

    impl<T, E: Debug + Display> std::error::Error for ArcError<T, E> {}

    impl<T, E: Display> Display for ArcError<T, E> {
        fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0.as_ref() {
                Ok(_) => unreachable!(),
                Err(err) => write!(&mut f, "{err}"),
            }
        }
    }

    impl<T, E: Debug> Debug for ArcError<T, E> {
        fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0.as_ref() {
                Ok(_) => unreachable!(),
                Err(err) => Debug::fmt(err, &mut f),
            }
        }
    }
}

pub mod _ecs_ {
    //! ECS prelude
    pub use crate::app_ctx::AppSenderExt as _;
    pub use i_hn_app_proc::{ecs_bundle, ecs_component, ecs_unique};
    pub use shipyard_app::prelude::*;
}

pub mod app_ctx;
pub mod database_plugin {
    use std::collections::HashSet;
    use std::marker::PhantomData;
    use std::path::PathBuf;
    use std::sync::Arc;

    use bonsaidb::core::schema::Schema;
    use bonsaidb::local;

    use crate::_ecs_::*;
    use crate::_result_::*;
    use crate::app_ctx::LocalDatabase;

    #[ecs_unique]
    #[derive(Default)]
    pub struct LastImport(pub HashSet<EntityId>);

    impl LastImport {
        /// For use to skip exporting entities that were changed as a result of insert from disk
        pub fn skip_once(&mut self, entity_id: EntityId) -> bool {
            self.0.remove(&entity_id)
        }
    }

    pub struct LocalDatabasePlugin<DB> {
        // get from configuration in the future?
        pub path: PathBuf,
        pub mark: PhantomData<DB>,
    }

    impl<DB: Schema> Plugin for LocalDatabasePlugin<DB> {
        fn build(&self, app: &mut AppBuilder) {
            let _span = tracing::info_span!("LocalDatabase::build").entered();
            let db: ArcResult<local::Database> = {
                let mut storage_conf = local::config::StorageConfiguration::default();
                storage_conf.path = Some(self.path.clone());
                Arc::new(local::Database::open::<DB>(storage_conf).context("creating database"))
            };
            app.add_unique(LocalDatabase(db));
            app.add_unique(LastImport::default());
        }
    }
}
pub mod logging;
