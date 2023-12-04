use clap::{self, Parser};
use devx_cmd::Cmd;
use std::ffi::OsString;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{collections::BTreeMap, ops::Rem};
use tracing::*;

use command_ext::{get_project_root_dir, CmdHandle, CommandExt};

mod command_ext;

#[derive(Debug, Parser)]
enum Args {
    /// Build web assets for development
    WebBuild {
        #[clap(long)]
        watch: bool,
    },
    /// Run server for development
    DevServer {
        /// Connect to jaeger
        #[clap(long)]
        no_jaeger: bool,
    },
    /// Run desktop for development
    DevDesktop {
        /// Connect to jaeger
        #[clap(long)]
        no_jaeger: bool,
    },
    /// Commands for right-now (rn-desktop)
    Rn {
        #[command(subcommand)]
        subcommand: RnArgs,
    },
    /// Build desktop app
    Desktop {
        /// Build desktop app (not run)
        #[clap(long)]
        build: bool,
        /// Don't send logs to jaeger
        #[clap(long)]
        no_jaeger: bool,
        /// Generate timing information and open the artifact
        #[clap(long)]
        timings: bool,
        /// Add a name to artifacts like the timing report
        #[clap(long)]
        label: Option<String>,
        /// Watch and rebuild on changes
        #[clap(long)]
        watch: bool,
    },
    /// View and watch a .slint file using slint-viewer of our Slint UI fork.
    View {
        /// Which file
        file: PathBuf,
    },
    /// Generate a [hn_hinted_id::HintedID] given a prefix first argument and optionally a count
    GenHintedID {
        /// What is the prefix?
        prefix: String,
        count: Option<usize>,
    },
    /// Assorted lint fixes
    Fix,
    /// Regenerate hakari workspace-hack (should happen after any dependency changes)
    Hakari,
    /// Develop loop for protocol definitions
    DevProtocol,
    /// Generate and show docs
    Doc,
    /// Build docker container
    Docker {
        #[clap(long)]
        bash: bool,
    },
    // /// Format SQL files (perhaps used by lint-staged)
    // FormatSql { files: Vec<PathBuf> },
    /// Run Jaeger
    Jaeger {
        #[clap(long)]
        docker: bool,
        /// If you're expecting to see the Jaeger dashboard via the config server proxy /dev/jaeger
        #[clap(long)]
        no_proxy: bool,
    },
    /// Use SQLx CLI
    Sqlx { rest: Vec<OsString> },
    /// Use Sea ORM CLI
    SeaOrmGenerate {
        #[command(subcommand)]
        command: sea_orm_cli::cli::GenerateSubcommands,
    },
}

#[derive(Debug, Parser)]
enum RnArgs {
    /// Add a migration step with SQLx to the Right Now codebase.
    DbAddMigration {
        /// The name of the migration
        rest: Vec<OsString>,
    },
    /// Generate SeaORM entity Rust files for Right Now Sqlite.
    DbGenRust,
    /// `revert` last SQLx migration, `run` SQLx migrations, and generate SeaORM entity Rust files for Right Now Sqlite.
    DbRevertRunGen {
        /// Watch and rebuild on changes
        #[clap(long)]
        watch: bool,
    },
    /// Run Right Now desktop for development
    Dev {
        /// Don't send logs to jaeger
        #[clap(long)]
        no_jaeger: bool,
    },
    /// Generate ui interfaces
    GenUi {
        /// Watch and regenerate on changes
        #[clap(long)]
        watch: bool,
        /// Wait until first change before running command
        #[clap(long)]
        watch_postpone: bool,
    },
    /// Regenerate icons with tauri cli
    GenIcons,
    /// Use SQLx CLI with the Right Now Sqlite database
    Sqlx { rest: Vec<OsString> },
}

