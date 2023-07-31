use clap::{self, Parser};
use std::{path::PathBuf, process::Command};

use command_ext::CommandExt;
mod command_ext;

#[derive(Debug, Parser)]
enum Args {
    /// Build web assets for development
    WebBuild { watch: bool },
    /// Run server for development
    Dev,
    /// Assorted lint fixes
    Fix,
    /// Generate and show docs
    Doc,
}

fn main() {
    let args = Args::parse();

    match args {
        Args::WebBuild { watch } => web_build(watch),
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

fn web_build(watch: bool) {
    let root_dir = get_project_root_dir();

    eprintln!("Building TypeScript");

    Command::new("npx")
        .args("tsc -p ./design-tools/tsconfig.json".split(' '))
        .current_dir(&root_dir)
        .run("build design tools like TailwindCSS settings");

    // make this a watch thing, too
    Command::new("deno")
        .args("run -A ./svelte-tools/compile-svelte.ts ./hn-server/templates".split(' '))
        .arg_if(watch, "--watch")
        .current_dir(&root_dir)
        .run("built svelte templates");

    Command::new("npx")
        .args("tailwindcss -i hn-server/config-html-server.css -o hn-server/src/config_html_server/build/config-html-server.css".split(' '))
        .arg_if(watch, "--watch")
        .current_dir(root_dir)
        .run("tailwindcss compilation");
}

fn dev() {
    let server = Command::new("cargo")
        .env("RUST_LOG", "debug,!hyper")
        .env("HERE_NOW_CONFIG_FOLDER", "../conf")
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
    Command::new("cargo")
        .args("fix --allow-dirty --allow-staged".split(' '))
        .root_dir(".")
        .run("fixing rust code in workspace");

    Command::new("cargo")
        .args("fmt".split(' '))
        .root_dir(".")
        .run("format rust files in workspace");
}

fn doc() {
    Command::new("cargo")
        .args("+nightly doc --workspace --open --target aarch64-apple-darwin".split(' '))
        // ensure not to get wasm bindgen stuff
        // the server and the desktop should work on this architecture
        .args("--target aarch64-apple-darwin".split(' '))
        .root_dir(".")
        .run("geenrate and open docs");
}
