use anyhow::Result;
pub use http::StatusCode;
pub trait OrInternalError<T> {
    fn err_500(self) -> Result<T, (StatusCode, String)>;
    fn err_400(self) -> Result<T, (StatusCode, String)>;
}

impl<T, E: std::fmt::Debug + std::fmt::Display> OrInternalError<T> for Result<T, E> {
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
