use bonsaidb::{core::schema::SerializedCollection, local};
use hn_app::{
    _ecs_::*, _tracing_::*, app_ctx::LocalDatabase, database_plugin::LastImport, HintedID,
};

use super::{data, ecs};

pub(crate) type VShared<'a> = (View<'a, HintedID>, View<'a, ecs::UserLabel>);
pub(crate) type VProfile<'a> = (View<'a, ecs::ProfileTag>, View<'a, ecs::ProfileKeys>);
pub(crate) type VPServer<'a> = (
    View<'a, ecs::PServerTag>,
    View<'a, ecs::PServerAssocProfile>,
    View<'a, ecs::PServerSettingURL>,
    View<'a, ecs::PServerPublicKey>,
);

#[instrument(skip_all)]
pub(super) fn sync_changes_to_database_system(
    uv_local_database: UniqueView<LocalDatabase>,
    mut last_import: UniqueViewMut<LastImport>,
    v_shared: VShared,
    v_profile: VProfile,
    v_pserver: VPServer,
) {
    match uv_local_database.get_database().as_ref().as_ref() {
        Ok(db) => {
            export_changed_profiles(&db, last_import.as_mut(), &v_shared, &v_profile);
            export_changed_pservers(&db, last_import.as_mut(), &v_shared, &v_pserver);
        }
        Err(err) => {
            // TODO report this to the UI somehow?
            error!(?err, "failed to export all");
        }
    }
}

#[instrument(skip_all)]
fn export_changed_pservers(
    db: &local::Database,
    last_import: &mut LastImport,
    (v_db_id, v_user_label): &VShared,
    (v_pserver_tag, v_pserver_assoc_profile, v_pserver_setting_url, v_pserver_public_key): &VPServer,
) {
    let updated = {
        let _span = info_span!("collect changed pserver documents").entered();
        v_pserver_tag.iter().ids().filter_map(|entity| {
            if last_import.skip_once(entity) {
                return None;
            }
            if v_db_id.is_inserted_or_modified(entity)
                || v_user_label.is_inserted_or_modified(entity)
                || v_pserver_assoc_profile.is_inserted_or_modified(entity)
                || v_pserver_setting_url.is_inserted_or_modified(entity)
                || v_pserver_public_key.is_inserted_or_modified(entity)
            {
                Some((
                    v_db_id.get(entity).ok()?,
                    v_user_label.get(entity).ok()?,
                    v_pserver_assoc_profile.get(entity).ok()?,
                    v_pserver_setting_url.get(entity).ok()?,
                    v_pserver_public_key.get(entity).ok()?,
                ))
            } else {
                None
            }
        })
    };

    for (id, user_label, assoc_profile, setting_url, public_key) in updated {
        let _span = info_span!("updating pserver document", ?id).entered();
        let assoc_db_id = v_db_id
            .get(assoc_profile.0)
            .as_setup_err(f!("Failed to find associated profile for server ({id:?})"))
            .unwrap();
        match data::ProfileServerBundle::overwrite(
            &id,
            data::ProfileServerBundle {
                c_label: user_label.0.clone(),
                c_assoc_profile: data::BundleRef::new(assoc_db_id.clone()),
                c_url: setting_url.0.clone(),
                c_server_key: public_key.0.as_ref().ok().cloned(),
            },
            db,
        ) {
            Ok(_) => {
                info!(?id, "updated profile server document");
            }
            Err(err) => {
                error!(?err, ?id, "failed to update profile server document");
            }
        }
    }
}

#[instrument(skip_all)]
fn export_changed_profiles(
    db: &local::Database,
    last_import: &mut LastImport,
    (v_db_id, v_user_label): &VShared,
    (v_profile_tag, v_profile_keys): &VProfile,
) {
    let updated = {
        let _span = info_span!("collect changed profile documents").entered();
        v_profile_tag.iter().ids().filter_map(|entity| {
            if last_import.skip_once(entity) {
                return None;
            }
            if v_db_id.is_inserted_or_modified(entity)
                || v_user_label.is_inserted_or_modified(entity)
                || v_profile_keys.is_inserted_or_modified(entity)
            {
                Some((
                    v_db_id.get(entity).ok()?,
                    v_user_label.get(entity).ok()?,
                    v_profile_keys.get(entity).ok()?,
                ))
            } else {
                None
            }
        })
    };

    for (id, user_label, keys) in updated {
        let _span = info_span!("updating profile document", ?id).entered();
        match data::ProfileBundle::overwrite(
            &id,
            data::ProfileBundle {
                c_label: user_label.0.clone(),
                c_keys: keys.0.clone(),
            },
            db,
        ) {
            Ok(_) => {
                info!(?id, "updated profile document");
            }
            Err(err) => {
                error!(?err, ?id, "failed to update profile document");
            }
        }
    }
}
