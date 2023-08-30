use axum::{
    extract::{FromRequest, FromRequestParts},
    response::IntoResponse,
    Extension,
};

use http::StatusCode;
use serde::de::DeserializeOwned;

use crate::prelude::*;

pub(super) struct Verified<T>(pub hn_keys::net::VerifiedMessage<T>);

pub(super) enum VerifiedRejection {
    InternalError,
    BodyError(axum::extract::rejection::BytesRejection),
    DeserializeError(anyhow::Error),
    BadSignature(anyhow::Error),
    // MissingVerifiedMessageContentType,
}

impl IntoResponse for VerifiedRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            VerifiedRejection::BadSignature(err) => {
                (StatusCode::UNAUTHORIZED, format!("Bad Signature: {err:?}")).into_response()
            }
            VerifiedRejection::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            VerifiedRejection::BodyError(err) => err.into_response(),
            VerifiedRejection::DeserializeError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Bad Request, failed to parse body: {err:?}"),
            )
                .into_response(),
        }
    }
}

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Verified<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<axum::BoxError>,
{
    type Rejection = VerifiedRejection;

    #[instrument(skip_all, name = "Verified::from_request")]
    async fn from_request(req: http::Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let local_keys = Extension::<hn_keys::LocalKeys>::from_request_parts(&mut parts, state)
            .await
            .map_err(|err| {
                error!(?err, "failed to get local keys from request");
                VerifiedRejection::InternalError
            })?;

        // http::Request::<B>::from_parts(parts, body) is a bit much, right?
        let bytes =
            axum::body::Bytes::from_request(http::Request::<B>::from_parts(parts, body), state)
                .await
                .map_err(|e| VerifiedRejection::BodyError(e))?;

        let wire_msg = hn_keys::net::WireMessage::from_bytes(&bytes)
            .map_err(|e| VerifiedRejection::DeserializeError(e))?;
        Ok(Verified(
            local_keys
                .recv::<T>(&wire_msg)
                .map_err(|e| VerifiedRejection::BadSignature(e))?,
        ))
    }
}
