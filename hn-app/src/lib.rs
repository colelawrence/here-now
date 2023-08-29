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

pub use hn_hinted_id::HintedID;
pub use i_hn_app_proc::ecs_bundle;

pub mod _ecs_ {
    //! ECS prelude
    pub use crate::app_ctx::AppSenderExt as _;
    pub use i_hn_app_proc::{ecs_component, ecs_unique};
    pub use shipyard_app::prelude::*;

    #[derive(Clone)]
    pub struct SetupError {
        label: String,
        body: Option<String>,
        debug: Option<String>,
        // // some kind of tags and values which can be matched
        // // to create mitigations ?
        // tags: Vec<String>,
        // vals: Vec<(String, String)>,
    }

    impl std::fmt::Display for SetupError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.label)?;
            if let Some(body) = &self.body {
                write!(f, ":\n • {}", body)?;
            }
            Ok(())
        }
    }

    impl std::fmt::Debug for SetupError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.label)?;
            if let Some(debug) = &self.debug {
                write!(f, ":\n • {}", debug)?;
            }
            Ok(())
        }
    }

    impl std::error::Error for SetupError {}

    pub trait SetupResultExt<T> {
        fn as_setup_err<L: std::fmt::Display>(self, label: L) -> SetupResult<T>;
    }

    impl<T, E: std::fmt::Display + std::fmt::Debug> SetupResultExt<T> for Result<T, E> {
        fn as_setup_err<L: std::fmt::Display>(self, label: L) -> SetupResult<T> {
            self.map_err(|e| SetupError {
                label: format!("{}", label),
                body: Some(format!("{}", e)),
                debug: Some(format!("{:?}", e)),
            })
        }
    }

    impl<T> SetupResultExt<T> for Option<T> {
        fn as_setup_err<L: std::fmt::Display>(self, label: L) -> SetupResult<T> {
            self.ok_or_else(|| SetupError {
                label: format!("{}", label),
                body: None,
                debug: None,
            })
        }
    }

    pub type SetupResult<T> = Result<T, SetupError>;

    pub use std::format_args as f;

    pub trait TupleRefsExt<'a> {
        type Out;
        fn as_tuple_refs(&'a self) -> Self::Out;
    }

    impl TupleRefsExt<'_> for () {
        type Out = ();
        fn as_tuple_refs(&self) -> Self::Out {}
    }
    impl<'a, A1: 'a> TupleRefsExt<'a> for (A1,) {
        type Out = (&'a A1,);
        fn as_tuple_refs(&'a self) -> Self::Out {
            (&self.0,)
        }
    }
    impl<'a, A1: 'a, A2: 'a> TupleRefsExt<'a> for (A1, A2) {
        type Out = (&'a A1, &'a A2);
        fn as_tuple_refs(&'a self) -> Self::Out {
            (&self.0, &self.1)
        }
    }
    impl<'a, A1: 'a, A2: 'a, A3: 'a> TupleRefsExt<'a> for (A1, A2, A3) {
        type Out = (&'a A1, &'a A2, &'a A3);
        fn as_tuple_refs(&'a self) -> Self::Out {
            (&self.0, &self.1, &self.2)
        }
    }
    impl<'a, A1: 'a, A2: 'a, A3: 'a, A4: 'a> TupleRefsExt<'a> for (A1, A2, A3, A4) {
        type Out = (&'a A1, &'a A2, &'a A3, &'a A4);
        fn as_tuple_refs(&'a self) -> Self::Out {
            (&self.0, &self.1, &self.2, &self.3)
        }
    }
}

pub mod app_ctx;
pub mod database_plugin;
pub mod logging;
