//! See https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
use std::{os::unix::process, process::Command};

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
                        .arg("--importScalarsFrom=./scalars.ts")
                        .arg(r#"--prependText=type Value = unknown;"#)
                        .current_dir(&current_directory),
                )
                .with_output_path("./example")
                .write()
                .print();

            derive_codegen::Generation::for_tag("output")
                .as_arg_of(
                    Command::new("deno")
                        .arg("run")
                        .arg("../vendor/derive-codegen/typescript-generator/generate-typescript.ts")
                        .arg("--includeLocationsRelativeTo=../../")
                        .arg("--fileName=output.gen.ts")
                        .arg("--importScalarsFrom=./scalars.ts")
                        .arg(r#"--prependText=type Value = unknown;"#)
                        .current_dir(&current_directory),
                )
                .with_output_path("./example")
                .write()
                .print();

            derive_codegen::Generation::for_tag("figma")
                .as_arg_of(
                    Command::new("deno")
                        .arg("run")
                        .arg("../vendor/derive-codegen/typescript-generator/generate-typescript.ts")
                        .arg("--includeLocationsRelativeTo=../../")
                        .arg("--fileName=figma.gen.ts")
                        .arg("--importScalarsFrom=./scalars.ts")
                        .arg(r#"--prependText=import { TypographyProperty } from "./output.gen.ts";
type Value = unknown;"#)
                        .current_dir(&current_directory),
                )
                .with_output_path("./example")
                .write()
                .print();
        }
        Commands::Example => {
            let child = Command::new("deno")
                .arg("run")
                .arg("./example/get-settings-json-to-stdout.ts")
                .current_dir(&current_directory)
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

            let all_tokens: crate::typography::output::TypographyExport =
                crate::typography::output::generate_typography_all_tokens(&found.typography)
                    .expect("generating all tokens")
                    .into();

            let output = Command::new("deno")
                .arg("run")
                .arg("./example/generate-tailwind-json-from-arg.ts")
                .arg(serde_json::to_string(&all_tokens).unwrap())
                .current_dir(&current_directory)
                .spawn()
                .expect("starting deno")
                .wait_with_output()
                .expect("exiting deno");

            if !output.status.success() {
                std::process::exit(output.status.code().unwrap_or_default());
            }
        }
    }
}
