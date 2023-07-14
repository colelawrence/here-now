use crate::prelude::*;

pub mod input;
pub mod output;
pub mod css;
pub mod figma;
pub mod scalars {
    use crate::prelude::*;

    /// FontStyleRule is whatever your source configuration is using to match the environment's
    /// font styles to the desired weights and such.
    ///
    /// Note: Due to the design system not knowing the details of these, the tooling may struggle
    /// to interpolate between two possible options. Perhaps, we should leave interpolation up to
    /// the implementor?
    /// 
    /// See [figma::]
    #[derive(Codegen, Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[codegen(tags = "typography,input,output")]
    #[codegen(scalar)]
    #[serde(transparent)]
    pub struct FontStyleRule(serde_json::Value);
}

