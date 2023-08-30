//! Data stored to disk

use std::marker::PhantomData;

use crate::prelude::*;
use hn_app::{_ecs_::*, ecs_bundle};

pub mod import_export;
pub use import_export::plugin::SavePlugin;

pub use hn_app::HintedID;

#[ecs_component("Device")]
#[derive(Debug, Clone)]
pub struct DeviceTag;

#[ecs_component("Cred")]
#[derive(Debug, Clone)]
pub enum CredTag {
    Discord,
}

#[ecs_component("Device")]
pub struct Linked<Tag: 'static> {
    pub items: Vec<EntityId>,
    _mark: PhantomData<Tag>,
}

#[ecs_component("Device")]
#[ecs_bundle(DeviceTag)]
#[derive(Debug, Default)]
pub struct AuthorizedKeys {
    pub keys: Vec<AuthorizedKey>,
}

#[ecs_bundle(DeviceTag)]
#[derive(Debug)]
pub struct AuthorizedKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_info: Option<String>,
    pub key: hn_keys::PublicKeyKind,
}

impl<Tag: 'static> Linked<Tag> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            _mark: PhantomData,
        }
    }
    pub fn new_with<I>(entity_id: I) -> Self
    where
        I: IntoIterator<Item = EntityId>,
    {
        Self {
            items: entity_id.into_iter().collect(),
            _mark: PhantomData,
        }
    }
}

#[ecs_bundle(CredTag)]
#[ecs_component("Cred")]
#[derive(Debug)]
pub struct EcsDiscordCred {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: std::time::SystemTime,
}
