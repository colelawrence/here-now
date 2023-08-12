use here_now_common::public;

use crate::{device_client::DeviceClient, local_keys};

pub(super) async fn start() {
    let client = DeviceClient::new(
        local_keys::get_keys().expect("get keys for app"),
        "http://0.0.0.0:9000".to_string(),
    );

    dbg!(client
        .send(public::Mutate::Ping)
        .await
        .expect("pinged server"));
}
