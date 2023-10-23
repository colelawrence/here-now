use serde::{Deserialize, Serialize};

/// User string type for display to the user.
#[derive(Debug, Serialize, Deserialize)]
pub enum UsrText {
    Literal(Vec<UsrTextPart>),
    // /// I18n key and placeholders
    // I18n(String, HashMap<String, I18nPlaceholder>),
}

// pub enum I18nPlaceholder {
//     Count(usize),
//     Text(UsrText),
//     Link(UsrLink),
//     // List(Vec<HashMap<String, I18nPlaceholder>>),
// }

impl From<String> for UsrText {
    fn from(text: String) -> Self {
        UsrText::Literal(vec![UsrTextPart::Text {
            text,
            marks: vec![],
        }])
    }
}

impl From<&str> for UsrText {
    fn from(text: &str) -> Self {
        UsrText::Literal(vec![UsrTextPart::Text {
            text: text.to_string(),
            marks: vec![],
        }])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UsrTextPart {
    Text { text: String, marks: Vec<UsrMark> },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UsrMark {
    Bold,
    Italic,
    Link(UsrLink),
    Color(UsrColor),
    Highlight(UsrColor),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UsrColor {
    /// Oklab a & b
    Oklab {
        /// a: how green/red the color is
        a: u8,
        /// b: how blue/yellow the color is
        b: u8,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UsrLink {
    /// Common external URLs
    URL { href: String },
    // // App Link? or some resource name and normalized external identifiers like "YoutubeVideoID" + ID ?
    // Resource {
    //     id: String,
    //     attrs: HashMap<String, String>,
    // },
}
