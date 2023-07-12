// import workSansMetrics from "@capsizecss/metrics/workSans";
import * as input from "./input.gen.ts";
import { iBMPlexMonoMetrics } from "./iBMPlexMonoMetrics.ts";
import { interMetrics } from "./interMetrics.ts";

const GOLDEN_RATIO = 1.61803398875;
// 1.61803 (golden ratio) ^ 0.5
const tightLineHeight = 1.272;
const spaciousLineHeight = 1.61803;

const emojiFontFamilies = ["Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Arial"];

const figmaWeightsGroup = input.FigmaTextStyleMatrixGroup({
  Description: "Font weight",
  Options: [
    { Name: "Base", Tokens: "" },
    { Name: "Thin", Tokens: "w100" },
    { Name: "Extra Light", Tokens: "w200" },
    { Name: "Light", Tokens: "w300" },
    { Name: "Normal", Tokens: "w400" },
    { Name: "Medium", Tokens: "w500" },
    { Name: "Semi Bold", Tokens: "w600" },
    { Name: "Bold", Tokens: "w700" },
    { Name: "Extra Bold", Tokens: "w800" },
    { Name: "Black", Tokens: "w900" },
  ],
});

const figmaItalicGroup = input.FigmaTextStyleMatrixGroup({
  Description: "Font italicized",
  Options: [
    { Name: "Base", Tokens: "" },
    { Name: "Italic", Tokens: "italic" },
  ],
});

const figmaProseStyleGroup = input.FigmaTextStyleMatrixGroup({
  Description: "Prose stylization",
  Options: [
    { Name: "Base", Tokens: "" },
    { Name: "Code", Tokens: "code" },
  ],
});

