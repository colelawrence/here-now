use std::{
    ops::Add,
    time::{Duration, SystemTime},
};

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use derive_codegen::Codegen;

use http::{header::LOCATION, StatusCode};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{ecs::HintedID, http::OrInternalError, prelude::*, svelte_templates};

use super::{discord, PublicServerBaseURL};

pub fn start_server_from_tcp_listener(
    listener: std::net::TcpListener,
    addr: &std::net::SocketAddr,
    app_ctx: AppCtx,
) -> axum_server::Handle {
    info!(?addr, "starting public server");
    let handle = axum_server::Handle::new();
    let server = axum_server::from_tcp(listener).handle(handle.clone());

    let templates_path = get_crate_path()
        .join("templates")
        .canonicalize()
        .expect("templates path exists");

    let app = Router::new()
        .route("/", get(login_page))
        .route("/login-discord", get(login_discord))
        .route("/callback-discord", get(callback_discord))
        .nest_service("/public", ServeDir::new(templates_path.join("./public")))
        .layer(TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
            info_span!("public-request", method = %request.method(), uri = %request.uri())
        }))
        .layer(Extension(app_ctx.clone()))
        .layer(Extension(svelte_templates::SvelteTemplates {
            dev_path: Arc::new(templates_path),
        }));

    app_ctx.spawn(async {
        server
            .serve(app.into_make_service())
            .await
            .context("serving public app")
    });

    handle
}

#[derive(Serialize, Codegen)]
#[codegen(tags = "templates")]
#[codegen(template = "login")]
#[allow(non_snake_case)]
pub struct LoginProps {
    loginURLs: Vec<LoginURL>,
}

/// What kind of login URL?
#[derive(Serialize, Codegen)]
#[codegen(tags = "templates")]
pub struct LoginURL {
    label: String,
    url: String,
}

#[test]
#[ignore]
fn generate_svelte_templates() {
    derive_codegen::Generation::for_tag("templates")
        .as_arg_of(
            std::process::Command::new("deno")
                .args("run ./generator/generate-typescript.ts".split(' '))
                .args("--sharedFileName=templates.ts".split(' '))
                .current_dir(get_crate_path().join("templates")),
        )
        .write()
        .print();
}

#[derive(Deserialize)]
struct LoginDiscordParams {
    bot: Option<String>,
    device_id: Option<HintedID>,
}