fn main() {
    hn_tracing::expect_init_logger_jaeger(
        "hn-xtask",
        Some("http://localhost:14268/api/traces".to_string()),
    );
    let args = Args::parse();
    let _span = info_span!("main", ?args).entered();

    match args {
        Args::WebBuild { watch } => web_build(watch),
        Args::Jaeger { docker, no_proxy } => jaeger(docker, no_proxy).join(),
        Args::DevServer { no_jaeger } => server_cmd(no_jaeger),
        Args::DevDesktop { no_jaeger } => {
            desktop_cmd(no_jaeger, true, true, false, Some("dev".to_string()))
        }
        Args::Rn { subcommand } => run_right_now_cmd(subcommand),
        Args::Desktop {
            build,
            no_jaeger,
            timings,
            watch,
            label,
        } => desktop_cmd(no_jaeger, build, watch, timings, label),
        Args::View { file } => viewer(file),
        Args::Hakari => hakari(),
        Args::Fix => fix(),
        Args::Doc => doc(),
        Args::Docker { bash } => build_docker(bash),
        Args::DevProtocol => dev_protocol(),
        Args::GenHintedID { prefix, count } => generate_hinted_id(&prefix, count),
        Args::Sqlx { rest } => {
            let err = format!("Run SQLx subcommand with args {rest:?}");
            let mut rest = rest;
            rest.insert(0, "sqlx".into());
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(sqlx_cli::run(sqlx_cli::Opt::parse_from(rest)))
                .expect(&err)
        }
        // Args::FormatSql { files } => {
        //     for file in files {
        //         if file.extension().map(|a| a != "sql").unwrap_or(true) {
        //             tracing::warn!(?file, "Skipping non-sql file");
        //             continue;
        //         }
        //         let content = std::fs::read_to_string(&file).expect("read file to sql format");
        //         let mut updated = sqlformat::format(
        //             &content,
        //             &sqlformat::QueryParams::None,
        //             sqlformat::FormatOptions {
        //                 indent: sqlformat::Indent::Spaces(2),
        //                 uppercase: true,
        //                 ..Default::default()
        //             },
        //         );
        //         if !updated.ends_with('\n') {
        //             updated.push('\n');
        //         }
        //         if updated != content {
        //             std::fs::write(file, updated).expect("write file");
        //         }
        //     }
        // }
        Args::SeaOrmGenerate { command } => tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(sea_orm_cli::run_generate_command(command, false))
            .unwrap(),
    }
}

fn hakari() {
    Cmd::new("cargo")
        .args("hakari generate".split(' '))
        .root_dir(".")
        .run_it("update hakari dependencies")
}

fn generate_hinted_id(prefix: &str, count: Option<usize>) {
    Cmd::new("cargo")
        .args("run --example generate --quiet".split(' '))
        .arg("--")
        .arg(prefix)
        .arg(count.unwrap_or(1).to_string())
        .root_dir("./hn-hinted-id")
        .run_it("generate hinted id")
}

fn jaeger(docker: bool, no_proxy: bool) -> CmdHandle {
    let proxy_base_path = "/dev/traces";
    if docker {
        Cmd::new("docker")
            .args("run --name jaeger".split(' '))
            .arg("--rm") // remove container when it exits
            .arg("-p16686:16686") // open port for web ui
            .arg("-p14268:14268") // open port for trace collector http
            .arg("jaegertracing/all-in-one:latest")
            .arg("--")
            .arg_if(!no_proxy, &format!("--query.base-path={proxy_base_path}"))
            .root_dir(".")
            .run_in_thread("starting jaeger in docker")
    } else {
        eprintln!("Starting jaeger locally. You can download jaeger binaries from https://github.com/jaegertracing/jaeger/releases/");
        Cmd::new("jaeger-all-in-one")
            .root_dir("./xtask/jaeger")
            .arg("--query.ui-config=./jaeger-config.json")
            .arg_if(!no_proxy, &format!("--query.base-path={proxy_base_path}"))
            .run_with_printer("starting jaeger locally", print_jaeger_line)
    }
}

