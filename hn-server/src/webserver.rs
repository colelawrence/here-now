use std::{borrow::Cow, sync::Arc};

use axum::{extract, extract::Extension, http::HeaderValue, response::Html, routing::get, Router};
use bonsaidb::{
    core::{async_trait::async_trait, connection::AsyncConnection},
    server::{CustomServer, HttpService, Peer},
};
use cfg_if::cfg_if;
use futures::{stream::FuturesUnordered, StreamExt};
use hyper::{header, server::conn::Http, Body, Request, Response, StatusCode};
use minority_game_shared::whole_percent;
use serde::{Deserialize, Serialize};
use tera::Tera;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

use crate::{
    schema::{PlayerByScore, PlayerStats},
    sort_players, CustomServerExt, Game,
};

cfg_if! {
    if #[cfg(debug_assertions)] {
        const STATIC_PATH: &str = "./client/static";
        const PKG_PATH: &str = "./client/pkg";
    } else {
        const PKG_PATH: &str = "./pkg";
        const STATIC_PATH: &str = "./static";
    }
}

#[derive(Debug, Clone)]
pub struct WebServer {
    server: CustomServer<Game>,
    templates: Arc<Tera>,
}

impl WebServer {
    pub(super) async fn new(server: CustomServer<Game>) -> Self {
        let mut templates = Tera::default();
        templates
            .add_raw_template("stats", &stats_template().await)
            .unwrap();
        let templates = Arc::new(templates);

        Self { server, templates }
    }
}

#[async_trait]
impl HttpService for WebServer {
    async fn handle_connection<
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
    >(
        &self,
        connection: S,
        peer: &Peer,
    ) -> Result<(), S> {
        if let Err(err) = Http::new()
            .serve_connection(connection, self.router(peer))
            .with_upgrades()
            .await
        {
            log::error!("[http] error serving {}: {:?}", peer.address, err);
        }

        Ok(())
    }
}

impl WebServer {
    fn webapp(&self, peer: &Peer) -> Router {
        Router::new()
            .nest(
                "/pkg",
                axum::routing::get_service(ServeDir::new(PKG_PATH)).handle_error(
                    |err: std::io::Error| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("unhandled internal error: {}", err),
                        )
                    },
                ),
            )
            .nest(
                "/static",
                axum::routing::get_service(ServeDir::new(STATIC_PATH)).handle_error(
                    |err: std::io::Error| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("unhandled internal error: {}", err),
                        )
                    },
                ),
            )
            .route("/ws", get(upgrade_websocket))
            .route("/game", axum::routing::get(spa_index))
            .route("/stats", axum::routing::get(stats))
            .route("/", axum::routing::get(index))
            // Attach the server and the remote address as extractable data for the /ws route
            .layer(Extension(self.server.clone()))
            .layer(Extension(peer.clone()))
            .layer(Extension(self.templates.clone()))
            .layer(SetResponseHeaderLayer::if_not_present(
                header::STRICT_TRANSPORT_SECURITY,
                HeaderValue::from_static("max-age=31536000; preload"),
            ))
    }

    #[cfg(debug_assertions)]
    fn router(&self, peer: &Peer) -> Router {
        self.webapp(peer)
    }

    #[cfg(not(debug_assertions))]
    fn router(&self, peer: &Peer) -> Router {
        if peer.secure {
            self.webapp(peer)
        } else {
            Router::new()
                .nest("/", axum::routing::get(redirect_to_https))
                .layer(Extension(self.server.clone()))
        }
    }
}

#[cfg(not(debug_assertions))]
async fn redirect_to_https(
    server: extract::Extension<CustomServer<Game>>,
    req: hyper::Request<Body>,
) -> hyper::Response<Body> {
    let path = req.uri().path();
    let mut response = hyper::Response::new(Body::empty());
    *response.status_mut() = hyper::StatusCode::PERMANENT_REDIRECT;
    response.headers_mut().insert(
        "Location",
        HeaderValue::from_str(&format!("https://{}{}", server.primary_domain(), path)).unwrap(),
    );
    response
}

async fn upgrade_websocket(
    server: extract::Extension<CustomServer<Game>>,
    peer: extract::Extension<Peer>,
    req: Request<Body>,
) -> Response<Body> {
    server.upgrade_websocket(peer.address, req).await
}

#[allow(clippy::unused_async)]
async fn index() -> Html<Cow<'static, str>> {
    let file_contents = {
        cfg_if! {
            if #[cfg(debug_assertions)] {
                Cow::Owned(tokio::fs::read_to_string("server/src/index.html")
                    .await
                    .unwrap())
            } else {
                Cow::Borrowed(include_str!("../../server/src/index.html"))
            }
        }
    };

    Html::from(file_contents)
}

#[allow(clippy::unused_async)]
async fn spa_index() -> Html<Cow<'static, str>> {
    let file_contents = {
        cfg_if! {
            if #[cfg(debug_assertions)] {
                Cow::Owned(tokio::fs::read_to_string("client/bootstrap.html")
                    .await
                    .unwrap())
            } else {
                Cow::Borrowed(include_str!("../../client/bootstrap.html"))
            }
        }
    };

    Html::from(file_contents)
}

async fn stats_template() -> Cow<'static, str> {
    cfg_if! {
        if #[cfg(debug_assertions)] {
            Cow::Owned(tokio::fs::read_to_string("server/src/stats.tera.html")
                .await
                .unwrap())
        } else {
            Cow::Borrowed(include_str!("../../server/src/stats.tera.html"))
        }
    }
}

async fn stats(
    server: extract::Extension<CustomServer<Game>>,
    templates: extract::Extension<Arc<Tera>>,
) -> Html<String> {
    let mut current_players = server
        .connected_clients()
        .await
        .iter()
        .map(|client| client.client_data())
        .collect::<FuturesUnordered<_>>()
        .filter_map(|player| async move { player.clone() })
        .collect::<Vec<_>>()
        .await;

    sort_players(&mut current_players);

    let db = server.game_database().await.unwrap();
    let top_players = db
        .view::<PlayerByScore>()
        .descending()
        .limit(10)
        .query()
        .await
        .unwrap();

    let html = templates
        .render(
            "stats",
            &tera::Context::from_serialize(Stats {
                current_players: current_players
                    .iter()
                    .enumerate()
                    .map(|(index, player)| {
                        RankedPlayer::from_player_stats(
                            &player.contents.stats,
                            player.header.id,
                            index,
                        )
                    })
                    .collect(),
                top_players: top_players
                    .into_iter()
                    .enumerate()
                    .map(|(index, map)| {
                        RankedPlayer::from_player_stats(
                            &map.value,
                            map.source.id.deserialize().unwrap(),
                            index,
                        )
                    })
                    .collect(),
            })
            .unwrap(),
        )
        .unwrap();

    Html::from(html)
}

#[derive(Serialize, Deserialize, Debug)]
struct RankedPlayer {
    id: u64,
    rank: u32,
    happiness: u32,
    times_went_out: u32,
    times_stayed_in: u32,
}

impl RankedPlayer {
    pub fn from_player_stats(player: &PlayerStats, id: u64, index: usize) -> Self {
        Self {
            id,
            rank: index as u32 + 1,
            happiness: whole_percent(player.happiness),
            times_stayed_in: player.times_stayed_in,
            times_went_out: player.times_went_out,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    current_players: Vec<RankedPlayer>,
    top_players: Vec<RankedPlayer>,
}
