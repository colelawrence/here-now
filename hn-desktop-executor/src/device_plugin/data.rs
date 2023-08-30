use std::marker::PhantomData;

use bonsaidb::core::schema;
use hn_app::{ecs_bundle, HintedID};

/// Stored in BonsaiDB
#[derive(Debug, schema::Collection)]
#[collection(name = "client-profiles", primary_key = HintedID)]
/// Bundled in the ECS
#[ecs_bundle(ProfileTag)]
pub struct ProfileBundle {
    /// Maps to [super::ecs::UserLabel]
    pub c_label: Option<String>,
    /// Maps to [super::ecs::ProfileKeys]
    pub c_keys: hn_keys::LocalKeys,
}

#[ecs_bundle]
#[derive(Debug)]
pub struct BundleRef<Bundle: 'static> {
    pub id: HintedID,
    #[serde(skip_serializing, default)]
    _mark: PhantomData<Bundle>,
}

impl<Bundle: 'static> BundleRef<Bundle> {
    pub fn new(id: HintedID) -> Self {
        Self {
            id,
            _mark: PhantomData,
        }
    }
}

/// Stored in BonsaiDB
#[derive(Debug, schema::Collection)]
#[collection(name = "client-servers", primary_key = HintedID)]
/// Bundled in the ECS
#[ecs_bundle(PServerTag)]
pub struct ProfileServerBundle {
    /// Maps to [super::ecs::UserLabel]
    pub c_label: Option<String>,
    /// Maps to [super::ecs::PServerSettingURL]
    pub c_url: Option<String>,
    /// Maps to [super::ecs::PServerPublicKey]
    pub c_server_key: Option<ServerPublicKey>,
    /// Maps to [super::ecs::PServerAssocProfile]
    pub c_assoc_profile: BundleRef<ProfileServerBundle>,
}

/// See [super::ecs::PServerPublicKey]
#[ecs_bundle(PServerTag)]
#[derive(Debug)]
pub struct ServerPublicKey {
    pub public_key: hn_keys::PublicKeyKind,
    pub retrieved_at: std::time::SystemTime,
}

// todo: add a collection for storing device keys
#[derive(schema::Schema)]
#[schema(name = "DesktopDBSchema", collections = [ProfileBundle, ProfileServerBundle])]
pub struct DBSchema;
