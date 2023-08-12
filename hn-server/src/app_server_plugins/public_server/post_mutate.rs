use anyhow::Result;
use async_trait::async_trait;
use here_now_common::{keys, public};

use crate::app_ctx::AppCtx;

type MutateResult<T> = Result<<T as public::Mutation>::Success, public::MutateRejection>;

#[async_trait]
pub trait Mutation: public::Mutation {
    async fn mutate(&self, sender: &keys::PublicKeyKind, app_ctx: AppCtx) -> MutateResult<Self>;
}

mod create_device;
