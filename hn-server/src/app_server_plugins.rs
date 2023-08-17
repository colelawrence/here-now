use std::time::Duration;

use crate::prelude::*;
use hn_app::_ecs_::*;

mod app_server_config_plugin;
mod discord;
mod public_server;

pub use app_server_config_plugin::PublicServerBaseURL;

#[derive(Default)]
pub struct AppServerPlugin {
    default_socket_addr: Option<std::net::SocketAddr>,
}

impl Plugin for AppServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(app_server_config_plugin::AppServerConfigPlugin {
            default_socket_addr: self.default_socket_addr.clone(),
        });
        app.add_plugin(discord::DiscordSettingsPlugin::default());
        app.add_unique(PublicServer {
            current_handle: None,
        });
        app.add_system(maintain_public_server_system);
        info!("Setting up app server plugin");
    }
}

/// Unique
#[derive(Component)]
struct PublicServer {
    // handle?
    current_handle: Option<axum_server::Handle>,
}

#[tracing::instrument(skip_all)]
fn maintain_public_server_system(
    uv_app_ctx: UniqueView<AppCtx>,
    uv_public_bind_address: UniqueView<app_server_config_plugin::PublicServerBindAddress>,
    mut uvm_public_server: UniqueViewMut<PublicServer>,
) {
    if uv_public_bind_address.is_inserted_or_modified() {
        if uvm_public_server.current_handle.is_some() {
            info!("Shutting down previous server gracefully");
            let current = uvm_public_server.current_handle.take().unwrap();
            // Hmmm: If you replace the addr from `0.0.0.0:80` to `127.0.0.1:80`, will the port conflict?
            current.graceful_shutdown(Some(Duration::from_secs(20)));
        }
        // shutdown gradefully if running
        let listener = uv_public_bind_address
            .as_ref()
            .0
            .as_ref()
            .as_ref()
            .map_err(|err| anyhow::anyhow!("{err:?}"))
            .and_then(|addr| {
                // Hmmm: If you replace the addr from `0.0.0.0:80` to `127.0.0.1:80`, will the port conflict?
                Ok((
                    std::net::TcpListener::bind(addr).context("starting public server listener")?,
                    addr,
                ))
            });

        let updated = match listener {
            Ok((listener, addr)) => Some(public_server::start_server_from_tcp_listener(
                listener,
                addr,
                uv_app_ctx.clone(),
            )),
            Err(err) => {
                // need to be able to surface this somehow?
                error!(?err, "failed to start public server");
                None
            }
        };

        uvm_public_server.current_handle = updated;
    }
}
