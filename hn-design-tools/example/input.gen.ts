/**
 * TODO: adjust API/Configuration to be more accomodating
 * of other color generation strategies than just Material You.
 *
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:5`](../../hn-design-tools/src/color/input.rs)
 */
export type ColorPalette = {
  Primary: InputColor;
  Extensions: Array<ColorExtension>;
};
/**
 * TODO: adjust API/Configuration to be more accomodating
 * of other color generation strategies than just Material You.
 *
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:5`](../../hn-design-tools/src/color/input.rs)
 */
export function ColorPalette(inner: ColorPalette): ColorPalette {
  return inner;
}
/**
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:13`](../../hn-design-tools/src/color/input.rs)
 */
export type ColorExtension = {
  /** e.g. `"blue"` */
  Token: string;
  Source: SourceColor;
};
/**
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:13`](../../hn-design-tools/src/color/input.rs)
 */
export function ColorExtension(inner: ColorExtension): ColorExtension {
  return inner;
}
/**
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:22`](../../hn-design-tools/src/color/input.rs)
 */
// deno-lint-ignore no-namespace
export namespace SourceColor {
  export type ApplyFns<R> = {
    // callbacks
    SimilarTo(inner: SimilarTo["SimilarTo"]): R;
    Exactly(inner: Exactly["Exactly"]): R;
  }
  /** Match helper for {@link SourceColor} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: SourceColor) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("SimilarTo" in input) return to.SimilarTo(input["SimilarTo"]);
      if ("Exactly" in input) return to.Exactly(input["Exactly"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link SourceColor} */
  export function match<R>(
    input: SourceColor,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type SimilarTo = {
    SimilarTo: InputColor
  };
  export function SimilarTo(value: InputColor): SimilarTo {
    return { SimilarTo: value };
  }
  export type Exactly = {
    Exactly: InputColor
  };
  export function Exactly(value: InputColor): Exactly {
    return { Exactly: value };
  }
}
/**
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:22`](../../hn-design-tools/src/color/input.rs)
 */
export type SourceColor =
  | SourceColor.SimilarTo
  | SourceColor.Exactly
/**
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:29`](../../hn-design-tools/src/color/input.rs)
 */
// deno-lint-ignore no-namespace
export namespace InputColor {
  export type ApplyFns<R> = {
    // callbacks
    Hex(inner: Hex["Hex"]): R;
  }
  /** Match helper for {@link InputColor} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: InputColor) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Hex" in input) return to.Hex(input["Hex"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link InputColor} */
  export function match<R>(
    input: InputColor,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type Hex = {
    Hex: string
  };
  export function Hex(value: string): Hex {
    return { Hex: value };
  }
}
/**
 * `#[codegen(tags = "input,color")]`
 *
 * [Source `hn-design-tools/src/color/input.rs:29`](../../hn-design-tools/src/color/input.rs)
 */
export type InputColor =
  | InputColor.Hex
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:3`](../../hn-design-tools/src/typography/input.rs)
 */
export type Typography = {
  Families: Array<FontFamilyInfo>;
  /** Scaling strategy for different font-sizes. */
  FontSizeScale: FontSizeScale;
  TextRoles: Array<TextRole>;
  /** A sort of matrice of all possible combinations of the variants */
  FigmaTextStyles: Array<FigmaTextStyle>;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:3`](../../hn-design-tools/src/typography/input.rs)
 */
export function Typography(inner: Typography): Typography {
  return inner;
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:18`](../../hn-design-tools/src/typography/input.rs)
 */
export type TextRole = {
  /** e.g. `"ui"` or `"content"` */
  Token: string;
  /**
   * e.g. `"Inter"` or `"Merriweather"`, this must be described
   * in [Typography] "Families".
   */
  FamilyBaseName: string;
  /** e.g. tight = `1.272` or spacious = `1.61803` */
  TargetRelativeLineHeight: number;
  /** Also called "letter spacing," this is the space between letters for different sizes */
  TrackingRule: FontFamilyTrackingRule;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:18`](../../hn-design-tools/src/typography/input.rs)
 */
export function TextRole(inner: TextRole): TextRole {
  return inner;
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:33`](../../hn-design-tools/src/typography/input.rs)
 */
export type FigmaTextStyle = {
  BaseName: string;
  BaseTokens: string;
  Description?: string | undefined | null | null | undefined;
  Groups: Array<FigmaTextStyleMatrixGroup>;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:33`](../../hn-design-tools/src/typography/input.rs)
 */
export function FigmaTextStyle(inner: FigmaTextStyle): FigmaTextStyle {
  return inner;
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:43`](../../hn-design-tools/src/typography/input.rs)
 */
export type FigmaTextStyleMatrixGroup = {
  Description?: string | undefined | null | null | undefined;
  Options: Array<FigmaTextStyleMatrixOption>;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:43`](../../hn-design-tools/src/typography/input.rs)
 */
export function FigmaTextStyleMatrixGroup(inner: FigmaTextStyleMatrixGroup): FigmaTextStyleMatrixGroup {
  return inner;
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:51`](../../hn-design-tools/src/typography/input.rs)
 */
export type FigmaTextStyleMatrixOption = {
  Name: string;
  Tokens: string;
  Description?: string | undefined | null | null | undefined;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:51`](../../hn-design-tools/src/typography/input.rs)
 */
export function FigmaTextStyleMatrixOption(inner: FigmaTextStyleMatrixOption): FigmaTextStyleMatrixOption {
  return inner;
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:60`](../../hn-design-tools/src/typography/input.rs)
 */
export type FontFamilyInfo = {
  /** e.g. `"Inter"` or `"Merriweather"` */
  BaseName: string;
  /** e.g. `"Inter"` or `"Merriweather"` */
  CSSFontFamilyName?: string | undefined | null | null | undefined;
  /** e.g. `"system-ui", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Arial", "sans-serif"` */
  CSSFontFamilyFallbacks: Array<string>;
  Weights: Array<FamilyWeightRule>;
  ItalicOption?: FontVariantInfo | undefined | null | null | undefined;
  /** e.g. metrics from @capsize/metrics */
  Metrics: FontFamilyMetrics;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:60`](../../hn-design-tools/src/typography/input.rs)
 */
export function FontFamilyInfo(inner: FontFamilyInfo): FontFamilyInfo {
  return inner;
}
/**
 * One variant such as applying italic or a weight.
 * Progress 2/10:
 * * It's a bit tricky to describe whether this variant requires a font suffix
 * versus it requiring a variable axes tweak, etc. Or if these might be
 * different between your design program and the web distributable.
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:83`](../../hn-design-tools/src/typography/input.rs)
 */
export type FontVariantInfo = {
  /**
   * String that follows the base name of the family.
   * This is used for your design programs like Adobe Illustrator or Figma.
   * e.g. `" Italic"` for italics of Inter or Source Serif
   * e.g. `" Thin"` for W100, `" Light"` for W300, `" Medium"` for W500, `" Bold"` for W700, etc.
   */
  Suffix?: string | undefined | null | null | undefined;
  /** depends on how you load your fonts in the application */
  CSSRule: CSSRule;
};
/**
 * One variant such as applying italic or a weight.
 * Progress 2/10:
 * * It's a bit tricky to describe whether this variant requires a font suffix
 * versus it requiring a variable axes tweak, etc. Or if these might be
 * different between your design program and the web distributable.
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:83`](../../hn-design-tools/src/typography/input.rs)
 */
export function FontVariantInfo(inner: FontVariantInfo): FontVariantInfo {
  return inner;
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:97`](../../hn-design-tools/src/typography/input.rs)
 */
// deno-lint-ignore no-namespace
export namespace CSSRule {
  export type ApplyFns<R> = {
    // callbacks
    FontStyleItalics(): R,
    FontWeightBold(): R,
    FontWeight(inner: FontWeight["FontWeight"]): R;
    /**
     * See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
     * e.g. `"'wght' 50"`
     */
    FontVariationSetting(inner: FontVariationSetting["FontVariationSetting"]): R;
  }
  /** Match helper for {@link CSSRule} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: CSSRule) => R {
    return function _match(input): R {
      // if-else strings
      if (input === "FontStyleItalics") return to.FontStyleItalics();
      if (input === "FontWeightBold") return to.FontWeightBold();
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("FontWeight" in input) return to.FontWeight(input["FontWeight"]);
      if ("FontVariationSetting" in input) return to.FontVariationSetting(input["FontVariationSetting"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link CSSRule} */
  export function match<R>(
    input: CSSRule,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type FontStyleItalics = "FontStyleItalics"
  export function FontStyleItalics(): FontStyleItalics {
    return "FontStyleItalics";
  }
  export type FontWeightBold = "FontWeightBold"
  export function FontWeightBold(): FontWeightBold {
    return "FontWeightBold";
  }
  export type FontWeight = {
    FontWeight: number
  };
  export function FontWeight(value: number): FontWeight {
    return { FontWeight: value };
  }
  /**
   * See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
   * e.g. `"'wght' 50"`
   */
  export type FontVariationSetting = {
    /**
     * See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
     * e.g. `"'wght' 50"`
     */
    FontVariationSetting: string
  };
  /**
   * See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
   * e.g. `"'wght' 50"`
   */
  export function FontVariationSetting(value: string): FontVariationSetting {
    return { FontVariationSetting: value };
  }
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:97`](../../hn-design-tools/src/typography/input.rs)
 */
export type CSSRule =
  | CSSRule.FontStyleItalics
  | CSSRule.FontWeightBold
  | CSSRule.FontWeight
  | CSSRule.FontVariationSetting
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:109`](../../hn-design-tools/src/typography/input.rs)
 */
// deno-lint-ignore no-namespace
export namespace FamilyWeightRule {
  export type ApplyFns<R> = {
    // callbacks
    /** e.g. `"Thin"` for Inter. */
    W100(inner: W100["W100"]): R;
    /** e.g. `"Extra Light"` for Inter. */
    W200(inner: W200["W200"]): R;
    /** e.g. `"Light"` for Inter. */
    W300(inner: W300["W300"]): R;
    /** e.g. `"Regular"` for Inter. */
    W400(inner: W400["W400"]): R;
    /** e.g. `"Medium"` for Inter. */
    W500(inner: W500["W500"]): R;
    /** e.g. `"Semi Bold"` for Inter. */
    W600(inner: W600["W600"]): R;
    /** e.g. `"Bold"` for Inter. */
    W700(inner: W700["W700"]): R;
    /** e.g. `"Extra Bold"` for Inter. */
    W800(inner: W800["W800"]): R;
    /** e.g. `"Black"` for Inter. */
    W900(inner: W900["W900"]): R;
  }
  /** Match helper for {@link FamilyWeightRule} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: FamilyWeightRule) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("W100" in input) return to.W100(input["W100"]);
      if ("W200" in input) return to.W200(input["W200"]);
      if ("W300" in input) return to.W300(input["W300"]);
      if ("W400" in input) return to.W400(input["W400"]);
      if ("W500" in input) return to.W500(input["W500"]);
      if ("W600" in input) return to.W600(input["W600"]);
      if ("W700" in input) return to.W700(input["W700"]);
      if ("W800" in input) return to.W800(input["W800"]);
      if ("W900" in input) return to.W900(input["W900"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link FamilyWeightRule} */
  export function match<R>(
    input: FamilyWeightRule,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  /** e.g. `"Thin"` for Inter. */
  export type W100 = {
    /** e.g. `"Thin"` for Inter. */
    W100: FontVariantInfo
  };
  /** e.g. `"Thin"` for Inter. */
  export function W100(value: FontVariantInfo): W100 {
    return { W100: value };
  }
  /** e.g. `"Extra Light"` for Inter. */
  export type W200 = {
    /** e.g. `"Extra Light"` for Inter. */
    W200: FontVariantInfo
  };
  /** e.g. `"Extra Light"` for Inter. */
  export function W200(value: FontVariantInfo): W200 {
    return { W200: value };
  }
  /** e.g. `"Light"` for Inter. */
  export type W300 = {
    /** e.g. `"Light"` for Inter. */
    W300: FontVariantInfo
  };
  /** e.g. `"Light"` for Inter. */
  export function W300(value: FontVariantInfo): W300 {
    return { W300: value };
  }
  /** e.g. `"Regular"` for Inter. */
  export type W400 = {
    /** e.g. `"Regular"` for Inter. */
    W400: FontVariantInfo
  };
  /** e.g. `"Regular"` for Inter. */
  export function W400(value: FontVariantInfo): W400 {
    return { W400: value };
  }
  /** e.g. `"Medium"` for Inter. */
  export type W500 = {
    /** e.g. `"Medium"` for Inter. */
    W500: FontVariantInfo
  };
  /** e.g. `"Medium"` for Inter. */
  export function W500(value: FontVariantInfo): W500 {
    return { W500: value };
  }
  /** e.g. `"Semi Bold"` for Inter. */
  export type W600 = {
    /** e.g. `"Semi Bold"` for Inter. */
    W600: FontVariantInfo
  };
  /** e.g. `"Semi Bold"` for Inter. */
  export function W600(value: FontVariantInfo): W600 {
    return { W600: value };
  }
  /** e.g. `"Bold"` for Inter. */
  export type W700 = {
    /** e.g. `"Bold"` for Inter. */
    W700: FontVariantInfo
  };
  /** e.g. `"Bold"` for Inter. */
  export function W700(value: FontVariantInfo): W700 {
    return { W700: value };
  }
  /** e.g. `"Extra Bold"` for Inter. */
  export type W800 = {
    /** e.g. `"Extra Bold"` for Inter. */
    W800: FontVariantInfo
  };
  /** e.g. `"Extra Bold"` for Inter. */
  export function W800(value: FontVariantInfo): W800 {
    return { W800: value };
  }
  /** e.g. `"Black"` for Inter. */
  export type W900 = {
    /** e.g. `"Black"` for Inter. */
    W900: FontVariantInfo
  };
  /** e.g. `"Black"` for Inter. */
  export function W900(value: FontVariantInfo): W900 {
    return { W900: value };
  }
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:109`](../../hn-design-tools/src/typography/input.rs)
 */
