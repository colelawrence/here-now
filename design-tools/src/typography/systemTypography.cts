import type { FontMetrics } from "@capsizecss/core";

/** https://rsms.me/inter/dynmetrics/ */
export type TrackingRule = {
  a: number;
  b: number;
  c: number;
};

export type TypographySettings = {
  FontFamilies: {
    ID: string;
    HTMLFontFamilyName: string;
    FigmaFontFamilyName: string;
  }[];
  Fonts: {
    ID: string;
    Value: Typography.Font;
  }[];
};

type FontSizeConfig = {
  ID: string;
  CapHeight: number;
};

type FontFamilyConfig = {
  ID: string;
  HTMLFontFamilyName: string;
  FigmaFontFamilyName: string;
  Metrics: FontMetrics;
  DynMetrics: Typography.DynMetrics;
  TargetRelativeLineHeight: number;
};

export function defineTypographyConfig(config: TypographyConfig): TypographyConfig {
  return config;
}

export type TypographyConfig = {
  sizes: FontSizeConfig[];
  families: FontFamilyConfig[];
};

export function generateTypographySettings(config: TypographyConfig): TypographySettings {
  const values: TypographySettings = {
    FontFamilies: [],
    Fonts: [],
  };

  for (const fam of config.families) {
    if (!fam.Metrics) throw new TypeError(`Metrics is required, but not found for '${fam.ID}'`)

    values.FontFamilies.push({
      ID: fam.ID,
      HTMLFontFamilyName: fam.HTMLFontFamilyName,
      FigmaFontFamilyName: fam.FigmaFontFamilyName,
    });

    for (const size of config.sizes) {
      values.Fonts.push({
        ID: `${fam.ID}-${size.ID}`,
        Value: {
          FontFamily: { ref$: fam.ID },
          ...calculateFontSizes(fam.Metrics, fam, size),
        },
      });
    }
  }

  return values;
}

namespace Typography {
  export type FontFamily = { FontFamily: { ref$: string } };
  export type Tracking = { Tracking: EM };
  export type LineHeight = { LineHeight: PX };
  export type FontSize = { FontSize: PX };

  /** https://rsms.me/inter/dynmetrics/ */
  export type DynMetrics = {
    a: number;
    b: number;
    c: number;
  };

  export type Font = FontFamily & Tracking & LineHeight & FontSize;
}

/**
 * Get font size + line height for a given sizing configuration
 */
function calculateFontSizes(
  metrics: FontMetrics,
  family: FontFamilyConfig,
  config: FontSizeConfig,
): Typography.FontSize & Typography.LineHeight & Typography.Tracking {
  // console.error(metrics);
  const recip = metrics.unitsPerEm / metrics.capHeight;
  const alignToGrid = 4;
  const fontUnitPx = config.CapHeight * recip;
  const lineHeightPx = fontUnitPx * family.TargetRelativeLineHeight;
  const lineHeightAlignedPx = Math.round(lineHeightPx / alignToGrid) * alignToGrid;
  // cap / unit = cap / unit
  const { a, b, c } = family.DynMetrics;

  const z = fontUnitPx;
  const tracking = a + b * Math.pow(Math.E, c * z);

  return {
    FontSize: px(fontUnitPx),
    LineHeight: px(lineHeightAlignedPx),
    Tracking: em(tracking),
  };
}

type PX = `${string}px` & { px: true };
type EM = `${string}em` & { em: true };

function px(n: number): PX {
  /*
   * Rounding all values to a precision of `4` based on discovering that browser
   * implementations of layout units fall between 1/60th and 1/64th of a pixel.
   * Reference: https://trac.webkit.org/wiki/LayoutUnit
   * (above wiki also mentions Mozilla - https://trac.webkit.org/wiki/LayoutUnit#Notes)
   */
  return `${n.toFixed(4).replace(/\.0+$/, "")}px` as PX;
}

function em(n: number): EM {
  return `${n.toFixed(4).replace(/\.0+$/, "")}em` as EM;
}
