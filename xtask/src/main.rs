use std::{fmt::format, process::Command};

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
    let args = Args::parse();

    match args {
        Args::WebBuild { watch } => web_build(watch),
        Args::Jaeger { docker, proxied } => jaeger(docker, proxied).join(),
        Args::Dev { jaeger } => dev(jaeger),
        Args::Fix => fix(),
        Args::Doc => doc(),
        Args::Docker { bash } => build_docker(bash),
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
        .args("test --bin hn-server -- app_server_plugins::public_server::generate_svelte_templates --exact --nocapture".split(' '))
        .root_dir(".")
        .watchable(
            watch,
            "-w hn-server/templates/generator -e ts,rs -w hn-server/src/app_server_plugins/public_server.rs",
        ).run_in_thread("built svelte template generated code");

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
}

fn dev(jaeger: bool) {
    let server = Cmd::new("cargo")
        .env("HERE_NOW_LOG", "debug,!hyper,!watchexec=error")
        .env("HERE_NOW_CONFIG_FOLDER", "../conf")
        .env_if(
            jaeger,
            "JAEGER_COLLECTOR_ENDPOINT",
            "http://localhost:14268/api/traces",
        )
        .arg("run")
        .root_dir("./hn-server")
        .watchable(true, "-w ./src -i *.j2 -i *.css")
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
        Cmd::new("docker")
            .arg("run")
            .arg(tag)
            .run_it("running docker image");
    }
}
