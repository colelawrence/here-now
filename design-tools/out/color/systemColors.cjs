"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.generateColorSettingsFromHTMLImage = exports.generateColorSettings = void 0;
const material_color_utilities_1 = require("@artprompt/material-color-utilities");
function generateColorSettings(colorConfig) {
    // Get the theme from a hex color
    return {
        config: colorConfig,
        mvpTheme: (0, material_color_utilities_1.themeFromSourceColor)((0, material_color_utilities_1.argbFromHex)(colorConfig.Primary.Hex), [...customColorsFromColorConfig(colorConfig)]),
    };
}
exports.generateColorSettings = generateColorSettings;
async function generateColorSettingsFromHTMLImage(image, colorConfig) {
    const theme = await (0, material_color_utilities_1.themeFromImage)(image, [...customColorsFromColorConfig(colorConfig)]);
    // Get the theme from a hex color
    return {
        config: {
            Primary: { Hex: (0, material_color_utilities_1.hexFromArgb)(theme.source) },
            ...colorConfig,
        },
        mvpTheme: theme,
    };
}
exports.generateColorSettingsFromHTMLImage = generateColorSettingsFromHTMLImage;
function customColorsFromColorConfig(colorConfig) {
    return colorConfig.extended.map((ext) => ({
        name: ext.ID,
        value: (0, material_color_utilities_1.argbFromHex)(ext.Seed.Hex),
        // I think this is "harmonize"
        blend: true,
    }));
}
//# sourceMappingURL=systemColors.cjs.map