async fn login_discord(
    Extension(app_ctx): Extension<AppCtx>,
    Query(LoginDiscordParams { bot, device_id }): Query<LoginDiscordParams>,
) -> HttpResult<impl IntoResponse> {
    use axum::response::*;

    let client_id = app_ctx
        .get_unique::<discord::DiscordClientID>("to get current settings for discord callback")
        .await;
    let client_id = client_id.0.as_err_arc_ref().err_500()?;

    let public_server_base_url = app_ctx
        .get_unique::<PublicServerBaseURL>("to get current settings for discord callback")
        .await;
    let public_server_base_url = public_server_base_url.0.as_err_arc_ref().err_500()?;

    // https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
    let mut scopes = vec![
        // "activities.read", // allows your app to fetch data from a user's "Now Playing/Recently Played" list — not currently available for apps
        // "activities.write", // allows your app to update a user's activity - requires Discord approval (NOT REQUIRED FOR GAMESDK ACTIVITY MANAGER)
        // "applications.builds.read", // allows your app to read build data for a user's applications
        // "applications.builds.upload", // allows your app to upload/update builds for a user's applications - requires Discord approval
        // "applications.commands", // allows your app to use commands in a guild
        // "applications.commands.update", // allows your app to update its commands using a Bearer token - client credentials grant only
        // "applications.commands.permissions.update", // allows your app to update permissions for its commands in a guild a user has permissions to
        // "applications.entitlements", // allows your app to read entitlements for a user's applications
        // "applications.store.update", // allows your app to read and update store data (SKUs, store listings, achievements, etc.) for a user's applications
        // "bot", // for oauth2 bots, this puts the bot in the user's selected guild by default
        // "connections", // allows /users/@me/connections to return linked third-party accounts
        // "dm_channels.read", // allows your app to see information about the user's DMs and group DMs - requires Discord approval
        // "email", // enables /users/@me to return an email
        // "gdm.join", // allows your app to join users to a group dm
        // "guilds", // allows /users/@me/guilds to return basic information about all of a user's guilds
        // "guilds.join", // allows /guilds/{guild.id}/members/{user.id} to be used for joining users to a guild
        // "guilds.members.read", // allows /users/@me/guilds/{guild.id}/member to return a user's member information in a guild
        // "relationships.read", // allows your app to know a user's friends and implicit relationships - requires Discord approval
        // "role_connections.write", // allows your app to update a user's connection and metadata for the app
        // "rpc", // for local rpc server access, this allows you to control a user's local Discord client - requires Discord approval
        // "rpc.activities.write", // for local rpc server access, this allows you to update a user's activity - requires Discord approval
        // "rpc.notifications.read", // for local rpc server access, this allows you to receive notifications pushed out to the user - requires Discord approval
        // "rpc.voice.read", // for local rpc server access, this allows you to read a user's voice settings and listen for voice events - requires Discord approval
        // "rpc.voice.write", // for local rpc server access, this allows you to update a user's voice settings - requires Discord approval
        // "voice", // allows your app to connect to voice on user's behalf and see all the voice members - requires Discord approval
        // "webhook.incoming", // this generates a webhook that is returned in the oauth token response for authorization code grants
        // "messages.read", // for local rpc server api access, this allows you to read messages from all client channels (otherwise restricted to channels/guilds your app creates)
        "identify", // allows /users/@me without email
    ];

    if bot.is_some() {
        scopes.push("bot");
    }

    // don't actually create the device until the handoff.
    let device_id = device_id.unwrap_or_else(|| HintedID::generate("web"));

    let scopes = scopes.join("%20");
    let redirect_uri = format!("{public_server_base_url}/callback-discord");
    let redirect_uri = urlencoding::encode(&redirect_uri);
    let url = [
        "https://discord.com/oauth2/authorize?",
        "response_type=code&",
        &format!("state={device_id}&"),
        &format!("client_id={client_id}&"),
        &format!("scope={scopes}&"),
        &format!("redirect_uri={redirect_uri}&"),
        &format!("prompt=consent"),
    ]
    .into_iter()
    .collect::<String>();

    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        AppendHeaders([(LOCATION, url)]),
    ))
}

#[derive(Deserialize, Serialize, Codegen)]
#[codegen(tags = "templates")]
struct CallbackError {
    error: String,
    error_description: Option<String>,
}

#[derive(Serialize, Codegen)]
#[codegen(tags = "templates")]
#[codegen(template = "discord-callback")]
struct DiscordCallbackProps<'a> {
    query: &'a DiscordCallbackQuery,
    text: String,
}

#[derive(Deserialize, Serialize, Codegen)]
#[codegen(tags = "templates")]
struct DiscordCallbackBot {
    /// `&guild_id=936348778330468482`
    guild_id: String,
    /// `&permissions=0`
    permissions: String,
}

#[derive(Deserialize, Serialize, Codegen)]
#[codegen(tags = "templates")]
struct DiscordCallbackQuery {
    state: HintedID,
    /// `error=invalid_scope&error_description=the+requested+scope+is+invalid%2c+unknown%2c+or+malformed.`
    #[serde(flatten)]
    error: Option<CallbackError>,
    /// for when adding a bot workflow
    #[serde(flatten)]
    bot: Option<DiscordCallbackBot>,
    // e.g. `TKNwGgDvQqJdR5ZxMhtZef25JY4KsM`
    code: Option<String>,
}

