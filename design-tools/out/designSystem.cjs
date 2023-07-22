"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.createArtpromptColorsFromHTMLImage = exports.createArtpromptColorsFromSourceHex = exports.designSystemTypographySettings = exports.designSystemColorSettings = exports.artpromptExtendedColors = exports.hexFromArgb = void 0;
const design_system_settings_cjs_1 = require("./design-system-settings.cjs");
const systemColors_cjs_1 = require("./color/systemColors.cjs");
exports.default = "Artprompt design";
var material_color_utilities_1 = require("@artprompt/material-color-utilities");
Object.defineProperty(exports, "hexFromArgb", { enumerable: true, get: function () { return material_color_utilities_1.hexFromArgb; } });
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
//# sourceMappingURL=designSystem.cjs.map