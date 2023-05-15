// use livekit::

use std::{
    array, env,
    future::Future,
    process,
    time::{Duration, Instant},
};

pub struct HereNowServer {
    pub(crate) livekit_url: String,
    pub(crate) livekit_key: String,
    pub(crate) livekit_secret: String,
}

impl HereNowServer {
    pub fn new(livekit_url: String, livekit_key: String, livekit_secret: String) -> Self {
        Self {
            livekit_url,
            livekit_key,
            livekit_secret,
        }
    }
    pub fn expect_from_env() -> Self {
        let [url, key, secret] = expect_envs(["LIVEKIT_URL", "LIVEKIT_KEY", "LIVEKIT_SECRET"]);
        Self {
            livekit_url: url,
            livekit_key: key,
            livekit_secret: secret,
        }
    }
}

#[track_caller]
fn expect_envs<const T: usize>(names: [&str; T]) -> [String; T] {
    let mut errors = Vec::<String>::new();
    let valid = names.map(|key| match std::env::var(key) {
        Err(err) => {
            errors.push(format!("{key:?} was {err:?}"));
            Default::default()
        }
        Ok(val) => val,
    });
    if errors.len() > 0 {
        panic!(
            "Error loading values from environment variables:\n- {}",
            errors.join("\n- ")
        );
    }
    valid
}

#[derive(Debug, Clone)]
pub struct RoomPermissions {
    pub can_publish_tracks: bool,
    pub can_subscribe_to_tracks: bool,
    pub can_publish_to_data_channel: bool,
    pub is_invisible: bool,
}

#[derive(Debug)]
pub struct RoomKey {
    key: String,
    for_room: String,
    for_username: String,
    expires_at: Instant,
    permissions: RoomPermissions,
}

impl RoomKey {
    pub fn expires_at(&self) -> Instant {
        self.expires_at
    }

    pub fn for_username(&self) -> &str {
        self.for_username.as_ref()
    }

    pub fn for_room(&self) -> &str {
        self.for_room.as_ref()
    }

    pub fn permissions(&self) -> &RoomPermissions {
        &self.permissions
    }
}

// type Fut<T> = Box<dyn Future<Output = T>>;
impl HereNowServer {
    pub async fn get_room_key(
        &self,
        username: &str,
        roomname: &str,
        expire_after: Duration,
        permissions: RoomPermissions
    ) -> anyhow::Result<RoomKey> {
        Ok(RoomKey {
            key: "TODO".to_string(),
            for_room: roomname.to_string(),
            for_username: username.to_string(),
            expires_at: Instant::now()
                .checked_add(expire_after)
                .expect("valid duration"),
            permissions,
        })
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn distributes_room_keys() {
        let server = HereNowServer::expect_from_env();
        let room_key = server
            .get_room_key("cole", "opensource", Duration::from_secs(60 * 60), super::RoomPermissions {
                can_publish_to_data_channel: true,
                can_publish_tracks: true,
                can_subscribe_to_tracks: true,
                is_invisible: false,
            })
            .await
            .expect("successful exchange for room key");

        assert_eq!(room_key.for_username(), "cole");
        assert_eq!(room_key.for_room(), "opensource");
        assert_eq!(room_key.permissions().can_publish_to_data_channel, true);
        assert_eq!(room_key.permissions().can_publish_tracks, true);
        assert_eq!(room_key.permissions().can_subscribe_to_tracks, true);
        assert_eq!(room_key.permissions().is_invisible, false);
        assert!(Instant::now() < room_key.expires_at());
    }
}
