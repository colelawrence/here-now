#![allow(unused)]
use std::process::Command;

pub(crate) mod prelude {
    pub use derive_codegen::Codegen;
    pub use serde::{Deserialize, Serialize};
}

pub(crate) mod input {
    use crate::prelude::*;

    #[derive(Debug, Deserialize, Codegen)]
    #[codegen(tags = "input")]
    pub struct SystemInput {
        color_palette: crate::color::input::ColorPalette,
        typography: crate::typography::input::Typography,
    }
}

mod typography;

mod cli;

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
    cli::run();
}