export type FamilyWeightRule =
  | FamilyWeightRule.W100
  | FamilyWeightRule.W200
  | FamilyWeightRule.W300
  | FamilyWeightRule.W400
  | FamilyWeightRule.W500
  | FamilyWeightRule.W600
  | FamilyWeightRule.W700
  | FamilyWeightRule.W800
  | FamilyWeightRule.W900
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:133`](../../hn-design-tools/src/typography/input.rs)
 */
export type FontSizeScale = {
  FontSizes: Array<FontSizeRel>;
  Equation: FontSizeEquation;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:133`](../../hn-design-tools/src/typography/input.rs)
 */
export function FontSizeScale(inner: FontSizeScale): FontSizeScale {
  return inner;
}
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:141`](../../hn-design-tools/src/typography/input.rs)
 */
export type FontSizeRel = {
  /** e.g. `"xs"`, `"sm"`, `"base"`, `"lg"`, etc. */
  Token: string;
  /** e.g. `-2`, `-1`, `0`, `1`, etc. */
  Rel: number;
};
/**
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:141`](../../hn-design-tools/src/typography/input.rs)
 */
export function FontSizeRel(inner: FontSizeRel): FontSizeRel {
  return inner;
}
/**
 * WIP: Based on @capsizecss/metrics
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:152`](../../hn-design-tools/src/typography/input.rs)
 */
