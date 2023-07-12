use crate::prelude::*;
#[derive(Codegen, Serialize)]
#[codegen(tags = "figma")]
pub enum FigmaVariable {
    Color(FigmaColor),
    Length(FigmaLength),
}

#[derive(Codegen, Serialize)]
#[codegen(tags = "figma")]
pub struct Named<T> {
    tailwind_id: String,
    name: String,
    description: Option<String>,
    value: T,
}

#[derive(Codegen, Serialize)]
#[codegen(tags = "figma")]
#[serde(transparent)]
pub struct Pixels(f64);

#[derive(Codegen, Serialize)]
#[codegen(tags = "figma")]
pub struct TextStyle {
    /// e.g. `"Inter"`
    font_family: String,
    /// e.g. `"Regular"`
    font_style: String,
    /// e.g. `12`
    font_size: Pixels,
    /// e.g. `15.5600004196167`
    line_height: Pixels,
    /// e.g. `1.0202931111111`
    letter_spacing: Pixels,
}

#[derive(Codegen, Serialize)]
#[codegen(tags = "figma")]
pub struct DesignSystem {
    variables: Vec<Named<FigmaVariable>>,
    text_styles: Vec<Named<TextStyle>>,
}

#[derive(Codegen, Serialize)]
#[codegen(tags = "figma")]
pub struct FigmaColor {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

#[derive(Codegen, Serialize)]
#[codegen(tags = "figma")]
#[serde(transparent)]
pub struct FigmaLength(f64);