#[instrument]
fn web_build(watch: bool) {
    eprintln!("Building web dependencies, watch={watch:?}");

    let typescript = Cmd::new("npx")
        .args("tsc -p ./design-tools/tsconfig.json".split(' '))
        .arg_if(watch, "--watch")
        .arg_if(watch, "--preserveWatchOutput")
        .root_dir(".")
        .run_in_thread("build design tool typescript like TailwindCSS settings");

    let svelte_generator = Cmd::new("cargo")
        .args("test --quiet --bin hn-server -- app_server_plugins::public_server::generate_svelte_templates --exact --nocapture --ignored".split(' '))
        .root_dir(".")
        .watchable(
            watch,
            "-w hn-server/templates/generator -e ts,rs -w hn-server/src/app_server_plugins/public_server.rs",
        ).run_in_thread("built svelte template generated code");

    let svelte_generator_data_browser = Cmd::new("cargo")
        .args("test --quiet --bin hn-server -- config_html_server::data_browser::generate_svelte_templates --exact --nocapture --ignored".split(' '))
        .root_dir(".")
        .watchable(
            watch,
            "-w hn-server/templates/generator -e ts,rs -w hn-server/src/config_html_server/data_browser.rs",
        ).run_in_thread("built svelte template generated code for data browser");

    let svelte = Cmd::new("deno")
        .args("run -A ./svelte-tools/compile-svelte.ts ./hn-server/templates".split(' '))
        .root_dir(".")
        .watchable(
            watch,
            "-w ./hn-server/templates -e svelte,ts --ignore ./hn-server/templates/generator",
        )
        .run_in_thread("built svelte templates");

    let tailwind = Cmd::new("npx")
        .args("tailwindcss -i hn-server/config-html-server.css -o hn-server/src/config_html_server/build/config-html-server.css".split(' '))
        .root_dir(".")
        .arg_if(watch, "--watch")
        .run_in_thread("tailwindcss compilation");

    typescript.join();
    svelte.join();
    tailwind.join();
    svelte_generator.join();
    svelte_generator_data_browser.join();
}

const WATCH_COMMON_DEPS: &str =
    "-w ../hn-public-api -w ../hn-keys -w ../hn-hinted-id -w ../hn-tracing -w ../hn-app";

#[instrument]
fn server_cmd(no_jaeger: bool) {
    let server = Cmd::new("cargo")
        .env("HERE_NOW_LOG", "debug,!pot,!nebari")
        .env("HERE_NOW_CONFIG_FOLDER", "../conf")
        .env_if(
            !no_jaeger,
            "JAEGER_COLLECTOR_ENDPOINT",
            "http://localhost:14268/api/traces",
        )
        .env("RUST_BACKTRACE", "1")
        .arg("run")
        .arg("--quiet")
        .root_dir("./hn-server")
        .watchable(true, &format!("-w ./src {WATCH_COMMON_DEPS} -e rs"))
        .run_in_thread("watch and run hn-server Rust program");

    let web_assets = jod_thread::spawn(|| {
        web_build(true);
    });

    web_assets.join();
    server.join();
}

fn run_right_now_cmd(subcommand: RnArgs) {
    match subcommand {
        RnArgs::Dev { no_jaeger } => right_now_dev_cmd(no_jaeger),
        RnArgs::DbAddMigration { rest } => right_now_db_add_migration_cmd(&rest),
        RnArgs::DbGenRust => right_now_db_gen_rust_cmd(),
        RnArgs::GenUi {
            watch,
            watch_postpone,
        } => right_now_rust_code_gen_thread(watch, watch_postpone).join(),
        RnArgs::GenIcons => {
            let src = get_project_root_dir().join("rn-desktop/src-tauri/icons/app-icon.png");
            if !src.exists() {
                panic!("src icon does not exist at {src:?}");
            }
            Cmd::new("pnpm")
                .args("exec tauri icon --output=src-tauri/icons-gen".split(' '))
                .arg(src)
                .root_dir("./rn-desktop")
                .run_it("generate icons for rn-desktop Tauri program");
        }
        RnArgs::DbRevertRunGen { watch } => {
            if watch {
                // run self without watch but with watch params
                return Cmd::new("cargo")
                    .args("xtask rn db-revert-run-gen".split(' '))
                    .root_dir("./rn-desktop/src-tauri")
                    .watchable(true, "-w ./migrations -e sql")
                    .run_it("watch and run right now database dev");
            }

            right_now_db_sqlx_scoped_cmd()
                .args("migrate revert".split(' '))
                .run_it("revert last");
            right_now_db_sqlx_scoped_cmd()
                .args("migrate run".split(' '))
                .run_it("run next");
            right_now_db_gen_rust_cmd();
        }
        RnArgs::Sqlx { rest } => right_now_db_sqlx_scoped_cmd()
            .args(rest)
            .run_it("run SQLx subcommand in Right Now"),
    }
}

