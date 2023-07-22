"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.extendWithTextSizes = exports.defineFontSizes = void 0;
// eslint-disable-next-line
const plugin = require("tailwindcss/plugin");
function defineFontSizes(v) {
    return v;
}
exports.defineFontSizes = defineFontSizes;
function extendWithTextSizes({ settings, webFallbackFonts, }) {
    if (!settings)
        throw new Error("Undefined settings");
    const utilObj = {};
    for (const font of settings.Fonts) {
        const foundFamily = settings.FontFamilies.find((a) => a.ID === font.Value.FontFamily.ref$);
        // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
        if (!foundFamily)
            throw new Error(`Failed to find family by ref "${font.Value.FontFamily.ref$}"`);
        // eslint-disable-next-line @typescript-eslint/restrict-plus-operands
        utilObj[".text-" + font.ID] = {
            letterSpacing: font.Value.Tracking,
            lineHeight: font.Value.LineHeight,
            fontSize: font.Value.FontSize,
            fontFamily: [foundFamily.HTMLFontFamilyName]
                .concat(webFallbackFonts[font.Value.FontFamily.ref$] ?? [])
                .map((a) => (a.includes(" ") ? JSON.stringify(a) : a))
                .join(", "),
        };
    }
    return plugin((utils) => {
        utils.addComponents(utilObj);
    });
}
exports.extendWithTextSizes = extendWithTextSizes;
//# sourceMappingURL=tailwind-apTypography.cjs.map