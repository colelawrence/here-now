use gumdrop::Options;
use std::{
    path::PathBuf,
    process::{self, Command},
};

#[derive(Options)]
enum XtaskCommand {
    // Command names are generated from variant names.
    // By default, a CamelCase name will be converted into a lowercase,
    // hyphen-separated name; e.g. `FooBar` becomes `foo-bar`.
    //
    // Names can be explicitly specified using `#[options(name = "...")]`
    #[options(help = "build web assets development")]
    WebBuild(WebBuildOptions),
    #[options(help = "run server for development")]
    Dev(DevOptions),
    #[options(help = "lint fixes")]
    Fix(FixOptions),
    #[options(help = "generate and show docs")]
    Docs(DocsOptions),
}

// Define options for the program.
#[derive(Options)]
struct MyOptions {
    // Options here can be accepted with any command (or none at all),
    // but they must come before the command name.
    #[options(help = "print help message")]
    help: bool,
    // #[options(help = "be verbose")]
    // verbose: bool,

    // The `command` option will delegate option parsing to the command type,
    // starting at the first free argument.
    #[options(command)]
    command: Option<XtaskCommand>,
}

fn main() {
    let opts = MyOptions::parse_args_default_or_exit();
    if opts.help {
        println!("{}", opts.self_usage());
        std::process::exit(0);
    }
    let command = if let Some(command) = opts.command {
        command
    } else {
        eprintln!("Sub-command required\n\n{}", opts.self_usage());
        std::process::exit(1);
    };

    match command {
        XtaskCommand::WebBuild(opts) => web_build(opts),
        XtaskCommand::Dev(opts) => dev(opts),
        XtaskCommand::Fix(opts) => fix(opts),
        XtaskCommand::Docs(opts) => docs(opts),
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

#[derive(Options)]
struct WebBuildOptions {}
fn web_build(_: WebBuildOptions) {
    let root_dir = get_project_root_dir();
    Command::new("npx")
        .args("tailwindcss -i ./config-html-server.css -o ./src/config_html_server/build/config-html-server.css --watch".split(' '))
        .current_dir(root_dir.join("./hn-server"))
        .spawn()
        .expect("generating")
        .wait_with_output()
        .expect("exiting");
}

#[derive(Options)]
struct DevOptions {}
fn dev(_: DevOptions) {
    let root_dir = get_project_root_dir();
    let server = Command::new("cargo")
        .args("watch --watch ./src --ignore *.j2".split(' '))
        .arg("--exec")
        .arg("run ./here-now-config.toml")
        .current_dir(root_dir.join("./hn-server"))
        .spawn()
        .expect("running server with watcher");

    let web_assets = jod_thread::spawn(|| {
        web_build(WebBuildOptions {});
    });
    let server = jod_thread::spawn(|| {
        server.wait_with_output().expect("exiting");
    });

    web_assets.join();
    server.join();
}

#[derive(Options)]
struct FixOptions {}
fn fix(_: FixOptions) {
    let root_dir = get_project_root_dir();
    let output = Command::new("cargo")
        .args("fix --allow-dirty --allow-staged".split(' '))
        .current_dir(&root_dir)
        .spawn()
        .expect("fixing code")
        .wait_with_output()
        .expect("exiting");

    if !output.status.success() {
        process::exit(output.status.code().unwrap_or(1))
    }

    let output = Command::new("cargo")
        .args("fmt".split(' '))
        .current_dir(root_dir)
        .spawn()
        .expect("formatting code")
        .wait_with_output()
        .expect("exiting");

    process::exit(output.status.code().unwrap_or_default());
}

#[derive(Options)]
struct DocsOptions {}
fn docs(_: DocsOptions) {
    let root_dir = get_project_root_dir();
    let output = Command::new("cargo")
        .args("+nightly doc --open".split(' '))
        .current_dir(root_dir)
        .spawn()
        .expect("fixing code")
        .wait_with_output()
        .expect("exiting");

    process::exit(output.status.code().unwrap_or_default());
}
