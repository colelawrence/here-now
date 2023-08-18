use hn_common::public;

use crate::{device_client::DeviceClient, local_keys};

pub(super) async fn _start() {
    let client = DeviceClient::new(
        local_keys::get_keys().expect("get keys for app"),
        "http://0.0.0.0:9000".to_string(),
    );

    let resp = client.send(public::Ping).await.expect("pinged server");

    let _ = dbg!(resp);

    let resp = client
        .send(public::CreateDeviceMutation {
            label: "desktop".to_string(),
        })
        .await
        .expect("created device on server");

    let _ = dbg!(resp);
}
