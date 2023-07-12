use crate::prelude::*;

/// TODO: adjust API/Configuration to be more accomodating
/// of other color generation strategies than just Material You.
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,color")]
#[allow(non_snake_case)]
pub struct ColorPalette {
    Primary: InputColor,
    Extensions: Vec<ColorExtension>,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,color")]
#[allow(non_snake_case)]
pub struct ColorExtension {
    /// e.g. `"blue"`
    Token: String,
    Source: SourceColor,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,color")]
pub enum SourceColor {
    SimilarTo(InputColor),
    Exactly(InputColor),
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,color")]
pub enum InputColor {
    Hex(String),
}
