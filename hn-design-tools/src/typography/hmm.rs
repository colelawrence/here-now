/// Future: Consider other sizing strategies like x-height, optical size, etc.
#[derive(Codegen, Debug, Deserialize)]
#[codegen(tags = "input,typography")]
#[allow(non_snake_case)]
pub enum FontSizeRule {
    /// Size for an exact cap height
    CapHeightByPx(f64),
    /// Expect a [FontSizingEquation]
    CapHeightByEquation(f64),
}
