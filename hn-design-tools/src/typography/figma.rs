use std::collections::BTreeMap;

use super::{input, output, scalars};
use crate::prelude::*;

pub mod figma_scalars {
    use crate::prelude::*;

    // should this just be something like file distributable name?
    /// String that follows the base name of the family.
    /// This is used for your design programs like Figma.
    /// e.g. `" Italic"` for italics of Inter or Source Serif
    /// e.g. `" Thin"` for W100, `" Light"` for W300, `" Medium"` for W500, `" Bold"` for W700, etc.
    #[derive(Codegen, Clone, Debug, Serialize)]
    #[codegen(tags = "figma,typography")]
    #[allow(non_snake_case)]
    pub enum FigmaFontStyleRule {
        /// Suffix plus order number
        FontSuffix(String, usize),
        /// See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
        /// e.g. `"'wght' 50"`
        FontVariation(String, String),
    }
}

#[derive(Debug, Codegen)]
#[codegen(tags = "figma,typography")]
pub struct TextStyle {
    pub name: String,
    pub family_name: String,
    pub properties: Vec<output::TypographyProperty>,
}

#[derive(Debug, Codegen)]
#[codegen(tags = "figma,typography")]
pub struct FigmaTypography {
    pub core_styles: Vec<TextStyle>,
    // // I think we'd need a documentation thing for each individual token as well, right?
    // pub all_tokens: output::TypographyAllTokens,
}

pub mod figma_config {
    use crate::prelude::*;

    #[derive(Codegen, Debug)]
    #[codegen(tags = "figma,typography")]
    #[allow(non_snake_case)]
    pub struct FigmaTypographyConfig {
        // TODO: Some kind of narrowing / selections for creating types / lints for the design system
        // e.g. we should be able to swap the font families, even if the new one has fewer weights.
        /// A sort of matrice of all possible combinations of the variants
        pub FigmaTextStyles: Vec<FigmaTextStyle>,
    }

    #[derive(Codegen, Debug)]
    #[codegen(tags = "figma,typography")]
    #[allow(non_snake_case)]
    pub struct FigmaTextStyle {
        pub BaseName: String,
        pub BaseTokens: String,
        pub Description: Option<String>,
        pub Groups: Vec<FigmaTextStyleMatrixGroup>,
    }

    #[derive(Codegen, Debug)]
    #[codegen(tags = "figma,typography")]
    #[allow(non_snake_case)]
    pub struct FigmaTextStyleMatrixGroup {
        pub Description: Option<String>,
        pub Options: Vec<FigmaTextStyleMatrixOption>,
    }

    #[derive(Codegen, Debug)]
    #[codegen(tags = "figma,typography")]
    #[allow(non_snake_case)]
    pub struct FigmaTextStyleMatrixOption {
        pub Name: String,
        pub Tokens: String,
        pub Description: Option<String>,
    }
}

pub fn generate_typography_for_figma(
    all_tokens: &output::TypographyExport,
    figma_settings: &figma_config::FigmaTypographyConfig,
) -> Result<FigmaTypography> {
    Ok(FigmaTypography {
        core_styles: todo!("use all tokens to create core styles: {all_tokens:#?}"),
    })
}