export type FontFamilyMetrics = {
  familyName: string;
  category: string;
  capHeight: number;
  ascent: number;
  descent: number;
  lineGap: number;
  unitsPerEm: number;
  xHeight: number;
  xWidthAvg: number;
};
/**
 * WIP: Based on @capsizecss/metrics
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:152`](../../hn-design-tools/src/typography/input.rs)
 */
export function FontFamilyMetrics(inner: FontFamilyMetrics): FontFamilyMetrics {
  return inner;
}
/**
 * WIP: Based on @capsizecss/metrics
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:168`](../../hn-design-tools/src/typography/input.rs)
 */
// deno-lint-ignore no-namespace
export namespace FontFamilyTrackingRule {
  export type ApplyFns<R> = {
    // callbacks
    /** Determine your tracking goals via a page like https://rsms.me/inter/dynmetrics/ */
    DynMetrics(inner: DynMetrics["DynMetrics"]): R,
  }
  /** Match helper for {@link FontFamilyTrackingRule} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: FontFamilyTrackingRule) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("DynMetrics" in input) return to.DynMetrics(input["DynMetrics"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link FontFamilyTrackingRule} */
  export function match<R>(
    input: FontFamilyTrackingRule,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  /** Determine your tracking goals via a page like https://rsms.me/inter/dynmetrics/ */
  export type DynMetrics = {
    /** Determine your tracking goals via a page like https://rsms.me/inter/dynmetrics/ */
    DynMetrics: {
      a: number;
      b: number;
      c: number;
    };
  }
  /** Determine your tracking goals via a page like https://rsms.me/inter/dynmetrics/ */
  export function DynMetrics(value: DynMetrics["DynMetrics"]): DynMetrics {
    return { DynMetrics: value }
  }
}
/**
 * WIP: Based on @capsizecss/metrics
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:168`](../../hn-design-tools/src/typography/input.rs)
 */
export type FontFamilyTrackingRule =
  | FontFamilyTrackingRule.DynMetrics
/**
 * WIP: Based on ratioInterval
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:177`](../../hn-design-tools/src/typography/input.rs)
 */
// deno-lint-ignore no-namespace
export namespace FontSizeEquation {
  export type ApplyFns<R> = {
    // callbacks
    Multiplier(inner: Multiplier["Multiplier"]): R,
  }
  /** Match helper for {@link FontSizeEquation} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: FontSizeEquation) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Multiplier" in input) return to.Multiplier(input["Multiplier"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link FontSizeEquation} */
  export function match<R>(
    input: FontSizeEquation,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type Multiplier = {
    Multiplier: {
      base_px: number;
      /**
       * Popular options would be `1.27201965` (sqrt(Golden Ratio)), or `1.4`
       * These would indicate the scale applied with each successive increase
       * of the font size base number.
       */
      multiplier: number;
    };
  }
  export function Multiplier(value: Multiplier["Multiplier"]): Multiplier {
    return { Multiplier: value }
  }
}
/**
 * WIP: Based on ratioInterval
 *
 * `#[codegen(tags = "input,typography")]`
 *
 * [Source `hn-design-tools/src/typography/input.rs:177`](../../hn-design-tools/src/typography/input.rs)
 */
export type FontSizeEquation =
  | FontSizeEquation.Multiplier