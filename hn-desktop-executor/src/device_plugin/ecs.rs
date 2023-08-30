use super::data;
use hn_app::_ecs_::*;
#[ecs_component(ProfileTag)]
pub struct ProfileTag;

/// ID found for the document in BonsaiDB.
#[ecs_component(ProfileTag)]
pub struct ProfileKeys(pub hn_keys::LocalKeys);

#[ecs_component(ProfileTag, PServerTag)]
pub struct UserLabel(pub Option<String>);

#[ecs_component(PServerTag)]
pub struct PServerTag;

#[ecs_component(PServerTag)]
pub struct PServerAssocProfile(pub shipyard::EntityId);

#[ecs_component(PServerTag)]
pub struct PServerSettingURL(pub Option<String>);

#[ecs_component(PServerTag)]
pub struct PServerPublicKey(pub SetupResult<data::ServerPublicKey>);
