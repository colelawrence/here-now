use crate::prelude::*;

/// depends on how you load your fonts in the application
#[derive(Codegen, Clone, Debug, Deserialize, Serialize)]
#[codegen(tags = "css,typography")]
#[allow(non_snake_case)]
pub enum CSSFontStyleRule {
    FontStyleItalics,
    FontWeightBold,
    FontWeight(usize),
    /// See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
    /// e.g. `"'wght' 50"`
    FontVariationSetting(String),
}
