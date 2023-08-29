use hn_app::{HintedID, _ecs_::*, _tracing_::*};

use super::{ecs, export_data, Messages};

#[ecs_unique]
pub struct UIWindowState {
    first_launch: bool,
    // main_shown: bool,
    // settings_shown: bool,
}

/// Plugin to handle the ui window opening requests.
///
/// Consider: split this out between the main window and various settings windows like "PServerSettingsPlugin"?.
#[derive(Default)]
pub struct WindowsPlugin(());

impl Plugin for WindowsPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder.add_unique(UIWindowState {
            first_launch: true,
            // main_shown: false,
            // settings_shown: false,
        });

        builder.add_system(sync_window_state_on_first_launch);
        builder.add_system(handle_open_messages);
    }
}

fn sync_window_state_on_first_launch(
    mut uvm_window_state: UniqueViewMut<UIWindowState>,
    mut uvm_executor_messages: UniqueViewMut<Messages<ui::ToExecutor>>,
) {
    if uvm_window_state.is_inserted_or_modified() {
        if uvm_window_state.first_launch {
            warn!("handle first launch");
            uvm_window_state.first_launch = false;
            // choose what to show based on current settings
            // uvm_executor_messages.add(ui::ToExecutor::CreateProfile);
            uvm_executor_messages.add(ui::ToExecutor::OpenMainWindow);
        }
    }
}

fn handle_open_messages(
    mut uvm_executor_messages: UniqueViewMut<Messages<ui::ToExecutor>>,
    mut uvm_ui_messages: UniqueViewMut<Messages<ui::ToUI>>,
    v_uid: View<HintedID>,
    v_label: View<ecs::UserLabel>,
    v_pserver: export_data::VPServer,
) {
    warn!("handle open settings {:?}", uvm_executor_messages.0);
    if !uvm_executor_messages.is_inserted_or_modified() {
        return;
    }

    uvm_executor_messages.handle(|a| match a {
        ui::ToExecutor::OpenMainWindow => {
            uvm_ui_messages.add(ui::ToUI::ShowMainWindow);
            Ok(true)
        }
        ui::ToExecutor::OpenPServerSettings(uid) => {
            // iterate over all storages to find the one with the uid
            let entity = v_uid
                .iter()
                .with_id()
                .find_map(
                    |(entity, ent_uid)| {
                        if uid == ent_uid {
                            Some(entity)
                        } else {
                            None
                        }
                    },
                )
                .as_setup_err(f!("Failed to find entity for uid ({uid:?})"))?;
            let (
                ecs::UserLabel(label),
                (
                    _tag,
                    ecs::PServerAssocProfile(_assoc_profile),
                    ecs::PServerSettingURL(setting_url),
                    ecs::PServerPublicKey(_),
                ),
            ) = (&v_label, v_pserver.as_tuple_refs())
                .get(entity)
                .as_setup_err(f!(
                    "Failed to find Profile Server information for uid ({uid:?})"
                ))?;

            uvm_ui_messages.add(ui::ToUI::ShowPServerSettings(ui::PServerSettings {
                uid: uid.clone(),
                label: ui::Setting::from_option(&label),
                server_url: ui::Setting::from_option(&setting_url),
            }));
            Ok(true)
        }
        _ => Ok(false),
    });
}
