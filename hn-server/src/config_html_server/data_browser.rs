use std::{collections::HashMap, time::SystemTime};

use axum::{extract::Path, response::Html, routing::get, Extension, Router};
use derive_codegen::Codegen;
use tokio::sync::oneshot;

use crate::{config::Settings, ecs::HintedID, http::OrInternalError, prelude::*, svelte_templates};

#[derive(Serialize, Codegen)]
#[codegen(tags = "data-browser")]
#[codegen(template = "data-collections")]
struct DataCollections {
    header: PageHeader,
}

#[derive(Serialize, Codegen)]
#[codegen(tags = "data-browser")]
struct PageHeader {
    title: String,
    /// label, then href
    links: Vec<(&'static str, &'static str)>,
    warning: Option<String>,
}

#[derive(Serialize, Codegen)]
#[codegen(tags = "data-browser")]
#[codegen(template = "collection-page")]
struct CollectionPage {
    header: PageHeader,
    rows: Vec<CollectionRow>,
    // sort_options: Vec<String>
}

#[derive(Serialize, Codegen)]
#[codegen(tags = "data-browser")]
struct CollectionRow {
    #[codegen(ts_as = "string")]
    id: HintedID,
    #[codegen(ts_as = "unknown")]
    content: serde_json::value::Value,
    // #[codegen(ts_as = "undefined | Record<string, unknown>")]
    ecs_content: Option<String>,
    // sort_options: Vec<String>
}

#[test]
#[ignore]
fn generate_svelte_templates() {
    derive_codegen::Generation::for_tag("data-browser")
        .as_arg_of(
            std::process::Command::new("deno")
                .args("run ./generator/generate-typescript.ts".split(' '))
                .args("--sharedFileName=templates.ts".split(' '))
                .current_dir(get_crate_path().join("templates")),
        )
        .with_output_path("data-browser")
        .write()
        .print();
}

pub(super) fn create_data_browser_router(app_ctx: AppCtx) -> Router<Arc<Settings>> {
    let router = Router::<Arc<Settings>>::new();

    let templates_path = get_crate_path()
        .join("templates")
        .canonicalize()
        .expect("templates path exists");

    router
        .route("/", get(get_home))
        .route("/:collection_id", get(get_collection))
        .layer(Extension(app_ctx))
        .layer(Extension(svelte_templates::SvelteTemplates {
            dev_path: Arc::new(templates_path),
        }))
}

async fn get_home(
    // Extension(app_ctx): Extension<AppCtx>,
    Extension(templates): Extension<svelte_templates::SvelteTemplates>,
) -> HttpResult {
    render_home(
        &templates,
        PageHeader {
            title: "Data Browser".to_string(),
            links: get_links(),
            warning: None,
        },
    )
    .await
}

async fn render_home(
    templates: &svelte_templates::SvelteTemplates,
    header: PageHeader,
) -> HttpResult {
    let template = svelte_template!("data-browser/data-collections.template.compiled.cjs");
    templates
        .render_svelte_into_html_page(&template, DataCollections { header })
        .context("rendering data browser page")
        .err_500()
        .map(Html)
}

fn get_links() -> std::vec::Vec<(&'static str, &'static str)> {
    return vec![("Credentials", "/data/creds"), ("Devices", "/data/devices")];
}

async fn get_all_rows<
    C: bonsaidb::core::schema::SerializedCollection<Contents = C, PrimaryKey = HintedID>
        + Send
        + Sync
        + Serialize,
>(
    db: &bonsai_::local::Database,
    app_ctx: &AppCtx,
) -> HttpResult<Vec<CollectionRow>> {
    let (tx, rx) = oneshot::channel::<HashMap<HintedID, Option<String>>>();

    let results = C::all(db)
        .query()
        .err_500()?
        .into_iter()
        .map(|cred| -> Result<CollectionRow> {
            Ok(CollectionRow {
                id: cred.header.id,
                content: serde_json::to_value(cred.contents).context("cred to json value")?,
                ecs_content: None,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .err_500()?;

    let empty_ids = results
        .iter()
        .map(|a| (a.id.clone(), None))
        .collect::<HashMap<HintedID, Option<String>>>();

    let id = SystemTime::now();

    app_ctx.run_system("collect ecs info for database", {
        let once = std::sync::Mutex::new(Some((empty_ids, tx)));
        move |storages: AllStoragesView| {
            if let Some((mut ids, tx)) = once.lock().unwrap().take() {
                let v_hinted_id = storages.borrow::<View<HintedID>>().unwrap();

                for (id, ecs_id) in v_hinted_id.iter().with_id() {
                    if let Some(value) = ids.get_mut(ecs_id) {
                        value.replace(format!("{id:?}"));
                    }
                }

                tx.send(ids).unwrap();
            } else {
                error!(?id, "unexpected second execution");
            }
        }
    });

    let mut results = results;
    let mut ecs_data = rx.await.context("receive back info from ECS").err_500()?;
    for item in results.iter_mut() {
        if let Some(value) = ecs_data
            .get_mut(&item.id)
            .expect("id still from original list")
            .take()
        {
            item.ecs_content = Some(value);
        }
    }

    Ok(results)
}

async fn get_collection(
    Extension(app_ctx): Extension<AppCtx>,
    Extension(templates): Extension<svelte_templates::SvelteTemplates>,
    Path((collection_id,)): Path<(String,)>,
) -> HttpResult {
    let db = app_ctx.get_database().await;
    let db = db.as_err_arc_ref().err_500()?;

    let (label, rows) = match collection_id.as_str() {
        "creds" => (
            "Credentials",
            get_all_rows::<ecs::import_export::CredBundle>(db, &app_ctx).await?,
        ),
        "devices" => (
            "Devices",
            get_all_rows::<ecs::import_export::DeviceBundle>(db, &app_ctx).await?,
        ),
        other => {
            return render_home(
                &templates,
                PageHeader {
                    title: "Not found".to_string(),
                    links: get_links(),
                    warning: Some(format!("Collection <code>{other:?}</code> not found.")),
                },
            )
            .await;
        }
    };

    let template = svelte_template!("data-browser/collection-page.template.compiled.cjs");
    templates
        .render_svelte_into_html_page(
            &template,
            CollectionPage {
                header: PageHeader {
                    title: format!("Collection / {label}"),
                    links: get_links(),
                    warning: None,
                },
                rows,
            },
        )
        .context("rendering data browser page")
        .err_500()
        .map(Html)
}
