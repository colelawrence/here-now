use crate::prelude::*;
use here_now_common::{
    keys::{self, net::WireMessage},
    public,
};

#[derive(Debug)]
pub struct DeviceClient {
    client: reqwest::Client,
    local_keys: keys::LocalKeys,
    server_base_url: String,
    // server_pk: Arc<Mutex<Option<keys::PublicKeyKind>>>,
}

impl DeviceClient {
    pub fn new(local_keys: keys::LocalKeys, server_base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            local_keys,
            server_base_url,
        }
    }

    #[tracing::instrument]
    async fn get_server_key(&self) -> Result<keys::PublicKeyKind> {
        self.client
            .get(format!("{}/_public_key", self.server_base_url))
            .send()
            .await
            .context("get server public key endpoint")?
            .json::<keys::PublicKeyKind>()
            .await
            .context("parse server public key")
    }

    #[tracing::instrument]
    pub async fn send<M: public::Mutation>(&self, msg: M) -> Result<public::MutateResult<M>> {
        let server_key = self
            .get_server_key()
            .await
            .context("need key for sending messages")?;

        let res = self
            .client
            .post(format!("{}/_mutate", self.server_base_url))
            .body(
                self.local_keys
                    .send::<&public::Mutate>(&msg.into_request(), &server_key)
                    .context("for body to send")?
                    .to_bytes(),
            )
            .send()
            .await
            .unwrap();

        let wire =
            WireMessage::from_bytes(&res.bytes().await.context("read wire bytes from response")?)
                .context("parse wire message")?;

        let verified = self
            .local_keys
            .recv::<public::MutateResult<M>>(&wire)
            .context("reading and parsing mutate response")?;

        if let Err(err) = expect_serde_eq(verified.sender(), &server_key) {
            return Err(err.context("sender key mismatch"));
        }

        Ok(verified.into_data())
    }
}

fn expect_serde_eq<S: serde::Serialize>(a: &S, b: &S) -> Result {
    let left = serde_json::to_value(a).unwrap();
    let right = serde_json::to_value(b).unwrap();
    if left != right {
        return Err(anyhow::anyhow!("{left:?} != {right:?}"));
    }

    return Ok(());
}
