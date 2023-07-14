use std::collections::{BTreeMap, BTreeSet};

use crate::prelude::*;

use super::{input, scalars};

#[derive(Debug, Serialize, Codegen)]
#[codegen(tags = "typography,output")]
pub struct TypographyExport {
    properties: Vec<TypographyProperty>,
    tokens: Vec<(BTreeSet<String>, Vec<usize>)>,
    /// For example, `{"figma": FigmaTypographyConfig, "tailwind": TailwindTypographyConfig}`
    extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Default)]
pub struct TypographyTokensCollector(BTreeMap<BTreeSet<String>, Vec<TypographyProperty>>);

impl From<TypographyTokensCollector> for TypographyExport {
    fn from(value: TypographyTokensCollector) -> Self {
        let mut result = TypographyExport {
            properties: Vec::new(),
            tokens: Vec::new(),
            extensions: BTreeMap::new(),
        };

        for (tokens, values) in value.0.into_iter() {
            let value_idxs = values.into_iter().map(|value| {
                let found_idx_opt = result
                    .properties
                    .iter()
                    .enumerate()
                    .find(|(_, v)| *v == &value)
                    .map(|(idx, _)| idx);
                match found_idx_opt {
                    Some(found_idx) => found_idx,
                    None => {
                        result.properties.push(value);
                        result.properties.len() - 1
                    }
                }
            });

            result.tokens.push((tokens, value_idxs.collect()));
        }

        result
    }
}

#[derive(Debug, Serialize, Codegen, PartialEq)]
#[codegen(tags = "typography,output")]
pub enum TypographyProperty {
    FontFamily { family_name: Cow<'static, str> },
    LineHeight { px: f64 },
    FontSize { px: f64 },
    LetterSpacing { px: f64 },
    FontStyle(scalars::FontStyleRule),
    // /// Hmm
    // Variable { key: String, value: f64 },
}

impl TypographyTokensCollector {
    fn push(&mut self, filter: &[&str], value: TypographyProperty) -> Result<()> {
        self.push_all(filter, [value])?;
        // self.0.push(TokenValue {
        //     filter: filter.iter().cloned().map(String::from).collect(),
        //     value,
        // });
        Ok(())
    }
    fn push_all(
        &mut self,
        filter: &[&str],
        values: impl IntoIterator<Item = TypographyProperty>,
    ) -> Result<()> {
        self.0
            .entry(filter.iter().cloned().map(String::from).collect())
            .or_default()
            .extend(values);
        // for value in values {
        //     self.push(filter, value);
        // }
        Ok(())
    }
}

pub fn generate_typography_all_tokens(
    input: &input::Typography,
) -> Result<TypographyTokensCollector> {
    let mut all_tokens = TypographyTokensCollector::default();

    let mut roles_by_family_name = BTreeMap::<&str, Vec<String>>::new();

    for text_role in input.TextRoles.iter() {
        let family_name = Cow::<'static, str>::Owned(text_role.FamilyBaseName.to_string());
        roles_by_family_name
            .entry(&text_role.FamilyBaseName)
            .or_default()
            .push(text_role.Token.clone());

        all_tokens.push(
            &["text", &text_role.Token],
            TypographyProperty::FontFamily {
                family_name: family_name.clone(),
            },
        )?;

        let family_info = input.Families.iter().find(|f| f.BaseName.as_str() == &family_name).ok_or_else(|| anyhow::anyhow!("Family name ({family_name:?}) used for text role ({:?}) does not have an entry in `Families`", text_role.Token))?;

        let recip = family_info.Metrics.unitsPerEm / family_info.Metrics.capHeight;

        for size in input.FontSizeScale.FontSizes.iter() {
            let cap_height_px = input
                .FontSizeScale
                .Equation
                .compute_cap_height_px(size.Rel, input.FontSizeScale.AlignCapHeightPxOption);

            let font_size_px = recip * cap_height_px;

            let tracking_px = text_role
                .TrackingRule
                .compute_font_tracking_px(font_size_px);

            let line_height_px = text_role
                .LineHeightRule
                .compute_line_height_px(font_size_px, input.FontSizeScale.AlignLineHeightPxOption);

            all_tokens.push_all(
                &["text", &text_role.Token, &size.Token],
                [
                    TypographyProperty::FontSize { px: font_size_px },
                    TypographyProperty::LetterSpacing { px: tracking_px },
                    TypographyProperty::LineHeight { px: line_height_px },
                ],
            )?;
        }
    }

    for family in input.Families.iter() {
        let role_tokens = match roles_by_family_name.get(family.BaseName.as_str()) {
            Some(roles) => roles,
            None => continue,
        };
        for role_token in role_tokens.iter() {
            for weight in family.Weights.iter() {
                all_tokens.push_all(
                    &[role_token.as_str(), &format!("W{}", weight.Weight)],
                    [TypographyProperty::FontStyle(weight.FontStyleRule.clone())],
                )?;
            }
        }
    }

    Ok(all_tokens)
}
