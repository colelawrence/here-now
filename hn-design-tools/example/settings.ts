// import workSansMetrics from "@capsizecss/metrics/workSans";
import * as input from "./input.gen.ts";
import { iBMPlexMonoMetrics } from "./iBMPlexMonoMetrics.ts";
import { interMetrics } from "./interMetrics.ts";
import { figmaTypographyExtension } from "./figma-typography-extension.ts";

const GOLDEN_RATIO = 1.61803398875;
// 1.61803 (golden ratio) ^ 0.5
const tightLineHeight = input.FontFamilyLineHeightRule.FontSizePxMultipler({ multiplier: Math.pow(GOLDEN_RATIO, 0.5) });
const spaciousLineHeight = input.FontFamilyLineHeightRule.FontSizePxMultipler({ multiplier: GOLDEN_RATIO });

const emojiFontFamilies = ["Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Arial"];

const typography = input.Typography({
  Extensions: {
    figma: figmaTypographyExtension,
  },
  Families: [
    {
      BaseName: "Inter",
      CSSFontFamilyName: "hnsans",
      CSSFontFamilyFallbacks: ["system-ui", ...emojiFontFamilies, "sans-serif"],
      Metrics: interMetrics,
      Weights: [
        { Weight: 100, FontStyleRule: { CSS: { FontWeight: 100 }, Figma: { Suffix: " Thin" } } },
        { Weight: 200, FontStyleRule: { CSS: { FontWeight: 200 }, Figma: { Suffix: " Extra Light" } } },
        { Weight: 300, FontStyleRule: { CSS: { FontWeight: 300 }, Figma: { Suffix: " Light" } } },
        { Weight: 400, FontStyleRule: { CSS: { FontWeight: 400 }, Figma: { Suffix: " Normal" } } },
        { Weight: 500, FontStyleRule: { CSS: { FontWeight: 500 }, Figma: { Suffix: " Medium" } } },
        { Weight: 600, FontStyleRule: { CSS: { FontWeight: 600 }, Figma: { Suffix: " Semi Bold" } } },
        { Weight: 700, FontStyleRule: { CSS: { FontWeight: 700 }, Figma: { Suffix: " Bold" } } },
        { Weight: 800, FontStyleRule: { CSS: { FontWeight: 800 }, Figma: { Suffix: " Extra Bold" } } },
        { Weight: 900, FontStyleRule: { CSS: { FontWeight: 900 }, Figma: { Suffix: " Black" } } },
      ],
      ItalicOption: { CSS: { FontStyle: "italic" }, Figma: { Suffix: " Italic" } },
    },
    {
      BaseName: "IBM Plex",
      CSSFontFamilyName: "hnmono",
      CSSFontFamilyFallbacks: ["Source Code Pro", ...emojiFontFamilies, "monospace"],
      Metrics: iBMPlexMonoMetrics,
      Weights: [
        { Weight: 100, FontStyleRule: { CSS: { FontWeight: 100 }, Figma: { Suffix: " Thin" } } },
        { Weight: 200, FontStyleRule: { CSS: { FontWeight: 200 }, Figma: { Suffix: " ExtraLight" } } },
        { Weight: 300, FontStyleRule: { CSS: { FontWeight: 300 }, Figma: { Suffix: " Light" } } },
        { Weight: 400, FontStyleRule: { CSS: { FontWeight: 400 }, Figma: { Suffix: " Normal" } } },
        { Weight: 500, FontStyleRule: { CSS: { FontWeight: 500 }, Figma: { Suffix: " Medium" } } },
        { Weight: 600, FontStyleRule: { CSS: { FontWeight: 600 }, Figma: { Suffix: " SemiBold" } } },
        { Weight: 700, FontStyleRule: { CSS: { FontWeight: 700 }, Figma: { Suffix: " Bold" } } },
      ],
      ItalicOption: { CSS: { FontStyle: "italic" }, Figma: { Suffix: " Italic" } },
    },
  ],
  TextRoles: [
    {
      Token: "content",
      FamilyBaseName: "Inter",
      LineHeightRule: spaciousLineHeight,
      TrackingRule: { DynMetrics: { a: -0.005, b: 0.26, c: -0.17 } },
    },
    {
      Token: "ui",
      FamilyBaseName: "Inter",
      LineHeightRule: tightLineHeight,
      TrackingRule: { DynMetrics: { a: -0.005, b: 0.26, c: -0.17 } },
    },
    {
      Token: "code",
      FamilyBaseName: "IBM Plex",
      LineHeightRule: tightLineHeight,
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
