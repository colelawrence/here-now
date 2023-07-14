use std::{collections::BTreeMap, f64::consts::E};

use crate::prelude::*;

use super::scalars;

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub struct Typography {
    pub Families: Vec<FontFamilyInfo>,
    // Use case for multiple size scales?
    /// Scaling strategy for different font-sizes.
    pub FontSizeScale: FontSizeScale,
    pub TextRoles: Vec<TextRole>,
    pub Extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub struct TextRole {
    /// e.g. `"ui"` or `"content"`
    pub Token: String,
    /// e.g. `"Inter"` or `"Merriweather"`, this must be described
    /// in [Typography] "Families".
    pub FamilyBaseName: String,
    /// e.g. tight = `1.272` or spacious = `1.61803`
    pub LineHeightRule: FontFamilyLineHeightRule,
    /// Also called "letter spacing," this is the space between letters for different sizes
    pub TrackingRule: FontFamilyTrackingRule,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub struct FontFamilyInfo {
    /// e.g. `"Inter"` or `"Merriweather"`
    pub BaseName: String,
    /// e.g. `"Inter"` or `"Merriweather"`
    pub CSSFontFamilyName: Option<String>,
    /// e.g. `"system-ui", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Arial", "sans-serif"`
    pub CSSFontFamilyFallbacks: Vec<String>,
    pub Weights: Vec<FamilyWeightRule>,
    pub ItalicOption: Option<scalars::FontStyleRule>,
    /// e.g. metrics from @capsize/metrics
    pub Metrics: FontFamilyMetrics,
    // TODO: Variable variations ?
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub struct FamilyWeightRule {
    /// Number between 0 and 1000
    /// For example, for "Inter":
    ///  * 100 is `"Thin"`
    ///  * 200 is `"Extra Light"`
    ///  * 300 is `"Light"`
    ///  * 400 is `"Regular"`
    ///  * 500 is `"Medium"`
    ///  * 600 is `"Semi Bold"`
    ///  * 700 is `"Bold"`
    ///  * 800 is `"Extra Bold"`
    ///  * 900 is `"Black"`
    pub Weight: usize,
    /// A scalar depending on the requirements of the different generators you're aiming to support
    pub FontStyleRule: scalars::FontStyleRule,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub struct FontSizeScale {
    pub FontSizes: Vec<FontSizeRel>,
    pub Equation: FontSizeEquation,
    /// For example, `1.0` for aligning to 1px.
    pub AlignCapHeightPxOption: Option<f64>,
    /// For example, `4.0` for aligning line-heights to 4px.
    pub AlignLineHeightPxOption: Option<f64>,
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub struct FontSizeRel {
    /// e.g. `"xs"`, `"sm"`, `"base"`, `"lg"`, etc.
    pub Token: String,
    /// e.g. `-2`, `-1`, `0`, `1`, etc.
    pub Rel: f64,
}

/// WIP: Based on @capsizecss/metrics
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub struct FontFamilyMetrics {
    pub familyName: String,
    pub category: String,
    pub capHeight: f64,
    pub ascent: f64,
    pub descent: f64,
    pub lineGap: f64,
    pub unitsPerEm: f64,
    pub xHeight: f64,
    pub xWidthAvg: f64,
}

/// WIP: Based on @capsizecss/metrics
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub enum FontFamilyTrackingRule {
    /// Determine your tracking goals via a page like https://rsms.me/inter/dynmetrics/
    DynMetrics { a: f64, b: f64, c: f64 },
}

impl FontFamilyTrackingRule {
    pub fn compute_font_tracking_px(&self, font_size_px: f64) -> f64 {
        match self {
            // reference https://rsms.me/inter/dynmetrics/
            FontFamilyTrackingRule::DynMetrics { a, b, c } => a + b * E.powf(c * font_size_px),
        }
    }
}

#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
#[allow(non_snake_case)]
pub enum FontFamilyLineHeightRule {
    /// Determine your tracking goals via a page like https://rsms.me/inter/dynmetrics/
    FontSizePxMultipler { multiplier: f64 },
}

impl FontFamilyLineHeightRule {
    pub fn compute_line_height_px(&self, font_size_px: f64, align_px_opt: Option<f64>) -> f64 {
        match self {
            // reference https://rsms.me/inter/dynmetrics/
            FontFamilyLineHeightRule::FontSizePxMultipler { multiplier } => {
                align_to(multiplier * font_size_px, align_px_opt)
            }
        }
    }
}

/// WIP: Based on ratioInterval
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "typography,input")]
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
    pub fn compute_cap_height_px(&self, rel: f64, align_px_opt: Option<f64>) -> f64 {
        match self {
            FontSizeEquation::Multiplier {
                base_px,
                multiplier,
            } => align_to(base_px * (multiplier.powf(rel)), align_px_opt),
        }
    }
}
