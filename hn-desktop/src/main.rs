use here_now_common::{keys, public};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let (private_key, public_key) = keys::init();
    let res = client
        .post("http://0.0.0.0:9000/_create_device")
        .json(&public::CreateDevicePayload {
            auth_key: public_key,
            label: Some("my device".to_string()),
        })
        .send()
        .await
        .unwrap();

    println!("{:?}", res);
    
    let device_id = res.json::<String>().await.unwrap();
    
    println!("device_id = {device_id:?}");
}
