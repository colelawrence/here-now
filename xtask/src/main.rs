use clap::{self, Parser};
use devx_cmd::Cmd;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{collections::BTreeMap, ops::Rem};
use tracing::*;

use command_ext::{get_project_root_dir, CommandExt};

mod command_ext;

#[derive(Debug, Parser)]
enum Args {
    /// Build web assets for development
    WebBuild {
        #[clap(long)]
        watch: bool,
    },
    /// Run server for development
    Dev {
        /// Connect to jaeger
        #[clap(long)]
        jaeger: bool,
    },
    /// Run desktop for development
    DevDesktop {
        /// Connect to jaeger
        #[clap(long)]
        jaeger: bool,
    },
    View {
        /// Which file
        file: PathBuf,
    },
    /// Assorted lint fixes
    Fix,
    /// Develop loop for protocol definitions
    DevProtocol,
    /// Generate and show docs
    Doc,
    /// Build docker container
    Docker {
        #[clap(long)]
        bash: bool,
    },
    /// Run Jaeger
    Jaeger {
        #[clap(long)]
        docker: bool,
        /// If you're expecting to see the Jaeger dashboard via the config server proxy /dev/jaeger
        #[clap(long)]
        proxied: bool,
    },
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
        Args::Jaeger { docker, proxied } => jaeger(docker, proxied).join(),
        Args::Dev { jaeger } => dev(jaeger),
        Args::DevDesktop { jaeger } => dev_desktop(jaeger),
        Args::View { file } => viewer(file),
        Args::Fix => fix(),
        Args::Doc => doc(),
        Args::Docker { bash } => build_docker(bash),
        Args::DevProtocol => dev_protocol(),
    }
}

fn jaeger(docker: bool, proxied: bool) -> jod_thread::JoinHandle {
    let proxy_base_path = "/dev/traces";
    if docker {
        Cmd::new("docker")
            .args("run --name jaeger".split(' '))
            .arg("--rm") // remove container when it exits
            .arg("-p16686:16686") // open port for web ui
            .arg("-p14268:14268") // open port for trace collector http
            .arg("jaegertracing/all-in-one:latest")
            .arg("--")
            .arg_if(proxied, &format!("--query.base-path={proxy_base_path}"))
            .root_dir(".")
            .run_in_thread("starting jaeger in docker")
    } else {
        eprintln!("Starting jaeger locally. You can download jaeger binaries from https://github.com/jaegertracing/jaeger/releases/");
        Cmd::new("jaeger-all-in-one")
            .root_dir("./xtask/jaeger")
            .arg("--query.ui-config=./jaeger-config.json")
            .arg_if(proxied, &format!("--query.base-path={proxy_base_path}"))
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

#[instrument]
fn dev(jaeger: bool) {
    let server = Cmd::new("cargo")
        .env("HERE_NOW_LOG", "debug,!pot,!nebari")
        .env("HERE_NOW_CONFIG_FOLDER", "../conf")
        .env_if(
            jaeger,
            "JAEGER_COLLECTOR_ENDPOINT",
            "http://localhost:14268/api/traces",
        )
        .arg("run")
        .arg("--quiet")
        .root_dir("./hn-server")
        .watchable(true, "-w ./src -w ../hn-common -w ../hn-app -e rs")
        .run_in_thread("watch and run hn-server Rust program");

    let web_assets = jod_thread::spawn(|| {
        web_build(true);
    });

    web_assets.join();
    server.join();
}

#[instrument]
fn dev_desktop(jaeger: bool) {
    Cmd::new("cargo")
        .env("HERE_NOW_LOG", "debug,!pot,!nebari")
        .env("SLINT_DEBUG_PERFORMANCE", "refresh_lazy,overlay")
        .env("DYLD_FALLBACK_LIBRARY_PATH", "~/lib:/usr/local/lib:/usr/lib")
        .env("SLINT_NO_QT", "1")
        .env_if(
            jaeger,
            "JAEGER_COLLECTOR_ENDPOINT",
            "http://localhost:14268/api/traces",
        )
        .arg("run")
        .arg("--quiet")
        .root_dir("./hn-desktop")
        .watchable(
            true,
            "-w ./src -w ../hn-common -w ../hn-app -w ../hn-desktop-ui-messages -w ../hn-desktop-ui -w ../hn-desktop-executor -e rs",
        )
        .run_in_thread("watch and run hn-desktop Rust program");
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
