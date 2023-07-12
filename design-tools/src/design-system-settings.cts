import iBMPlexMonoMetrics from "@capsizecss/metrics/iBMPlexMono";
import interMetrics from "@capsizecss/metrics/inter";
import workSansMetrics from "@capsizecss/metrics/workSans";

import type { ExtendedColors } from "./color/systemColors.cjs";
import { generateColorSettings } from "./color/systemColors.cjs";
import { generateTypographySettings } from "./typography/systemTypography.cjs";
import { ratioInterval } from "./numbers/ratioInterval.cjs";

// Colors

export const artpromptExtendedColors: ExtendedColors = [
  { ID: "blue", Seed: { Hex: "#1f108b" } },
  { ID: "yellow", Seed: { Hex: "#472e05" } },
  { ID: "cyan", Seed: { Hex: "#2bae9e" } },
  { ID: "orange", Seed: { Hex: "#c77d46" } },
  { ID: "magenta", Seed: { Hex: "#9c1a91" } },
  { ID: "green", Seed: { Hex: "#4be589" } },
];

export const designSystemColorSettings = generateColorSettings({
  Primary: { Hex: "#006684" },
  extended: artpromptExtendedColors,
});

// Typography

const baseCapSize = 12;
const fontSizeRel = (rel: number) => ratioInterval(baseCapSize, rel);

// 1.61803 (golden ratio) ^ 0.5
const tightLineHeight = 1.272;
const spaciousLineHeight = 1.61803;

export const designSystemTypographySettings = generateTypographySettings({
  families: [
    {
      ID: "ui",
      HTMLFontFamilyName: "apui",
      FigmaFontFamilyName: "Inter",
      Metrics: interMetrics,
      TargetRelativeLineHeight: tightLineHeight,
      DynMetrics: { a: -0.005, b: 0.26, c: -0.17 },
    },
    {
      ID: "content",
      HTMLFontFamilyName: "apcontent",
      FigmaFontFamilyName: "Inter",
      Metrics: interMetrics,
      TargetRelativeLineHeight: spaciousLineHeight,
      DynMetrics: { a: -0.005, b: 0.26, c: -0.17 },
    },
    {
      ID: "title",
      HTMLFontFamilyName: "aptitle",
      FigmaFontFamilyName: "Forecaster Work Sans",
      Metrics: workSansMetrics,
      TargetRelativeLineHeight: tightLineHeight,
      DynMetrics: { a: -0.02, b: 0.3, c: -0.1 },
    },
    {
      ID: "mono",
      HTMLFontFamilyName: "apmono",
      FigmaFontFamilyName: "IBM Plex Mono",
      Metrics: iBMPlexMonoMetrics,
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