fn right_now_rust_code_gen_thread(watch: bool, postpone: bool) -> CmdHandle {
    let postpone_watch_arg = if postpone { "--postpone " } else { "" };
    Cmd::new("cargo")
        .args("test --package rn-desktop --bin rn-desktop -- ui::generate_ui_typescript --exact --nocapture --ignored".split(' '))
        .root_dir("./rn-desktop")
        .watchable(
            watch,
            &format!("{postpone_watch_arg}--debounce=2sec -w src-tauri/src/ui.rs -w src-tauri/src/rn_todos_plugin.rs -w src-tauri/dev-codegen -e ts,rs"),
        ).run_in_thread("generated ui typescript code for Tauri front-end")
}

#[instrument]
fn right_now_dev_cmd(no_jaeger: bool) {
    // drop on exiting
    let codegen = right_now_rust_code_gen_thread(true, true);
    Cmd::new("pnpm")
        .args("tauri dev".split(' '))
        .root_dir("./rn-desktop")
        .env(
            "RIGHTNOW_APP_DATA_DIR",
            get_project_root_dir().join("rn-desktop/dev-app-data"),
        )
        .env_if(
            !no_jaeger,
            "JAEGER_COLLECTOR_ENDPOINT",
            "http://localhost:14268/api/traces",
        )
        .run_it("build or run rn-desktop Tauri program");

    // kill codegen if tauri exited from some reason...
    codegen.kill();
}

#[instrument]
fn right_now_db_add_migration_cmd(args: &[OsString]) {
    right_now_db_sqlx_scoped_cmd()
        .args("migrate add -r".split(' '))
        .args(args)
        .run_it("add migration to rn-desktop Tauri program");
}

fn right_now_db_sqlx_scoped_cmd() -> Cmd {
    Cmd::new("cargo")
        .env("DATABASE_URL", "sqlite:./rightnow.sqlite?mode=rwc")
        .root_dir("./rn-desktop/src-tauri")
        // need the -- so we pass the rest of the params as though they are external to this command
        .args("xtask sqlx --".split(' '))
        .to_owned()
}

#[instrument]
fn right_now_db_gen_rust_cmd() {
    right_now_db_sqlx_scoped_cmd()
        .args("migrate run".split(' '))
        .run_it("apply migrations before generating");
    Cmd::new("cargo")
        .env("DATABASE_URL", "sqlite:./rightnow.sqlite?mode=rwc")
        .args("xtask sea-orm-generate entity".split(' '))
        .arg2("--output-dir", "./src/db_gen")
        .root_dir("./rn-desktop/src-tauri")
        .run_it("generate sea-orm entity files for rn-desktop Tauri program");
}

