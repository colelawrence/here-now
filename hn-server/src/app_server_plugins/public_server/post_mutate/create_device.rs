use super::*;
use crate::prelude::*;
use hn_app::_ecs_::*;

#[async_trait]
impl Mutation for hn_public_api::CreateDeviceMutation {
    #[instrument(skip(app_ctx), name = "create device mutation")]
    async fn mutate(
        &self,
        sender: &hn_keys::PublicKeyKind,
        app_ctx: AppCtx,
    ) -> hn_public_api::MutateResult<Self> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let tx = std::sync::Mutex::new(Some(tx));
        let label = self.label.clone();
        let sender = sender.clone();
        app_ctx.run_system(
            "create device",
            move |mut entities: EntitiesViewMut,
                  mut vm_hinted_id: ViewMut<ecs::HintedID>,
                  mut vm_device_tag: ViewMut<ecs::DeviceTag>,
                  mut vm_linked_creds: ViewMut<ecs::Linked<ecs::CredTag>>,
                  mut vm_authorized_keys: ViewMut<ecs::AuthorizedKeys>| {
                let device_id = ecs::HintedID::generate("web");
                tx.lock()
                    .unwrap()
                    .take()
                    .unwrap()
                    .send(device_id.clone())
                    .unwrap();
                entities.add_entity(
                    (
                        &mut vm_device_tag,
                        &mut vm_hinted_id,
                        &mut vm_linked_creds,
                        &mut vm_authorized_keys,
                    ),
                    (
                        ecs::DeviceTag,
                        device_id.clone(),
                        ecs::Linked::new_with(None),
                        ecs::AuthorizedKeys {
                            keys: vec![ecs::AuthorizedKey {
                                label: Some(label.clone()),
                                dev_info: None,
                                key: sender.clone(),
                            }],
                        },
                    ),
                );
            },
        );

        rx.await
            .context("receiving device id")
            .map(|device_id| hn_public_api::CreateDeviceResponse {
                device_id: device_id.to_id_string(),
            })
            .map_err(|err| hn_public_api::MutateRejection::InternalError(err.to_string()))
    }
}
