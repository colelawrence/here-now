//! See https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
use std::process::Command;

use clap::{Parser, Subcommand};

use crate::input;
/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate from example
    Example,
    /// Generate code from example
    ExampleCodegen,
}

pub(crate) fn run() {
    let cli = Cli::parse();
    let current_directory =
        std::env::var("CARGO_MANIFEST_DIR").expect("getting cargo manifest directory");

    match cli.command {
        Commands::ExampleCodegen => {
            derive_codegen::Generation::for_tag("input")
                .as_arg_of(
                    Command::new("deno")
                        .arg("run")
                        .arg("../vendor/derive-codegen/typescript-generator/generate-typescript.ts")
                        .arg("--includeLocationsRelativeTo=../../")
                        .arg("--fileName=input.gen.ts")
                        .current_dir(current_directory),
                )
                .with_output_path("./example")
                .write()
                .print();
        }
        Commands::Example => {
            let child = Command::new("deno")
                .arg("run")
                .arg("./example/get-settings-json-to-stdout.ts")
                .current_dir(current_directory)
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("starting deno command");

            let output = child.wait_with_output().expect("exiting deno command");

            if (!output.status.success()) {
                use std::io::Write;
                eprintln!("Failed to run deno command");
                std::process::exit(output.status.code().unwrap_or_default());
            }

            let found = serde_json::from_slice::<input::SystemInput>(&output.stdout).expect(
                "parsing system input from stdout of example/get-settings-json-to-stdout.ts",
            );

            println!("System settings: {found:#?}");
        }
    }
}