#[instrument]
fn desktop_cmd(no_jaeger: bool, build: bool, watch: bool, timings: bool, label: Option<String>) {
    Cmd::new("cargo")
    .env("HERE_NOW_LOG", "debug,!pot,!nebari")
    .env("SLINT_DEBUG_PERFORMANCE", "refresh_lazy,overlay")
    .env("DYLD_FALLBACK_LIBRARY_PATH", "~/lib:/usr/local/lib:/usr/lib")
    // Ensure we don't try to link to Qt (which would add complexity to the build)
    .env("SLINT_NO_QT", "1")
    .env("RUST_BACKTRACE", "1")
        .env_if(
            !no_jaeger,
            "JAEGER_COLLECTOR_ENDPOINT",
            "http://localhost:14268/api/traces",
        )
        // ensure that the timings use a different cache than IDE and other dev tools
        .arg_if(timings, "--config")
        .arg_if(timings, &format!("build.target-dir={:?}", get_project_root_dir().join("target/cargo-timings-target").to_str().expect("valid utf8 path")))
        .arg_if(!build, "run")
        // .args_if(timings, "+nightly")
        // .args_if(timings, "-Zunstable-options --timings=html")
        .args_if(build, "build")
        .args_if(timings, "--timings")
        .root_dir("./hn-desktop")
        .watchable(
            watch,
            &format!("-w ./src {WATCH_COMMON_DEPS} -w ../hn-desktop-ui-messages -w ../hn-desktop-ui -w ../hn-desktop-executor -e rs"),
        )
        .run_it("build or run hn-desktop Rust program");

    if !watch && build && timings {
        // get last in list of in directory
        let mut file_names =
            std::fs::read_dir(get_project_root_dir().join("target/cargo-timings/"))
                .expect("timings exist")
                .into_iter()
                .filter_map(|a| a.ok())
                .map(|a| a.path())
                // get only the ones with a timestamp
                .filter(|path| !path.ends_with("cargo-timing.html"))
                .collect::<Vec<PathBuf>>();

        // get most recent timing html
        file_names.sort();

        let last = file_names
            .last()
            .expect("at least one timing file with timestamp");

        // rename the file with the label as suffix if specified
        if let Some(label) = label {
            let mut new_name_str = last.to_str().expect("valid utf8 path").to_string();
            let last_dot = new_name_str
                .rfind('.')
                .expect("at least one dot in file name");
            new_name_str.insert_str(last_dot, &format!("-{}", label));
            std::fs::rename(last, &new_name_str).expect("renaming timing file");
            Cmd::new("open")
                .arg(new_name_str)
                .run_it("open renamed timing report");
        } else {
            Cmd::new("open").arg(last).run_it("open timing report");
        }
    }
}

#[instrument]
fn viewer(file: PathBuf) {
    Cmd::new("cargo")
        .env("SLINT_DEBUG_PERFORMANCE", "refresh_lazy,overlay")
        .env("SLINT_NO_QT", "1")
        .arg("run")
        .arg("--quiet")
        .arg2("-p", "slint-viewer")
        .arg("--")
        .arg("--auto-reload")
        .arg2("--style", "fluent")
        .arg2("--backend", "winit")
        .arg(file)
        .run_in_thread("preview slint file with slint-viewer");
}

#[instrument]
fn dev_protocol() {
    devx_cmd::Cmd::new("cargo")
        .args(
            "test --quiet --package hn-server --bin hn-server -- data::generate --exact --nocapture --ignored"
                .split(' '),
        )
        .root_dir("./hn-server")
        .watchable(true, "-w ./proc -w ./src/data.rs -w ./protocols/generator")
        .run_it("watching and generating protocol code");
}

#[instrument]
fn fix() {
    Cmd::new("cargo")
        .args("fix --allow-dirty --allow-staged".split(' '))
        .root_dir(".")
        .run_it("fixing rust code in workspace");

    Cmd::new("cargo")
        .args("fmt".split(' '))
        .root_dir(".")
        .run_it("format rust files in workspace");
}

#[instrument]
fn doc() {
    Cmd::new("cargo")
        .args("+nightly doc --workspace --open".split(' '))
        // ensure not to get wasm bindgen stuff
        // the server and the desktop should work on this architecture
        .arg2("--target", current_platform::CURRENT_PLATFORM)
        // ugh... I don't know why the error occurs that there's a duplicate "crate".
        .arg2("--exclude", "i-codegen-derive")
        .root_dir(".")
        .run_it("generate and open docs");
}

