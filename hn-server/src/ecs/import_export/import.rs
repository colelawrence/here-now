use std::collections::HashMap;

use hn_app::{app_ctx::LocalDatabase, database_plugin::LastImport};

use super::*;

type ViewMutAll<'a> = (ViewMut<'a, HintedID>, ViewMutCred<'a>, ViewMutDevice<'a>);
type ViewMutCred<'a> = (ViewMut<'a, ecs::CredTag>, ViewMut<'a, ecs::EcsDiscordCred>);
type ViewMutDevice<'a> = (
    ViewMut<'a, ecs::DeviceTag>,
    ViewMut<'a, ecs::Linked<ecs::CredTag>>,
    ViewMut<'a, ecs::AuthorizedKeys>,
);

pub(super) fn import_all(
    uv_local_database: UniqueView<LocalDatabase>,
    mut last_import: UniqueViewMut<LastImport>,
    mut entities: EntitiesViewMut,
    (mut vm_hinted_id, mut vm_cred, mut vm_device): ViewMutAll,
) -> Result<()> {
    let _span = tracing::info_span!("import_all").entered();
    if uv_local_database.is_inserted_or_modified() {
        let mut map: HashMap<HintedID, EntityId> = HashMap::new();
        let db = uv_local_database.get_database();
        let db = db.as_err_arc_ref()?;

        import_creds(
            &db,
            &mut map,
            &mut entities,
            &mut vm_hinted_id,
            &mut vm_cred,
        )?;
        import_devices(
            &db,
            &mut map,
            &mut entities,
            &mut vm_hinted_id,
            &mut vm_device,
        )?;

        last_import.0.extend(map.iter().map(|a| *a.1));
    } else {
        warn!("database not changed, but import called");
    }

    Ok(())
}

fn import_creds(
    db: &local::Database,
    map: &mut HashMap<HintedID, EntityId>,
    mut entities: &mut EntitiesViewMut,
    vm_hinted_id: &mut ViewMut<HintedID>,
    (vm_cred_tag, vm_discord_cred): &mut ViewMutCred,
    //
) -> Result<()> {
    let _span = tracing::info_span!("import_creds from bonsai").entered();
    for cred in CredBundle::all(db).query().context("getting all creds")? {
        // let doc = cred.to_document().context("cred to document")?;
        map.insert(
            cred.header.id.clone(),
            match cred.contents.kind {
                CredBundleKind::Discord { c_discord_cred } => (&mut entities).add_entity(
                    (&mut *vm_hinted_id, &mut *vm_cred_tag, &mut *vm_discord_cred),
                    (cred.header.id, ecs::CredTag::Discord, c_discord_cred),
                ),
            },
        );
    }

    Ok(())
}

fn import_devices(
    db: &local::Database,
    map: &mut HashMap<HintedID, EntityId>,
    mut entities: &mut EntitiesViewMut,
    vm_hinted_id: &mut ViewMut<HintedID>,
    (vm_device_tag, vm_linked_creds, vm_authorized_keys): &mut ViewMutDevice,
    //
) -> Result<()> {
    let _span = tracing::info_span!("import_devices from bonsai").entered();
    for device in DeviceBundle::all(db)
        .query()
        .context("getting all devices")?
    {
        let DeviceBundle {
            c_authorized_keys,
            c_linked_creds,
        } = device.contents;
        map.insert(
            device.header.id.clone(),
            (&mut entities).add_entity(
                (
                    &mut *vm_hinted_id,
                    &mut *vm_device_tag,
                    &mut *vm_linked_creds,
                    &mut *vm_authorized_keys,
                ),
                (
                    device.header.id,
                    ecs::DeviceTag,
                    ecs::Linked::new_with(
                        c_linked_creds
                            .items
                            .into_iter()
                            .map(|id| *map.get(&id).expect("linked cred exists")),
                    ),
                    c_authorized_keys,
                ),
            ),
        );
    }

    Ok(())
}
