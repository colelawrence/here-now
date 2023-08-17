use async_trait::async_trait;
use hn_common::{keys, public};

use crate::app_ctx::AppCtx;

mod create_device;

/// A mutation is a request to change the state of the server.
/// This is usually a verified request from a client `POST` to the `/_mutate` public endpoint.
#[async_trait]
pub trait Mutation: public::Mutation {
    async fn mutate(
        &self,
        sender: &keys::PublicKeyKind,
        app_ctx: AppCtx,
    ) -> public::MutateResult<Self>;
}
