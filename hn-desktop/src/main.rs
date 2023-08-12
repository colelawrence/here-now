mod prelude {
    pub use anyhow::Context;
    pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
    pub use here_now_common::keys;
}

mod device;
mod device_client;
mod local_keys;

#[tokio::main]
async fn main() {
    device::start().await;
}
