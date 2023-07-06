use std::{path::PathBuf, process::Command};

use gumdrop::Options;
mod dev;

#[derive(Options)]
enum XtaskCommand {
    // Command names are generated from variant names.
    // By default, a CamelCase name will be converted into a lowercase,
    // hyphen-separated name; e.g. `FooBar` becomes `foo-bar`.
    //
    // Names can be explicitly specified using `#[options(name = "...")]`
    #[options(help = "run app and server for development")]
    Dev(dev::SubOptions),

    BuildServerStaticAssets(dev::SubOptions),
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
        XtaskCommand::Dev(sub) => dev::run(sub),
        XtaskCommand::BuildServerStaticAssets(_) => build_server_static_assets(),
    }
}

fn build_server_static_assets() {
    // let this_file_path = PathBuf::from(file!());
    // let root_dir = this_file_path.parent().unwrap().parent().unwrap();
    let root_dir = std::env::current_dir().unwrap();
    let child = Command::new("npx")
        .args("tailwindcss -i ./private-server.css -o ./dist/private-server.css --watch".split(' '))
        .current_dir(root_dir.join("./hn-server"))
        .spawn()
        .expect("generating")
        .wait_with_output()
        .expect("finish");
}
