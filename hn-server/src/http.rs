use anyhow::Result;
use axum::response::Html;
pub use http::StatusCode;
pub trait OrInternalError<T> {
    fn err_500(self) -> Result<T, (StatusCode, String)>;
    fn err_400(self) -> Result<T, (StatusCode, String)>;
}

impl<T> OrInternalError<T> for Result<T> {
    fn err_500(self) -> Result<T, (StatusCode, String)> {
        self.map_err(|err| {
            // redundant?
            tracing::error!(err=?err, "internal service error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                // in dev mode show dev errors
                format!("INTERNAL ERROR:\n{}", err),
            )
        })
    }
    fn err_400(self) -> Result<T, (StatusCode, String)> {
        self.map_err(|err| (StatusCode::BAD_REQUEST, format!("BAD REQUEST:\n{}", err)))
    }
}

pub type Response = Result<Html<String>, (StatusCode, String)>;
