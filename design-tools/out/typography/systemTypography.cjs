"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.generateTypographySettings = exports.defineTypographyConfig = void 0;
function defineTypographyConfig(config) {
    return config;
}
exports.defineTypographyConfig = defineTypographyConfig;
function generateTypographySettings(config) {
    const values = {
        FontFamilies: [],
        Fonts: [],
    };
    for (const fam of config.families) {
        if (!fam.Metrics)
            throw new TypeError(`Metrics is required, but not found for '${fam.ID}'`);
        values.FontFamilies.push({
            ID: fam.ID,
            HTMLFontFamilyName: fam.HTMLFontFamilyName,
            FigmaFontFamilyName: fam.FigmaFontFamilyName,
        });
        for (const size of config.sizes) {
            values.Fonts.push({
                ID: `${fam.ID}-${size.ID}`,
                Value: {
                    FontFamily: { ref$: fam.ID },
                    ...calculateFontSizes(fam.Metrics, fam, size),
                },
            });
        }
    }
    return values;
}
exports.generateTypographySettings = generateTypographySettings;
/**
 * Get font size + line height for a given sizing configuration
 */
function calculateFontSizes(metrics, family, config) {
    // console.error(metrics);
    const recip = metrics.unitsPerEm / metrics.capHeight;
    const alignToGrid = 4;
    const fontUnitPx = config.CapHeight * recip;
    const lineHeightPx = fontUnitPx * family.TargetRelativeLineHeight;
    const lineHeightAlignedPx = Math.round(lineHeightPx / alignToGrid) * alignToGrid;
    // cap / unit = cap / unit
    const { a, b, c } = family.DynMetrics;
    const z = fontUnitPx;
    const tracking = a + b * Math.pow(Math.E, c * z);
    return {
        FontSize: px(fontUnitPx),
        LineHeight: px(lineHeightAlignedPx),
        Tracking: em(tracking),
    };
}
function px(n) {
    /*
     * Rounding all values to a precision of `4` based on discovering that browser
     * implementations of layout units fall between 1/60th and 1/64th of a pixel.
     * Reference: https://trac.webkit.org/wiki/LayoutUnit
     * (above wiki also mentions Mozilla - https://trac.webkit.org/wiki/LayoutUnit#Notes)
     */
    return `${n.toFixed(4).replace(/\.0+$/, "")}px`;
}
function em(n) {
    return `${n.toFixed(4).replace(/\.0+$/, "")}em`;
}
//# sourceMappingURL=systemTypography.cjs.map