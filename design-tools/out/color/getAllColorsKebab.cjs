"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getAllColorsKebab = void 0;
const no_case_1 = require("no-case");
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
                {
                    id: `on-${kebab}-container`,
                    argb: scheme.onColorContainer,
                    ext: true,
                },
            ];
        }),
    ];
}
exports.getAllColorsKebab = getAllColorsKebab;
//# sourceMappingURL=getAllColorsKebab.cjs.map