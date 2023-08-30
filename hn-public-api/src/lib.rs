use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// POST `/_mutate` endpoint
/// See [MutateResponse]
#[derive(Debug, Serialize, Deserialize)]
pub enum Mutate {
    Ping(Ping),
    CreateDevice(create_device::CreateDeviceMutation),
}

pub use create_device::{CreateDeviceMutation, CreateDeviceResponse};

pub type MutateResult<M> = Result<<M as Mutation>::Success, MutateRejection>;

mod create_device {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateDeviceMutation {
        pub label: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateDeviceResponse {
        pub device_id: String,
    }

    impl Mutation for CreateDeviceMutation {
        type Success = CreateDeviceResponse;
        fn into_request(self) -> Mutate {
            Mutate::CreateDevice(self)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MutateRejection {
    InternalError(String),
    BadRequest(String),
    Unauthorized(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pong;

impl Mutation for Ping {
    type Success = Pong;
    fn into_request(self) -> Mutate {
        Mutate::Ping(self)
    }
}

pub trait Mutation: std::fmt::Debug + Serialize + DeserializeOwned {
    type Success: Serialize + DeserializeOwned;
    fn into_request(self) -> Mutate;
}