const typography = input.Typography({
  Families: [
    {
      BaseName: "Inter",
      CSSFontFamilyName: "hnsans",
      CSSFontFamilyFallbacks: ["system-ui", ...emojiFontFamilies, "sans-serif"],
      Metrics: interMetrics,
      Weights: [
        { W100: { CSSRule: { FontWeight: 100 }, Suffix: " Thin" } },
        { W200: { CSSRule: { FontWeight: 200 }, Suffix: " Extra Light" } },
        { W300: { CSSRule: { FontWeight: 300 }, Suffix: " Light" } },
        { W400: { CSSRule: { FontWeight: 400 }, Suffix: " Normal" } },
        { W500: { CSSRule: { FontWeight: 500 }, Suffix: " Medium" } },
        { W600: { CSSRule: { FontWeight: 600 }, Suffix: " Semi Bold" } },
        { W700: { CSSRule: { FontWeight: 700 }, Suffix: " Bold" } },
        { W800: { CSSRule: { FontWeight: 800 }, Suffix: " Extra Bold" } },
        { W900: { CSSRule: { FontWeight: 900 }, Suffix: " Black" } },
      ],
      ItalicOption: { CSSRule: "FontStyleItalics", Suffix: " Italic" },
    },
    {
      BaseName: "IBM Plex",
      CSSFontFamilyName: "hnmono",
      CSSFontFamilyFallbacks: ["Source Code Pro", ...emojiFontFamilies, "monospace"],
      Metrics: iBMPlexMonoMetrics,
      Weights: [
        { W100: { CSSRule: { FontWeight: 100 }, Suffix: "Thin" } },
        { W200: { CSSRule: { FontWeight: 200 }, Suffix: "ExtraLight" } },
        { W300: { CSSRule: { FontWeight: 300 }, Suffix: "Light" } },
        { W400: { CSSRule: { FontWeight: 400 }, Suffix: "Normal" } },
        { W500: { CSSRule: { FontWeight: 500 }, Suffix: "Medium" } },
        { W600: { CSSRule: { FontWeight: 600 }, Suffix: "SemiBold" } },
        { W700: { CSSRule: { FontWeight: 700 }, Suffix: "Bold" } },
      ],
      ItalicOption: { CSSRule: "FontStyleItalics", Suffix: " Italic" },
    },
  ],
  TextRoles: [
    {
      Token: "content",
      FamilyBaseName: "Inter",
      TargetRelativeLineHeight: spaciousLineHeight,
      TrackingRule: { DynMetrics: { a: -0.005, b: 0.26, c: -0.17 } },
    },
    {
      Token: "ui",
      FamilyBaseName: "Inter",
      TargetRelativeLineHeight: tightLineHeight,
      TrackingRule: { DynMetrics: { a: -0.005, b: 0.26, c: -0.17 } },
    },
    {
      Token: "code",
      FamilyBaseName: "IBM Plex",
      TargetRelativeLineHeight: tightLineHeight,
      TrackingRule: { DynMetrics: { a: -0.005, b: 0.26, c: -0.17 } },
    },
  ],
  FontSizeScale: {
    Equation: input.FontSizeEquation.Multiplier({
      base_px: 12,
      multiplier: Math.sqrt(GOLDEN_RATIO),
    }),
    FontSizes: [
      { Token: "xs", Rel: -2 },
      { Token: "sm", Rel: -1 },
      // Base (12px based on input.FontSizeEquation above)
      { Token: "base", Rel: 0 },
      // Quote
      { Token: "lg", Rel: 1 },
      // h3
      { Token: "xl", Rel: 2 },
      // h2
      { Token: "2xl", Rel: 3 },
      // h1
      { Token: "3xl", Rel: 4 },
      { Token: "4xl", Rel: 5 },
    ],
  },
  FigmaTextStyles: [
    {
      BaseName: "Content",
      BaseTokens: "content",
      Groups: [
        {
          Options: [
            { Name: "Smaller", Tokens: "xs" },
            { Name: "Small", Tokens: "sm" },
            { Name: "Base", Tokens: "base" },
            { Name: "Quote", Tokens: "lg w500" },
            { Name: "Heading 3", Tokens: "lg w700", Description: "Use gray color" },
            { Name: "Heading 2", Tokens: "xl w700" },
            { Name: "Heading 1", Tokens: "2xl w700" },
            { Name: "Hero Title (3XL)", Tokens: "3xl w700" },
            { Name: "Hero Title (4XL)", Tokens: "4xl w700" },
          ],
        },
        figmaWeightsGroup,
        figmaProseStyleGroup,
        figmaItalicGroup,
      ],
    },
    {
      BaseName: "UI",
      BaseTokens: "ui",
      Groups: [
        {
          Description: "text size",
          Options: [
            { Name: "Smaller", Tokens: "xs" },
            { Name: "Small", Tokens: "sm" },
            { Name: "Base", Tokens: "base" },
            { Name: "Large", Tokens: "lg" },
            { Name: "Larger", Tokens: "xl" },
            // Add 2X if you like.
            { Name: "3X Large", Tokens: "3xl" },
          ],
        },
        figmaWeightsGroup,
        figmaProseStyleGroup,
        figmaItalicGroup,
      ],
    },
    {
      BaseName: "Mono",
      BaseTokens: "code base",
      Groups: [figmaWeightsGroup, figmaItalicGroup],
    },
  ],
});

const color_palette = input.ColorPalette({
  Primary: { Hex: "#AFD2E9" },
  Extensions: [
    { Token: "blue", Source: { SimilarTo: { Hex: "#1f108b" } } },
    { Token: "yellow", Source: { SimilarTo: { Hex: "#472e05" } } },
    { Token: "cyan", Source: { SimilarTo: { Hex: "#2bae9e" } } },
    { Token: "orange", Source: { SimilarTo: { Hex: "#c77d46" } } },
    { Token: "magenta", Source: { SimilarTo: { Hex: "#9c1a91" } } },
    { Token: "green", Source: { SimilarTo: { Hex: "#4be589" } } },
  ],
});

export const settings = input.SystemInput({
  color_palette,
  typography,
});
