use hn_app::{app_ctx::LocalDatabase, database_plugin::LastImport};

use super::*;

type ViewAll<'a> = (View<'a, HintedID>, ViewCred<'a>, ViewDevice<'a>);
type ViewCred<'a> = (View<'a, ecs::CredTag>, View<'a, ecs::EcsDiscordCred>);
type ViewDevice<'a> = (
    View<'a, ecs::DeviceTag>,
    View<'a, ecs::Linked<ecs::CredTag>>,
    View<'a, ecs::AuthorizedKeys>,
);

pub(super) fn export_all(
    uv_local_database: UniqueView<LocalDatabase>,
    mut uvm_last_import: UniqueViewMut<LastImport>,
    (v_hinted_id, v_cred, v_device): ViewAll,
) {
    let _span = tracing::info_span!("export_all").entered();
    match uv_local_database.as_ref().as_ref() {
        Ok(db) => {
            export_changed_creds(&db, uvm_last_import.as_mut(), &v_hinted_id, &v_cred);
            export_changed_devices(
                &db,
                uvm_last_import.as_mut(),
                &v_hinted_id,
                &v_cred.0,
                &v_device,
            );
        }
        Err(err) => {
            error!(?err, "failed to export all");
        }
    }
}

fn export_changed_creds(
    db: &local::Database,
    last_import: &mut LastImport,
    v_hinted_id: &View<HintedID>,
    (v_cred_tag, v_discord_cred): &ViewCred,
    //
) {
    let _span = tracing::info_span!("export_changed_creds").entered();
    let updated = {
        let _span = info_span!("collect changed creds documents").entered();
        v_cred_tag.iter().ids().filter_map(|entity| {
            if last_import.skip_once(entity) {
                return None;
            }
            if v_hinted_id.is_inserted_or_modified(entity)
                || v_discord_cred.is_inserted_or_modified(entity)
            {
                Some((
                    v_hinted_id.get(entity).ok()?,
                    v_discord_cred.get(entity).ok()?,
                ))
            } else {
                None
            }
        })
    };

    for (id, discord_cred) in updated {
        let _span = info_span!("updating creds document", ?id).entered();
        match CredBundle::overwrite(
            id,
            CredBundle {
                kind: CredBundleKind::Discord {
                    c_discord_cred: discord_cred.clone(),
                },
            },
            db,
        ) {
            Ok(_) => {
                info!(?id, "updated creds document");
            }
            Err(err) => {
                error!(?err, ?id, "failed to update creds document");
            }
        }
    }
}

fn export_changed_devices(
    db: &local::Database,
    last_import: &mut LastImport,
    v_hinted_id: &View<HintedID>,
    v_cred_tag: &View<ecs::CredTag>,
    (v_device_tag, v_linked_creds, v_authorized_keys): &ViewDevice,
    //
) {
    let _span = tracing::info_span!("export_changed_devices").entered();
    let updated = {
        let _span = info_span!("collect changed device documents").entered();
        v_device_tag.iter().ids().filter_map(|entity| {
            if last_import.skip_once(entity) {
                return None;
            }
            if v_hinted_id.is_inserted_or_modified(entity)
                || v_linked_creds.is_inserted_or_modified(entity)
                || v_authorized_keys.is_inserted_or_modified(entity)
            {
                Some((
                    // saying "?" means that if we miss something then we will not come back to this
                    // but that should be okay, because we'll check again when the status has changed.
                    v_hinted_id.get(entity).ok()?,
                    v_linked_creds.get(entity).ok()?,
                    v_authorized_keys.get(entity).ok()?,
                ))
            } else {
                None
            }
        })
    };

    for (id, linked_creds, authorized_keys) in updated {
        let _span = info_span!("updating device document", ?id).entered();
        let mut items = Vec::<HintedID>::new();
        for entity_id in linked_creds.items.iter() {
            match v_hinted_id.get(*entity_id) {
                Ok(id) => {
                    if v_cred_tag.contains(*entity_id) {
                        items.push(id.clone());
                    } else {
                        error!(
                            err = "missing component",
                            ?entity_id,
                            ?id,
                            "linked cred id entity did not have the cred tag"
                        );
                    }
                }
                Err(err) => {
                    error!(?err, ?entity_id, "failed to find cred id based on entity");
                }
            }
        }
        match DeviceBundle::overwrite(
            id,
            DeviceBundle {
                c_linked_creds: LinkedBundle {
                    items,
                    _mark: PhantomData,
                },
                c_authorized_keys: authorized_keys.clone(),
            },
            db,
        ) {
            Ok(_) => {
                info!(?id, "updated creds document");
            }
            Err(err) => {
                error!(?err, ?id, "failed to update creds document");
            }
        }
    }
}
