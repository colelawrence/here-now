use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// POST `/_mutate` endpoint
/// See [MutateResponse]
#[derive(Debug, Serialize, Deserialize)]
pub enum ToServer {
    Ping(Ping),
    CreateDevice(create_device::CreateDevice),
    Device(create_device::CreateDevice),
}

pub use create_device::{CreateDevice, CreateDeviceResponse};

pub type ServerResult<M> = Result<<M as Mutation>::Success, ServerRejection>;

mod device {

    use hn_usr::{UsrLink, UsrText};

    use super::*;

    /// Marker for [LocalID] usage.
    #[derive(Debug)]
    pub struct Profile;
    /// Marker for [ServerID] usage.
    #[derive(Debug)]
    pub struct Room;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum MeToServer {
        Heartbeat {
            // /// Like "mobile" | "desktop" ?
            // device_type: String
            // rooms: Vec<ServerID<Room>>,
        },
        SetStatus {
            text: UsrText,
            // Maybe should be an "instant" for simplicity?
            expires_in: std::time::Duration,
            // rooms: Vec<ServerID<Room>>,
        },
        SetDoNotDisturb {
            expires_in: std::time::Duration,
            // rooms: Vec<ServerID<Room>>,
        },
        /// These call destination preferences are in addition to the server/room defaults.
        SetCallChoices {
            options: Vec<CallChoice>,
            // rooms: Vec<ServerID<Room>>,
        },
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum InteractToServer {
        WaveAt { to: LocalID<Profile> },
        TalkRequest { to: LocalID<Profile> },
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LocalID<Resource> {
        pub lid: usize,
        _phantom: std::marker::PhantomData<Resource>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerID<Resource> {
        pub sid: hn_hinted_id::HintedID,
        _phantom: std::marker::PhantomData<Resource>,
    }

    /// This is the place that the profile prefers to receive calls via.
    /// Future: Perhaps this could be influenced by the device they heartbeat from
    /// e.g. if you're on mobile, then your preference could be a telephone call...
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CallChoice {
        /// e.g. "Join me in Figma", "Call me on FaceTime", "Call me on Discord", "Join the Work Discord"
        label: UsrText,
        /// e.g. a scheduling page, a Discord profile, Discord Voice Channel, or personal meeting room.
        link: Option<UsrLink>,
        /// Hmmm...
        mvp_icon: Option<String>,
    }
}

mod create_device {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateDevice {
        pub label: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateDeviceResponse {
        pub device_id: String,
    }

    impl Mutation for CreateDevice {
        type Success = CreateDeviceResponse;
        fn into_request(self) -> ToServer {
            ToServer::CreateDevice(self)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerRejection {
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
    fn into_request(self) -> ToServer {
        ToServer::Ping(self)
    }
}

pub trait Mutation: std::fmt::Debug + Serialize + DeserializeOwned {
    type Success: Serialize + DeserializeOwned;
    fn into_request(self) -> ToServer;
}
