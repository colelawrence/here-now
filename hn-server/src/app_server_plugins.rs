use std::{str::FromStr, time::Duration};

use axum::{routing::get, Router};

use crate::{
    config_plugins::{self, ReadConfigFile},
    prelude::*,
};

#[derive(Default)]
pub struct AppServerPlugin {
    default_socket_addr: Option<std::net::SocketAddr>,
}

#[derive(Component, Clone)]
#[track(All)]
pub struct AppServerConfigFile(());

impl ReadConfigFile for AppServerConfigFile {
    type Content = toml_edit::Document;
    type Error = anyhow::Error;

    fn relative_path(&self) -> &str {
        "here-now-app.toml"
    }

    fn load(&self, bytes: &[u8]) -> Result<Self::Content, Self::Error> {
        let str = String::from_utf8(bytes.to_vec()).with_context(|| "loading test.toml config")?;
        let doc =
            toml_edit::Document::from_str(&str).with_context(|| "parsing test.toml as toml")?;
        Ok(doc)
    }
}

impl Plugin for AppServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_tracked_value(PublicServerBindAddress(
            self.default_socket_addr
                .context("default socket address not configured"),
        ));
        app.add_unique(PublicServer {
            current_handle: None,
        });
        app.add_plugin(config_plugins::ConfigFilePlugin(AppServerConfigFile(())));
        info!("Setting up app server plugin");
        app.add_system(index_bind_address_system);
        app.add_system(maintain_public_server_system);
    }
}

/// Unique
#[derive(Component)]
#[track(All)]
struct PublicServerBindAddress(pub Result<std::net::SocketAddr>);

/// Unique
#[derive(Component)]
struct PublicServer {
    // handle?
    current_handle: Option<axum_server::Handle>,
}

fn index_bind_address_system(
    uv_config: UniqueView<config_plugins::ConfigFileContent<AppServerConfigFile>>,
    mut uvm_public_bind_address: UniqueViewMut<PublicServerBindAddress>,
) {
    if uv_config.is_inserted_or_modified() {
        let new_bind_address_res = uv_config
            .get_content()
            .context("expected config to have content")
            .and_then(|inner| inner.content.as_err_arc_ref())
            .and_then(|doc| {
                doc.get("public_bind_address")
                    .context("Toml has public_bind_address key defined")
            })
            .and_then(|item| {
                item.as_str()
                    .context("expected public_bind_address to be a string")
            })
            .and_then(|str| {
                str.parse::<std::net::SocketAddr>()
                    .context("expected public_bind_address to be parseable as a socket address")
            })
            .map_err(|e| anyhow::anyhow!("{e:?}"));

        if uvm_public_bind_address.0.as_ref().ok() == new_bind_address_res.as_ref().ok() {
            return; // already up to date!
        }

        // as_mut marks it for modified
        uvm_public_bind_address.as_mut().0 = new_bind_address_res;
    }
}

fn maintain_public_server_system(
    uv_app_ctx: UniqueView<AppCtx>,
    uv_public_bind_address: UniqueView<PublicServerBindAddress>,
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
            .map_err(|err| anyhow::anyhow!("{err:?}"))
            .and_then(|addr| {
                // Hmmm: If you replace the addr from `0.0.0.0:80` to `127.0.0.1:80`, will the port conflict?
                Ok((
                    std::net::TcpListener::bind(addr).context("starting public server listener")?,
                    addr,
                ))
            });

        let updated = match listener {
            Ok((listener, addr)) => Some(start_server_from_tcp_listener(
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

fn start_server_from_tcp_listener(
    listener: std::net::TcpListener,
    addr: &std::net::SocketAddr,
    app_ctx: AppCtx,
) -> axum_server::Handle {
    info!(?addr, "starting public server");
    let handle = axum_server::Handle::new();
    let server = axum_server::from_tcp(listener).handle(handle.clone());

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    app_ctx.spawn(async {
        server
            .serve(app.into_make_service())
            .await
            .context("serving public app")
    });

    handle
}
