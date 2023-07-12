#![allow(unused)]
use std::process::Command;

pub(crate) mod prelude {
    pub use derive_codegen::Codegen;
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FromInput<T> {
        debug_source_location: String,
        // TODO: Source location
        input: T,
    }
}

mod typography;

/// TODO
pub mod lengths {
    use crate::prelude::*;

    #[derive(Codegen, Serialize)]
    #[codegen(tags = "lengths")]
    pub struct LengthLogical {
        pixels: f64,
    }
}

mod color;

mod figma;

fn main() {
    println!("Running at {:?} ({})", std::env::current_dir(), file!());
    let current_directory =
        std::env::var("CARGO_MANIFEST_DIR").expect("getting cargo manifest directory");

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