async fn callback_discord(
    Query(query): Query<DiscordCallbackQuery>,
    Extension(app_ctx): Extension<AppCtx>,
    Extension(templates): Extension<svelte_templates::SvelteTemplates>,
) -> HttpResult {
    let client_id = app_ctx
        .get_unique::<discord::DiscordClientID>("to get current settings for discord callback")
        .await;
    let client_id = client_id.0.as_err_arc_ref().err_500()?;

    let client_secret = app_ctx
        .get_unique::<discord::DiscordClientSecret>("to get current settings for discord callback")
        .await;
    let client_secret = client_secret.0.as_err_arc_ref().err_500()?;

    let public_server_base_url = app_ctx
        .get_unique::<PublicServerBaseURL>("to get current settings for discord callback")
        .await;
    let public_server_base_url = public_server_base_url.0.as_err_arc_ref().err_500()?;

    let text = if let Some(code) = query.code.as_ref() {
        let redirect_uri = format!("{public_server_base_url}/callback-discord");

        let client = reqwest::Client::new();
        // reqwest
        let form = [
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", &redirect_uri),
            ("code", &code),
        ];

        let res = client
            .post("https://discord.com/api/oauth2/token")
            .form(&form)
            .send()
            .await
            .todo(f!("successfully sent"));

        let token = res
            .text()
            .await
            .context("reading text from discord code excange")
            .and_then(|text| {
                serde_json::from_str::<DiscordToken>(&text)
                    .with_context(|| format!("deserializing discord token: `{text}`"))
            })
            .err_400()?;

        let expires_at = SystemTime::now().add(Duration::from_secs(token.expires_in));
        let access_token = token.access_token.clone();
        let device_id = query.state.clone();
        app_ctx.schedule_system(
            "insert new credential",
            move |mut entities: EntitiesViewMut,
                  mut vm_hinted_id: ViewMut<HintedID>,
                  // creds
                  mut vm_cred_tag: ViewMut<ecs::CredTag>,
                  mut vm_discord_cred: ViewMut<ecs::EcsDiscordCred>,
                  // device
                  mut vm_device_tag: ViewMut<ecs::DeviceTag>,
                  mut vm_linked_creds: ViewMut<ecs::Linked<ecs::CredTag>>| {
                let cred_id = HintedID::generate("cred");
                let cred_entity_id = entities.add_entity(
                    (&mut vm_hinted_id, &mut vm_cred_tag, &mut vm_discord_cred),
                    (
                        cred_id,
                        ecs::CredTag::Discord,
                        ecs::EcsDiscordCred {
                            access_token: token.access_token.clone(),
                            refresh_token: token.refresh_token.clone(),
                            expires_at,
                        },
                    ),
                );
                match vm_hinted_id
                    .iter()
                    .with_id()
                    .find(|(_entity_id, id)| *id == &device_id)
                {
                    Some((entity_id, _id)) => {
                        // update
                        let mut linked_creds = (&mut vm_linked_creds)
                            .get(entity_id)
                            .todo(f!("query state was a device id with linked creds"));
                        linked_creds.as_mut().items.push(cred_entity_id);
                    }
                    None => {
                        // insert
                        entities.add_entity(
                            (&mut vm_device_tag, &mut vm_hinted_id, &mut vm_linked_creds),
                            (
                                ecs::DeviceTag,
                                device_id.clone(),
                                ecs::Linked::new_with(Some(cred_entity_id)),
                            ),
                        );
                    }
                }
            },
        );

        client
            .get("https://discord.com/api/users/@me")
            .bearer_auth(access_token)
            .send()
            .await
            .context("getting user info from discord")
            .err_500()?
            .text()
            .await
            .context("reading text from discord user info")
            .err_500()?
    } else {
        String::from("No code from login")
    };

    let template = svelte_template!("discord-callback.template.compiled.cjs");
    templates
        .render_svelte_into_html_page(
            &template,
            DiscordCallbackProps {
                query: &query,
                text,
            },
        )
        .context("rendering login page")
        .err_500()
        .map(Html)
}

/// For example
/// ```json
/// {
///     "token_type": "Bearer",
///     "access_token": "mtrv1234DsMWomqBiooo6RdnCs7zjR",
///     "expires_in": 604800,
///     "refresh_token": "KTdYabcdMBUeXJ3cvRmtdeIXwBnLro",
///     "scope": "identify"
/// }
/// ```
#[derive(Deserialize)]
#[allow(unused)]
struct DiscordToken {
    token_type: String,
    access_token: String,
    /// in seconds
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

async fn login_page(
    Extension(templates): Extension<svelte_templates::SvelteTemplates>,
) -> HttpResult {
    let props = LoginProps {
        loginURLs: vec![
            LoginURL {
                label: "Add Discord Bot".to_string(),
                url: "login-discord?bot".to_string(),
            },
            LoginURL {
                label: "Continue with Discord".to_string(),
                url: "login-discord".to_string(),
            },
            LoginURL {
                label: "Continue with Slack".to_string(),
                url: "login-slack".to_string(),
            },
            LoginURL {
                label: "Continue with Google Workspace".to_string(),
                url: "login-google-workspace".to_string(),
            },
        ],
    };
    let template = svelte_template!("login.template.compiled.cjs");
    templates
        .render_svelte_into_html_page(&template, props)
        .context("rendering login page")
        .err_500()
        .map(Html)
}
