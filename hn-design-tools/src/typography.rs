use crate::prelude::*;

pub mod input;

pub struct FontMetrics {}

pub struct FontFamilyKey(String);

pub struct FontFamilyRule {
    key: FontFamilyKey,
    metrics: FontMetrics,
    documentation: Option<String>,
    documentation_links: Vec<(String, String)>,
    // meta
    css: Option<String>,
    figma: Option<String>,
    file_name: Option<String>,
    other: Vec<(String, String)>,
}

pub struct Name {
    /// e.g. `"ui"` or `"red"`
    code: String,
    /// e.g. `"UI Text"` or `"Red"`
    display: Option<String>,
}

pub struct NamedSize {
    name: Name,
    size: f64,
}

pub struct TypographyRules {
    font_family: FontFamilyRule,
    font_sizes: Vec<NamedSize>,
    // ... font metrics available
    font_size: FontSizeRule,
    // Expressions access fontSize
    letter_spacing: LetterSpacingRule,
    line_height: LineHeightRule,
}

pub enum FontLength {
    Pixels(f64),
}

/// https://docs.rs/mexprp/latest/mexprp/
pub struct FontExpression(String);

pub enum FontSizeRule {
    Expression(FontExpression),
    CapSize { align_to: FontLength },
}

pub enum LineHeightRule {
    Expression(FontExpression),
    CapSize { align_to: FontLength },
}

pub enum LetterSpacingRule {
    Constant(FontLength),
    Expression(FontExpression),
    /// https://rsms.me/inter/dynmetrics/
    DynMetrics {
        a: f64,
        b: f64,
        c: f64,
    },
}
