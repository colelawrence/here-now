import { TypographyProperty } from "./output.gen.ts";
type Value = unknown;
/**
 * String that follows the base name of the family.
 * This is used for your design programs like Figma.
 * e.g. `" Italic"` for italics of Inter or Source Serif
 * e.g. `" Thin"` for W100, `" Light"` for W300, `" Medium"` for W500, `" Bold"` for W700, etc.
 *
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:14`](../../hn-design-tools/src/typography/figma.rs)
 */
// deno-lint-ignore no-namespace
export namespace FigmaFontStyleRule {
  export type ApplyFns<R> = {
    // callbacks
    /** Suffix plus order number */
    FontSuffix(inner: [string, number]): R,
    /**
     * See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
     * e.g. `"'wght' 50"`
     */
    FontVariation(inner: [string, string]): R,
  }
  /** Match helper for {@link FigmaFontStyleRule} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: FigmaFontStyleRule) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("FontSuffix" in input) return to.FontSuffix(input["FontSuffix"]);
      if ("FontVariation" in input) return to.FontVariation(input["FontVariation"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link FigmaFontStyleRule} */
  export function match<R>(
    input: FigmaFontStyleRule,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  /** Suffix plus order number */
  export type FontSuffix = { FontSuffix: [string, number] };
  /** Suffix plus order number */
  export function FontSuffix(a: string, b: number): FontSuffix {
    return { FontSuffix: [a, b] };
  }
  /**
   * See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
   * e.g. `"'wght' 50"`
   */
  export type FontVariation = { FontVariation: [string, string] };
  /**
   * See https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings
   * e.g. `"'wght' 50"`
   */
  export function FontVariation(a: string, b: string): FontVariation {
    return { FontVariation: [a, b] };
  }
}
/**
 * String that follows the base name of the family.
 * This is used for your design programs like Figma.
 * e.g. `" Italic"` for italics of Inter or Source Serif
 * e.g. `" Thin"` for W100, `" Light"` for W300, `" Medium"` for W500, `" Bold"` for W700, etc.
 *
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:14`](../../hn-design-tools/src/typography/figma.rs)
 */
export type FigmaFontStyleRule =
  | FigmaFontStyleRule.FontSuffix
  | FigmaFontStyleRule.FontVariation
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:45`](../../hn-design-tools/src/typography/figma.rs)
 */
export type FigmaTypographyConfig = {
  /** A sort of matrice of all possible combinations of the variants */
  FigmaTextStyles: Array<FigmaTextStyle>;
};
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:45`](../../hn-design-tools/src/typography/figma.rs)
 */
export function FigmaTypographyConfig(inner: FigmaTypographyConfig): FigmaTypographyConfig {
  return inner;
}
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:55`](../../hn-design-tools/src/typography/figma.rs)
 */
export type FigmaTextStyle = {
  BaseName: string;
  BaseTokens: string;
  Description?: string | undefined | null | null | undefined;
  Groups: Array<FigmaTextStyleMatrixGroup>;
};
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:55`](../../hn-design-tools/src/typography/figma.rs)
 */
export function FigmaTextStyle(inner: FigmaTextStyle): FigmaTextStyle {
  return inner;
}
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:65`](../../hn-design-tools/src/typography/figma.rs)
 */
export type FigmaTextStyleMatrixGroup = {
  Description?: string | undefined | null | null | undefined;
  Options: Array<FigmaTextStyleMatrixOption>;
};
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:65`](../../hn-design-tools/src/typography/figma.rs)
 */
export function FigmaTextStyleMatrixGroup(inner: FigmaTextStyleMatrixGroup): FigmaTextStyleMatrixGroup {
  return inner;
}
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:73`](../../hn-design-tools/src/typography/figma.rs)
 */
export type FigmaTextStyleMatrixOption = {
  Name: string;
  Tokens: string;
  Description?: string | undefined | null | null | undefined;
};
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:73`](../../hn-design-tools/src/typography/figma.rs)
 */
export function FigmaTextStyleMatrixOption(inner: FigmaTextStyleMatrixOption): FigmaTextStyleMatrixOption {
  return inner;
}
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:26`](../../hn-design-tools/src/typography/figma.rs)
 */
export type TextStyle = {
  name: string;
  family_name: string;
  properties: Array<TypographyProperty>;
};
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:26`](../../hn-design-tools/src/typography/figma.rs)
 */
export function TextStyle(inner: TextStyle): TextStyle {
  return inner;
}
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:34`](../../hn-design-tools/src/typography/figma.rs)
 */
export type FigmaTypography = {
  core_styles: Array<TextStyle>;
};
/**
 * `#[codegen(tags = "figma,typography")]`
 *
 * [Source `hn-design-tools/src/typography/figma.rs:34`](../../hn-design-tools/src/typography/figma.rs)
 */
export function FigmaTypography(inner: FigmaTypography): FigmaTypography {
  return inner;
}