"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.exportForFigma = void 0;
// Exporter for the "Figma Tokens Studio" plugin
// Todo: replace with Figma Plugin for loading the variables
const fs_1 = require("fs");
const no_case_1 = require("no-case");
const designSystem_cjs_1 = require("../designSystem.cjs");
const getAllColorsKebab_cjs_1 = require("../color/getAllColorsKebab.cjs");
const _example = {
    Typography: {
        "text-title-lg": {
            value: {
                fontFamily: "$title",
                lineHeight: "28px",
                fontSize: "22.7273px",
                letterSpacing: "0.0005em",
            },
            type: "typography",
        },
        title: {
            value: "Forecaster Work Sans",
            type: "fontFamilies",
        },
    },
    "Blue Light": {
        "sys-primary": {
            value: "#e41111",
            type: "color",
        },
    },
};
const altPrimarySets = [
    { Name: "Green", SeedHex: "#23a200" },
    { Name: "Blue", SeedHex: "#0023f2" },
    { Name: "Pink", SeedHex: "#ffd223" },
];
console.log("figmaTokens-export");
const COLOR_ID_ATTRIBUTES_SET = new Set(["container", "inverse", "on"]);
exportForFigma({
    typography: designSystem_cjs_1.designSystemTypographySettings,
    color: designSystem_cjs_1.designSystemColorSettings,
});
function exportForFigma(settings) {
    const typographyTokens = {};
    const colorTokens = {};
    for (const fam of settings.typography.FontFamilies) {
        typographyTokens[fam.ID] = {
            type: "fontFamilies",
            value: fam.FigmaFontFamilyName,
        };
    }
    for (const font of settings.typography.Fonts) {
        typographyTokens[font.ID] = {
            type: "typography",
            value: {
                fontFamily: `$${font.Value.FontFamily.ref$}`,
                fontSize: font.Value.FontSize,
                letterSpacing: font.Value.Tracking,
                lineHeight: font.Value.LineHeight,
            },
        };
    }
    const colorSets = [
        { Name: "Default", PrimaryHex: settings.color.config.Primary.Hex },
        ...altPrimarySets.map((set) => ({
            Name: set.Name,
            PrimaryHex: set.SeedHex,
        })),
    ];
    const allColorSets = colorSets
        .map((set) => ({
        Name: set.Name,
        theme: (0, designSystem_cjs_1.createArtpromptColorsFromSourceHex)(set.PrimaryHex, settings.color.config.extended),
    }))
        .flatMap((set) => [
        {
            Name: `${set.Name} Light`,
            Colors: (0, getAllColorsKebab_cjs_1.getAllColorsKebab)(set.theme, { dark: false }),
        },
        {
            Name: `${set.Name} Dark`,
            Colors: (0, getAllColorsKebab_cjs_1.getAllColorsKebab)(set.theme, { dark: true }),
        },
    ]);
    for (const color of allColorSets) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const colorSet = {};
        for (const c of color.Colors) {
            const prefix = [c.ext ? "ext" : "sys"];
            const postfix = [];
            for (const part of (0, no_case_1.noCase)(c.id, { delimiter: " " }).split(" ")) {
                if (COLOR_ID_ATTRIBUTES_SET.has(part)) {
                    postfix.push(part);
                }
                else {
                    prefix.push(part);
                }
            }
            // ? need to add -color suffix so the colors don't merge over the nests
            postfix.push("color");
            let layerObj = colorSet;
            for (const layer of prefix) {
                layerObj[layer] = layerObj[layer] ?? {};
                if (layerObj.type === "color")
                    console.error("Found conflicting item key", {
                        prefix,
                        postfix,
                        layerObj,
                    });
                layerObj = layerObj[layer];
            }
            const itemKey = postfix.join("-");
            if (layerObj[itemKey])
                console.error("Found conflicting item key", {
                    prefix,
                    postfix,
                    itemKey,
                });
            layerObj[postfix.join("-")] = {
                type: "color",
                value: (0, designSystem_cjs_1.hexFromArgb)(c.argb),
            };
        }
        colorTokens[color.Name] = colorSet;
    }
    const [outputFile] = process.argv.slice(-1);
    const { "Default Light": defaultColorSet, ...alternativeColorSets } = colorTokens;
    const outputJSON = {
        Defaults: {
            ...typographyTokens,
            ...defaultColorSet,
        },
        ...alternativeColorSets,
    };
    if (outputFile && outputFile.endsWith(".json")) {
        console.error(`Writing ${outputFile}`);
        (0, fs_1.writeFileSync)(outputFile, JSON.stringify(outputJSON, null, 2));
    }
    else {
        console.error("output json file not specified as last argument", outputJSON);
    }
}
exports.exportForFigma = exportForFigma;
//# sourceMappingURL=figmaTokens-export.cjs.map