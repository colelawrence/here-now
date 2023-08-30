use std::collections::HashMap;

use bonsaidb::{core::schema::SerializedCollection, local};
use hn_app::{
    _ecs_::*, _result_::AsErrArcRefExt, _tracing_::*, app_ctx::LocalDatabase,
    database_plugin::LastImport, HintedID,
};

use super::{data, ecs};

pub(crate) type VMShared<'a> = (ViewMut<'a, HintedID>, ViewMut<'a, ecs::UserLabel>);
pub(crate) type VMProfile<'a> = (ViewMut<'a, ecs::ProfileTag>, ViewMut<'a, ecs::ProfileKeys>);
pub(crate) type VMPServer<'a> = (
    ViewMut<'a, ecs::PServerTag>,
    ViewMut<'a, ecs::PServerAssocProfile>,
    ViewMut<'a, ecs::PServerSettingURL>,
    ViewMut<'a, ecs::PServerPublicKey>,
);

#[instrument(skip_all)]
pub(super) fn import_data_from_database_system(
    uv_local_database: UniqueView<LocalDatabase>,
    mut last_import: UniqueViewMut<LastImport>,
    mut entities: EntitiesViewMut,
    mut vm_shared: VMShared,
    mut vm_profile: VMProfile,
    mut vm_pserver: VMPServer,
) {
    if !uv_local_database.is_inserted_or_modified() {
        return;
    }

    let db = uv_local_database.get_database();
    let db = db
        .as_err_arc_ref()
        .as_setup_err("Database failed to load.")
        .expect("database to load");

    import_settings_from_database_inner(
        &db,
        &mut last_import,
        &mut entities,
        &mut vm_shared,
        &mut vm_profile,
        &mut vm_pserver,
    )
    .expect("import from database?")
}

#[instrument(skip_all)]
fn import_settings_from_database_inner(
    db: &local::Database,
    last_import: &mut UniqueViewMut<LastImport>,
    mut entities: &mut EntitiesViewMut,
    mut vm_shared: &mut VMShared,
    mut vm_profile: &mut VMProfile,
    mut vm_pserver: &mut VMPServer,
) -> SetupResult<()> {
    let mut map: HashMap<HintedID, EntityId> = HashMap::new();

    import_profiles(
        &db,
        &mut map,
        &mut entities,
        &mut vm_shared,
        &mut vm_profile,
    )?;

    import_pservers(
        &db,
        &mut map,
        &mut entities,
        &mut vm_shared,
        &mut vm_pserver,
    )?;

    last_import.0.extend(map.iter().map(|a| *a.1));

    Ok(())
}

#[instrument(skip_all)]
fn import_profiles(
    db: &local::Database,
    map: &mut HashMap<HintedID, EntityId>,
    mut entities: &mut EntitiesViewMut,
    (vm_db_id, vm_user_label): &mut VMShared,
    (vm_profile_tag, vm_profile_keys): &mut VMProfile,
) -> SetupResult<()> {
    for profile in data::ProfileBundle::all(db)
        .query()
        .as_setup_err("Failed to get profiles from database")?
    {
        let db_id = HintedID::from(profile.header.id);
        let data::ProfileBundle { c_label, c_keys } = profile.contents;
        map.insert(db_id.clone(), {
            (&mut entities).add_entity(
                (
                    &mut *vm_db_id,
                    &mut *vm_profile_tag,
                    &mut *vm_user_label,
                    &mut *vm_profile_keys,
                ),
                (
                    db_id,
                    ecs::ProfileTag,
                    ecs::UserLabel(c_label),
                    ecs::ProfileKeys(c_keys),
                ),
            )
        });
    }

    Ok(())
}

#[instrument(skip_all)]
fn import_pservers(
    db: &local::Database,
    map: &mut HashMap<HintedID, EntityId>,
    mut entities: &mut EntitiesViewMut,
    (vm_db_id, vm_user_label): &mut VMShared,
    (vm_pserver_tag, vm_pserver_assoc_profile, vm_pserver_setting_url, vm_pserver_public_key): &mut VMPServer,
) -> SetupResult<()> {
    for profile in data::ProfileServerBundle::all(db)
        .query()
        .as_setup_err("Failed to get server profiles from database")?
    {
        let db_id: HintedID = profile.header.id;
        let data::ProfileServerBundle {
            c_label,
            c_url,
            c_server_key,
            c_assoc_profile,
        } = profile.contents;
        let assoc_profile_entity = map
            .get(&c_assoc_profile.id)
            .copied()
            .as_setup_err(f!("Assoc profile not inserted ({:?})", c_assoc_profile.id))?;
        map.insert(db_id.clone(), {
            (&mut entities).add_entity(
                (
                    &mut *vm_db_id,
                    &mut *vm_pserver_tag,
                    &mut *vm_user_label,
                    &mut *vm_pserver_assoc_profile,
                    &mut *vm_pserver_setting_url,
                    &mut *vm_pserver_public_key,
                ),
                (
                    db_id,
                    ecs::PServerTag,
                    ecs::UserLabel(c_label),
                    ecs::PServerAssocProfile(assoc_profile_entity),
                    ecs::PServerSettingURL(c_url),
                    ecs::PServerPublicKey(c_server_key.as_setup_err("Server key not loaded, yet.")),
                ),
            )
        });
    }

    Ok(())
}
