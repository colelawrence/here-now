use std::marker::PhantomData;

use super::HintedID;
use crate::prelude::bonsai_::*;
use crate::prelude::ecs_::*;
use crate::prelude::*;

#[derive(schema::Schema)]
#[schema(name = "DBSchema", collections = [CredBundle, DeviceBundle])]
pub struct DBSchema;

pub mod export;
pub mod plugin;

/// Stored in BonsaiDB
#[derive(schema::Collection)]
#[collection(name = "ecs-devices", primary_key = HintedID)]
// Bundled in the ECS
#[ecs_bundle(DeviceTag)]
#[derive(Debug)]
pub struct DeviceBundle {
    // what is the component for this again?
    c_linked_creds: LinkedBundle<ecs::CredTag>,
}
#[ecs_bundle]
#[derive(Debug, Default)]
pub struct LinkedBundle<Tag: 'static> {
    items: Vec<HintedID>,
    _mark: PhantomData<Tag>,
}

/// Stored in BonsaiDB
#[derive(schema::Collection)]
#[collection(name = "ecs-creds", primary_key = HintedID)]
// Bundled in the ECS
#[ecs_bundle(CredTag)]
#[derive(Debug)]
pub struct CredBundle {
    kind: CredBundleKind,
}
#[ecs_bundle(CredTag)]
#[derive(Debug)]
pub enum CredBundleKind {
    Discord { c_discord_cred: ecs::EcsDiscordCred },
}
