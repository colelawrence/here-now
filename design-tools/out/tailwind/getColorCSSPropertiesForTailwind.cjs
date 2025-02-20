"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getColorCSSPropertiesForTailwind = void 0;
const material_color_utilities_1 = require("@material/material-color-utilities");
const getAllColorsKebab_cjs_1 = require("../color/getAllColorsKebab.cjs");
function getColorCSSPropertiesForTailwind(colorSettings, options) {
    const kebab = (0, getAllColorsKebab_cjs_1.getAllColorsKebab)(colorSettings, options);
    return kebab.map((a) => ({
        twID: `${a.ext ? "ext" : "sys"}-${a.id}`,
        cssProperty: `--${a.ext ? "ext" : "sys"}-color-${a.id}`,
        valueHex: (0, material_color_utilities_1.hexFromArgb)(a.argb),
        valueRGBSpaced: `${(0, material_color_utilities_1.redFromArgb)(a.argb)} ${(0, material_color_utilities_1.greenFromArgb)(a.argb)} ${(0, material_color_utilities_1.blueFromArgb)(a.argb)}`,
    }));
}
exports.getColorCSSPropertiesForTailwind = getColorCSSPropertiesForTailwind;
//# sourceMappingURL=getColorCSSPropertiesForTailwind.cjs.map