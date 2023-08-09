//! Data stored to disk

use std::marker::PhantomData;

use crate::prelude::ecs_::*;
use crate::prelude::*;

pub mod import_export;
pub use import_export::plugin::SavePlugin;

pub mod hinted_id;
pub use hinted_id::HintedID;

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
