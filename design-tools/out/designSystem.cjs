"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getColorCSSProperties = exports.getAllColorsKebab = exports.createArtpromptColorsFromHTMLImage = exports.createArtpromptColorsFromSourceHex = exports.designSystemTypographySettings = exports.designSystemColorSettings = exports.artpromptExtendedColors = exports.hexFromArgb = void 0;
const material_color_utilities_1 = require("@artprompt/material-color-utilities");
const no_case_1 = require("no-case");
const design_system_settings_cjs_1 = require("./design-system-settings.cjs");
const systemColors_cjs_1 = require("./systemColors.cjs");
exports.default = "Artprompt design";
var material_color_utilities_2 = require("@artprompt/material-color-utilities");
Object.defineProperty(exports, "hexFromArgb", { enumerable: true, get: function () { return material_color_utilities_2.hexFromArgb; } });
var design_system_settings_cjs_2 = require("./design-system-settings.cjs");
Object.defineProperty(exports, "artpromptExtendedColors", { enumerable: true, get: function () { return design_system_settings_cjs_2.artpromptExtendedColors; } });
Object.defineProperty(exports, "designSystemColorSettings", { enumerable: true, get: function () { return design_system_settings_cjs_2.designSystemColorSettings; } });
Object.defineProperty(exports, "designSystemTypographySettings", { enumerable: true, get: function () { return design_system_settings_cjs_2.designSystemTypographySettings; } });
function createArtpromptColorsFromSourceHex(hex, extended) {
    return (0, systemColors_cjs_1.generateColorSettings)({
        Primary: { Hex: hex },
        extended: extended ?? design_system_settings_cjs_1.artpromptExtendedColors,
    });
}
exports.createArtpromptColorsFromSourceHex = createArtpromptColorsFromSourceHex;
function createArtpromptColorsFromHTMLImage(image, extended) {
    return (0, systemColors_cjs_1.generateColorSettingsFromHTMLImage)(image, {
        extended: extended ?? design_system_settings_cjs_1.artpromptExtendedColors,
    });
}
exports.createArtpromptColorsFromHTMLImage = createArtpromptColorsFromHTMLImage;
function getAllColorsKebab({ mvpTheme }, options) {
    const scheme = options.dark ? mvpTheme.schemes.dark : mvpTheme.schemes.light;
    return [
        ...Object.entries(scheme.toJSON()).map(([name, argb]) => {
            const kebab = (0, no_case_1.noCase)(name, { delimiter: "-" });
            return { id: kebab, argb: argb, ext: false };
        }),
        ...mvpTheme.customColors.flatMap((custom) => {
            const kebab = (0, no_case_1.noCase)(custom.color.name, { delimiter: "-" });
            const scheme = options.dark ? custom.dark : custom.light;
            return [
                { id: kebab, argb: scheme.color, ext: true },
                { id: `on-${kebab}`, argb: scheme.onColor, ext: true },
                { id: `${kebab}-container`, argb: scheme.colorContainer, ext: true },
                { id: `on-${kebab}-container`, argb: scheme.onColorContainer, ext: true },
            ];
        }),
    ];
}
exports.getAllColorsKebab = getAllColorsKebab;
function getColorCSSProperties(colorSettings, options) {
    const kebab = getAllColorsKebab(colorSettings, options);
    return kebab.map((a) => ({
        twID: `${a.ext ? "ext" : "sys"}-${a.id}`,
        cssProperty: `--${a.ext ? "ext" : "sys"}-color-${a.id}`,
        valueHex: (0, material_color_utilities_1.hexFromArgb)(a.argb),
        valueRGBSpaced: `${(0, material_color_utilities_1.redFromArgb)(a.argb)} ${(0, material_color_utilities_1.greenFromArgb)(a.argb)} ${(0, material_color_utilities_1.blueFromArgb)(a.argb)}`,
    }));
}
exports.getColorCSSProperties = getColorCSSProperties;
//# sourceMappingURL=designSystem.cjs.map