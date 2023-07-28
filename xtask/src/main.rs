use clap::{self, Parser};
use devx_cmd::Cmd;
use std::{
    path::PathBuf,
    process::{self, Command, Output},
};

#[derive(Debug, Parser)]
enum Args {
    // Command names are generated from variant names.
    // By default, a CamelCase name will be converted into a lowercase,
    // hyphen-separated name; e.g. `FooBar` becomes `foo-bar`.
    //
    // Names can be explicitly specified using `#[options(name = "...")]`
    // #[clap(info = "build web assets development")]
    WebBuild,
    // #[clap(help = "run server for development")]
    Dev,
    // #[clap(help = "lint fixes")]
    Fix,
    // #[options(help = "generate and show docs")]
    Doc,
}

fn main() {
    let args = Args::parse();

    match args {
        Args::WebBuild => web_build(),
        Args::Dev => dev(),
        Args::Fix => fix(),
        Args::Doc => doc(),
    }
}

fn get_project_root_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR")
        .and_then(|value| PathBuf::from(value).parent().map(PathBuf::from))
        .expect("CARGO_MANIFEST_DIR was defined")
}

fn web_build() {
    let root_dir = get_project_root_dir();

    eprintln!("Building TypeScript");
    let status = Command::new("npx")
        .args("tsc -p ./design-tools/tsconfig.json".split(' '))
        .current_dir(&root_dir)
        .spawn()
        .expect("building design tools")
        .wait_with_output()
        .expect("exiting");

    expect_success(&status);

    // make this a watch thing, too
    Cmd::new("deno")
        .args("run -A ./svelte-tools/compile-svelte.ts ./hn-server/templates".split(' '))
        .current_dir(&root_dir)
        .run()
        .expect("built templates");

    Command::new("npx")
        .args("tailwindcss -i hn-server/config-html-server.css -o hn-server/src/config_html_server/build/config-html-server.css --watch".split(' '))
        .current_dir(root_dir)
        .spawn()
        .expect("generating")
        .wait_with_output()
        .expect("exiting");
}

fn dev() {
    let root_dir = get_project_root_dir();
    let server = Command::new("cargo")
        .env("RUST_LOG", "debug,!hyper")
        .env("HERE_NOW_CONFIG_FOLDER", "../conf")
        .args("watch --watch ./src --ignore *.j2 --ignore *.css".split(' '))
        .arg("--exec")
        .arg("run")
        .current_dir(root_dir.join("./hn-server"))
        .spawn()
        .expect("running server with watcher");

    let web_assets = jod_thread::spawn(|| {
        web_build();
    });
    let server = jod_thread::spawn(|| {
        server.wait_with_output().expect("exiting");
    });

    web_assets.join();
    server.join();
}

fn fix() {
    let root_dir = get_project_root_dir();
    let output = Command::new("cargo")
        .args("fix --allow-dirty --allow-staged".split(' '))
        .current_dir(&root_dir)
        .spawn()
        .expect("fixing code")
        .wait_with_output()
        .expect("exiting");

    expect_success(&output);

    let output = Command::new("cargo")
        .args("fmt".split(' '))
        .current_dir(root_dir)
        .spawn()
        .expect("formatting code")
        .wait_with_output()
        .expect("exiting");

    expect_success(&output);
}

fn doc() {
    let root_dir = get_project_root_dir();
    let output = Command::new("cargo")
        .args("+nightly doc --workspace --open --target aarch64-apple-darwin".split(' '))
        // ensure not to get wasm bindgen stuff
        // the server and the desktop should work on this architecture
        .args("--target aarch64-apple-darwin".split(' '))
        .current_dir(root_dir)
        .spawn()
        .expect("fixing code")
        .wait_with_output()
        .expect("exiting");

    expect_success(&output);
}

fn expect_success(output: &Output) {
    if !output.status.success() {
        process::exit(output.status.code().unwrap_or(1))
    }
}
