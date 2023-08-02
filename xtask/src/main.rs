use clap::{self, Parser};
use devx_cmd::Cmd;

use command_ext::CommandExt;
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
    /// Assorted lint fixes
    Fix,
    /// Generate and show docs
    Doc,
    /// Run Jaeger
    Jaeger {
        #[clap(long)]
        docker: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args {
        Args::WebBuild { watch } => web_build(watch),
        Args::Jaeger { docker } => jaeger(docker).join(),
        Args::Dev { jaeger } => dev(jaeger),
        Args::Fix => fix(),
        Args::Doc => doc(),
    }
}

fn jaeger(docker: bool) -> jod_thread::JoinHandle {
    if docker {
        Cmd::new("docker")
            .args("run --name jaeger".split(' '))
            .arg("--rm") // remove container when it exits
            .arg("-p16686:16686") // open port for web ui
            .arg("-p14268:14268") // open port for trace collector http
            .arg("jaegertracing/all-in-one:latest")
            .root_dir(".")
            .run_in_thread("starting jaeger in docker")
    } else {
        eprintln!("Starting jaeger locally. You can download jaeger binaries from https://github.com/jaegertracing/jaeger/releases/");
        Cmd::new("jaeger-all-in-one")
            .root_dir("./xtask/jaeger")
            .arg("--query.ui-config=./jaeger-config.json")
            .run_in_thread("starting jaeger locally")
    }
}

fn web_build(watch: bool) {
    eprintln!("Building web dependencies, watch={watch:?}");

    let typescript = Cmd::new("npx")
        .args("tsc -p ./design-tools/tsconfig.json".split(' '))
        .arg_if(watch, "--watch")
        .arg_if(watch, "--preserveWatchOutput")
        .root_dir(".")
        .run_in_thread("build design tool typescript like TailwindCSS settings");

    let svelte_generator = Cmd::new("cargo")
        .args("test --bin server -- app_server_plugins::public_server::generate_svelte_templates --exact --nocapture".split(' '))
        .root_dir(".")
        .run_watchable(
            "built svelte template generated code",
            watch,
            "-w hn-server/templates/generator -e ts,rs -w hn-server/src/app_server_plugins/public_server.rs",
        );

    let svelte = Cmd::new("deno")
        .args("run -A ./svelte-tools/compile-svelte.ts ./hn-server/templates".split(' '))
        .root_dir(".")
        .run_watchable(
            "built svelte templates",
            watch,
            "-w ./hn-server/templates -e svelte,ts --ignore ./hn-server/templates/generator",
        );

    let tailwind = Cmd::new("npx")
        .args("tailwindcss -i hn-server/config-html-server.css -o hn-server/src/config_html_server/build/config-html-server.css".split(' '))
        .root_dir(".")
        .arg_if(watch, "--watch")
        .run_in_thread("tailwindcss compilation");

    typescript.join();
    svelte.join();
    tailwind.join();
    svelte_generator.join();
}

fn dev(jaeger: bool) {
    let server = Cmd::new("cargo")
        .env("RUST_LOG", "debug,!hyper")
        .env("HERE_NOW_CONFIG_FOLDER", "../conf")
        .env_if(
            jaeger,
            "JAEGER_COLLECTOR_ENDPOINT",
            "http://localhost:14268/api/traces",
        )
        .args("watch --watch ./src --ignore *.j2 --ignore *.css".split(' '))
        .arg("--exec")
        .arg("run")
        .root_dir("./hn-server")
        .run_in_thread("watch and run hn-server Rust program");

    let web_assets = jod_thread::spawn(|| {
        web_build(true);
    });

    web_assets.join();
    server.join();
}

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

fn doc() {
    Cmd::new("cargo")
        .args("+nightly doc --workspace --open --target aarch64-apple-darwin".split(' '))
        // ensure not to get wasm bindgen stuff
        // the server and the desktop should work on this architecture
        .args("--target aarch64-apple-darwin".split(' '))
        .root_dir(".")
        .run_it("geenrate and open docs");
}
