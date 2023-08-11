use std::sync::Arc;

use anyhow::Context;
use here_now_common::{keys, public};
use tokio::sync::Mutex;

mod prelude {
    pub use anyhow::Context;
    pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
    pub use here_now_common::keys;
}

mod local_keys;

#[tokio::main]
async fn main() {
    let client = DeviceClient {
        client: reqwest::Client::new(),
        local_keys: local_keys::get_keys().expect("get keys for app"),
        server_base_url: "http://0.0.0.0:9000".to_string(),
        server_pk: Arc::new(Mutex::new(None)),
    };

    client
        .send(public::Mutate {
            messages: vec![public::Message::Ping],
        })
        .await
        .expect("pinged server");

    // let res = client
    //     .post("http://0.0.0.0:9000/_create_device")
    //     .json(&public::CreateDevicePayload {
    //         auth_key: local_keys.public_key().clone(),
    //         label: Some("my device".to_string()),
    //     })
    //     .send()
    //     .await
    //     .unwrap();
}

#[derive(Debug)]
struct DeviceClient {
    client: reqwest::Client,
    local_keys: keys::LocalKeys,
    server_base_url: String,
    server_pk: Arc<Mutex<Option<keys::PublicKeyKind>>>,
}

impl DeviceClient {
    #[tracing::instrument]
    async fn get_server_key(&self) -> prelude::Result<keys::PublicKeyKind> {
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
    async fn send(&self, msg: public::Mutate) -> prelude::Result {
        let server_key = self
            .get_server_key()
            .await
            .context("need key for sending messages")?;

        let res = self
            .client
            .post(format!("{}/_mutate", self.server_base_url))
            .body(
                self.local_keys
                    .send(&msg, &server_key)
                    .context("for body to send")?
                    .to_bytes(),
            )
            .send()
            .await
            .unwrap();

        Ok(())
    }
}
mod device {
    pub(super) async fn start() {}
}
