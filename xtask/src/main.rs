use devx_cmd::{run, Cmd};
use clap::{self, Parser};
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
    Docs,
}

fn main() {
    let args = Args::parse();

    match args {
        Args::WebBuild => web_build(),
        Args::Dev => dev(),
        Args::Fix => fix(),
        Args::Docs => docs(),
    }
}

fn get_project_root_dir() -> PathBuf {
    std::env::vars_os()
        .into_iter()
        .filter_map(|(key, value)| (key == "CARGO_MANIFEST_DIR").then_some(value))
        .next()
        .and_then(|value| PathBuf::from(value).parent().map(PathBuf::from))
        .unwrap_or_else(|| std::env::current_dir().unwrap())
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
        .args("watch --watch ./src --ignore *.j2 --ignore *.css".split(' '))
        .arg("--exec")
        .arg("run ./here-now-config.toml")
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

fn docs() {
    let root_dir = get_project_root_dir();
    let output = Command::new("cargo")
        .args("+nightly doc --open".split(' '))
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
