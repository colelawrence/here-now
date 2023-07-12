use crate::prelude::*;

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub struct Typography {
    Families: Vec<FontFamilyInfo>,
    // Use case for multiple size scales?
    /// Scaling strategy for different font-sizes.
    FontSizeScale: FontSizeScale,
    TextRoles: Vec<TextRole>,
    // TODO: Some kind of narrowing / selections for creating types / lints for the design system
    // e.g. we should be able to swap the font families, even if the new one has fewer weights.
    /// A sort of matrice of all possible combinations of the variants
    FigmaTextStyles: Vec<FigmaTextStyle>,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub struct TextRole {
    /// e.g. `"ui"` or `"content"`
    Token: String,
    /// e.g. `"Inter"` or `"Merriweather"`, this must be described
    /// in [Typography] "Families".
    FamilyBaseName: String,
    /// e.g. tight = `1.272` or spacious = `1.61803`
    TargetRelativeLineHeight: f64,
    /// Also called "letter spacing," this is the space between letters for different sizes
    TrackingRule: FontFamilyTrackingRule,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
struct FigmaTextStyle {
    BaseName: String,
    BaseTokens: String,
    Description: Option<String>,
    Groups: Vec<FigmaTextStyleMatrixGroup>,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
struct FigmaTextStyleMatrixGroup {
    Description: Option<String>,
    Options: Vec<FigmaTextStyleMatrixOption>,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
struct FigmaTextStyleMatrixOption {
    Name: String,
    Tokens: String,
    Description: Option<String>,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub struct FontFamilyInfo {
    /// e.g. `"Inter"` or `"Merriweather"`
    BaseName: String,
    /// e.g. `"Inter"` or `"Merriweather"`
    CSSFontFamilyName: Option<String>,
    /// e.g. `"system-ui", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Arial", "sans-serif"`
    CSSFontFamilyFallbacks: Vec<String>,
    Weights: Vec<FamilyWeightRule>,
    ItalicOption: Option<FontVariantInfo>,
    /// e.g. metrics from @capsize/metrics
    Metrics: FontFamilyMetrics,
    // TODO: Variable variations ?
}

/// One variant such as applying italic or a weight.
///
/// Progress 2/10:
///  * It's a bit tricky to describe whether this variant requires a font suffix
///    versus it requiring a variable axes tweak, etc. Or if these might be
///    different between your design program and the web distributable.
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
struct FontVariantInfo {
    // should this just be something like file distributable name?
    /// String that follows the base name of the family.
    /// This is used for your design programs like Adobe Illustrator or Figma.
    /// e.g. `" Italic"` for italics of Inter or Source Serif
    /// e.g. `" Thin"` for W100, `" Light"` for W300, `" Medium"` for W500, `" Bold"` for W700, etc.
    Suffix: Option<String>,
    /// depends on how you load your fonts in the application
    CSSRule: CSSRule,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
enum CSSRule {
    FontStyleItalics,
    FontWeightBold,
    FontWeight(usize),
    /// See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
    /// e.g. `"'wght' 50"`
    FontVariationSetting(String),
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
enum FamilyWeightRule {
    /// e.g. `"Thin"` for Inter.
    W100(FontVariantInfo),
    /// e.g. `"Extra Light"` for Inter.
    W200(FontVariantInfo),
    /// e.g. `"Light"` for Inter.
    W300(FontVariantInfo),
    /// e.g. `"Regular"` for Inter.
    W400(FontVariantInfo),
    /// e.g. `"Medium"` for Inter.
    W500(FontVariantInfo),
    /// e.g. `"Semi Bold"` for Inter.
    W600(FontVariantInfo),
    /// e.g. `"Bold"` for Inter.
    W700(FontVariantInfo),
    /// e.g. `"Extra Bold"` for Inter.
    W800(FontVariantInfo),
    /// e.g. `"Black"` for Inter.
    W900(FontVariantInfo),
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub struct FontSizeScale {
    FontSizes: Vec<FontSizeRel>,
    Equation: FontSizeEquation,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub struct FontSizeRel {
    /// e.g. `"xs"`, `"sm"`, `"base"`, `"lg"`, etc.
    Token: String,
    /// e.g. `-2`, `-1`, `0`, `1`, etc.
    Rel: f64,
}

/// WIP: Based on @capsizecss/metrics
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub struct FontFamilyMetrics {
    familyName: String,
    category: String,
    capHeight: f64,
    ascent: f64,
    descent: f64,
    lineGap: f64,
    unitsPerEm: f64,
    xHeight: f64,
    xWidthAvg: f64,
}

/// WIP: Based on @capsizecss/metrics
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub enum FontFamilyTrackingRule {
    /// Determine your tracking goals via a page like https://rsms.me/inter/dynmetrics/
    DynMetrics { a: f64, b: f64, c: f64 },
}

/// WIP: Based on ratioInterval
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub enum FontSizeEquation {
    Multiplier {
        base_px: f64,
        /// Popular options would be `1.27201965` (sqrt(Golden Ratio)), or `1.4`
        /// These would indicate the scale applied with each successive increase
        /// of the font size base number.
        multiplier: f64,
    },
}

impl FontSizeEquation {
    fn compute_font_size_px(&self, rel: f64) -> f64 {
        match self {
            FontSizeEquation::Multiplier {
                base_px,
                multiplier,
            } => base_px * (multiplier.powf(rel)),
        }
    }
}