#[instrument]
fn build_docker(bash: bool) {
    let tag = "herenow/server";
    // docker build --file=./Dockerfile.here-now -t herenow/server .  && docker run -it herenow/server /bin/bash
    Cmd::new("docker")
        .env("DOCKER_BUILDKIT", "1")
        .arg("build")
        .arg("--file=./Dockerfile.here-now")
        .arg(&format!("--tag={tag}"))
        .arg(".")
        .root_dir(".")
        .run_it("building docker image");

    if bash {
        let mut child = Command::new("docker")
            .arg("run")
            .arg("-it")
            .arg(tag)
            .arg("/bin/bash")
            .spawn() // inherits stdin, stdout, stderr
            .expect("running docker image");
        child.wait().expect("docker exited");
    } else {
        let conf = get_project_root_dir().join("./conf");
        let conf_str = conf.to_str().expect("conf path is valid utf8");
        Cmd::new("docker")
            .arg("run")
            // remove on parent command exit
            .args("--rm --init --name herenow".split(' '))
            .arg2("-e", "HERE_NOW_CONFIG_FOLDER=/app/conf")
            .arg2("-p", "3000:3000")
            .arg2("-p", "9000:9000")
            .arg2("-v", format!("{conf_str}:/app/conf"))
            .arg(tag)
            .root_dir(".")
            .run_it("running docker image");
    }
}
/// ```json
/// {"level":"info","ts":1690998316.6558468,"caller":"fswatcher/fswatcher.go:117","msg":"Received event","event":"CHMOD         \"jaeger-config.json\""}
/// {"level":"info","ts":1690998341.6361809,"caller":"app/static_handler.go:150","msg":"reloaded UI config","filename":"./jaeger-config.json"}
/// {"level":"info","ts":1690999310.2993999,"caller":"fswatcher/fswatcher.go:117","msg":"Received event","event":"CHMOD         \"jaeger-config.json\""}
/// ```
fn print_jaeger_line(line: &str) {
    #[derive(serde::Deserialize)]
    #[allow(unused)]
    struct JaegerLine {
        level: String,
        msg: String,
        ts: f64,
        caller: String,
        #[serde(flatten)]
        rest: BTreeMap<String, serde_json::Value>,
    }

    const ASCII_YELLOW: &str = "\x1b[33m";
    const ASCII_RED: &str = "\x1b[31m";
    const ASCII_CYAN: &str = "\x1b[36m";
    const ASCII_DIM: &str = "\x1b[2m";
    const ASCII_RESET: &str = "\x1b[0m";
    const ASCII_RESET_DIMMED: &str = "\x1b[0m\x1b[2m";
    fn colored(name: &str) -> String {
        let mut total = String::new();
        for part in name.split(':') {
            if total.len() > 0 {
                total.push_str(ASCII_DIM);
                total.push(':');
                total.push_str(ASCII_RESET);
            }
            let bytes = part.as_bytes();
            let len = bytes.len();
            if len > 2 {
                let x = unsafe { *bytes.get_unchecked(0) as usize };
                let y = unsafe { *bytes.get_unchecked(len - 1) as usize };
                let z = unsafe { *bytes.get_unchecked(1) as usize };
                // Bright on workerlog
                // See ./wikipedia-ansi-color-chart.png
                // Inspired by https://github.com/autoplayhq/workerlog/blob/c2e773c3bee59ff092255d32e5c07bc4e2c29b1f/src/workerlog.ts#L461-L463
                let mut r = x.rem(6);
                let mut g = y.rem(6);
                let mut b = z.rem(6);
                if (r + g + b) < 3 {
                    // too dark
                    r += 1;
                    g += 1;
                    b += 1;
                }

                if r == g && r == b {
                    // too gray
                    b += 1;
                }
                let hue = 16 + r * 36 + g * 6 + b;
                write!(&mut total, "\x1b[38;5;{hue}m{part}").unwrap();
            } else {
                write!(&mut total, "{ASCII_CYAN}{part}").unwrap();
            }
        }
        total
    }

    match serde_json::from_str::<JaegerLine>(&line) {
        Ok(JaegerLine {
            level,
            msg,
            caller: _,
            ts: _,
            rest,
        }) => {
            // let caller = colored(caller.split('/').last().unwrap_or(&caller));
            let (level_color, msg_color) = match level.as_str() {
                "info" => (ASCII_CYAN, ASCII_RESET_DIMMED),
                "error" => (ASCII_RED, ASCII_RESET),
                "warn" => (ASCII_YELLOW, ASCII_RESET),
                _ => (ASCII_RESET_DIMMED, ASCII_RESET_DIMMED),
            };
            print!("{level_color}{level}{msg_color} {msg} ");
            for (key, value) in rest {
                print!("{}{msg_color}{msg_color}={} ", colored(&key), value);
            }
            println!("{ASCII_RESET}");
        }
        Err(_) => println!("{ASCII_DIM}{line}{ASCII_RESET}"),
    }
}
