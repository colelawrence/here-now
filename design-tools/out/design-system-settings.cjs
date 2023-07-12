"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.designSystemTypographySettings = exports.designSystemColorSettings = exports.artpromptExtendedColors = void 0;
const iBMPlexMono_1 = __importDefault(require("@capsizecss/metrics/iBMPlexMono"));
const inter_1 = __importDefault(require("@capsizecss/metrics/inter"));
const workSans_1 = __importDefault(require("@capsizecss/metrics/workSans"));
const systemColors_cjs_1 = require("./systemColors.cjs");
const systemTypography_cjs_1 = require("./systemTypography.cjs");
// Colors
exports.artpromptExtendedColors = [
    { ID: "blue", Seed: { Hex: "#1f108b" } },
    { ID: "yellow", Seed: { Hex: "#472e05" } },
    { ID: "cyan", Seed: { Hex: "#2bae9e" } },
    { ID: "orange", Seed: { Hex: "#c77d46" } },
    { ID: "magenta", Seed: { Hex: "#9c1a91" } },
    { ID: "green", Seed: { Hex: "#4be589" } },
];
exports.designSystemColorSettings = (0, systemColors_cjs_1.generateColorSettings)({
    Primary: { Hex: "#006684" },
    extended: exports.artpromptExtendedColors,
});
// Typography
const baseCapSize = 12;
const fontSizeRel = (rel) => ratioInterval(baseCapSize, rel);
// 1.61803 (golden ratio) ^ 0.5
const tightLineHeight = 1.272;
const spaciousLineHeight = 1.61803;
exports.designSystemTypographySettings = (0, systemTypography_cjs_1.generateTypographySettings)({
    families: [
        {
            ID: "ui",
            HTMLFontFamilyName: "apui",
            FigmaFontFamilyName: "Inter",
            Metrics: inter_1.default,
            TargetRelativeLineHeight: tightLineHeight,
            DynMetrics: { a: -0.005, b: 0.26, c: -0.17 },
        },
        {
            ID: "content",
            HTMLFontFamilyName: "apcontent",
            FigmaFontFamilyName: "Inter",
            Metrics: inter_1.default,
            TargetRelativeLineHeight: spaciousLineHeight,
            DynMetrics: { a: -0.005, b: 0.26, c: -0.17 },
        },
        {
            ID: "title",
            HTMLFontFamilyName: "aptitle",
            FigmaFontFamilyName: "Forecaster Work Sans",
            Metrics: workSans_1.default,
            TargetRelativeLineHeight: tightLineHeight,
            DynMetrics: { a: -0.02, b: 0.3, c: -0.1 },
        },
        {
            ID: "mono",
            HTMLFontFamilyName: "apmono",
            FigmaFontFamilyName: "IBM Plex Mono",
            Metrics: iBMPlexMono_1.default,
            TargetRelativeLineHeight: tightLineHeight,
            DynMetrics: { a: -0.005, b: 0.26, c: -0.17 },
        },
    ],
    sizes: [
        { ID: "xs", CapHeight: fontSizeRel(-2) },
        { ID: "sm", CapHeight: fontSizeRel(-1) },
        // Base
        { ID: "base", CapHeight: fontSizeRel(0) },
        // Quote
        { ID: "lg", CapHeight: fontSizeRel(1) },
        // h3
        { ID: "xl", CapHeight: fontSizeRel(2) },
        // h2
        { ID: "2xl", CapHeight: fontSizeRel(3) },
        // h1
        { ID: "3xl", CapHeight: fontSizeRel(4) },
        { ID: "4xl", CapHeight: fontSizeRel(5) },
    ],
});
/**
 * @param {number} base example `16`
 * @param {number} rel example `-1`, `0`, `1`
 */
function ratioInterval(base, rel) {
    const GOLDEN = 1.618033988749894;
    const GOLDEN_INTERVAL = Math.sqrt(GOLDEN);
    return Math.round(base * Math.pow(GOLDEN_INTERVAL, rel));
}
//# sourceMappingURL=design-system-settings.cjs.map