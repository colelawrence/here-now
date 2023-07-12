/** `#[codegen(tags = "figma")]` */
// deno-lint-ignore no-namespace
export namespace FigmaVariable {
  export type ApplyFns<R> = {
    // callbacks
    Color(inner: Color["Color"]): R;
    Length(inner: Length["Length"]): R;
  }
  /** Match helper for {@link FigmaVariable} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: FigmaVariable) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Color" in input) return to.Color(input["Color"]);
      if ("Length" in input) return to.Length(input["Length"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link FigmaVariable} */
  export function match<R>(
    input: FigmaVariable,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type Color = {
    Color: FigmaColor
  };
  export function Color(value: FigmaColor): Color {
    return { Color: value };
  }
  export type Length = {
    Length: FigmaLength
  };
  export function Length(value: FigmaLength): Length {
    return { Length: value };
  }
}
/** `#[codegen(tags = "figma")]` */
export type FigmaVariable =
  | FigmaVariable.Color
  | FigmaVariable.Length
/** `#[codegen(tags = "figma")]` */
export type Named<T> = {
  tailwind_id: string;
  name: string;
  description?: string | undefined | null | null | undefined;
  value: T;
};
/** `#[codegen(tags = "figma")]` */
export function Named<T>(inner: Named<T>): Named<T> {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "figma")]`
 */
export type Pixels = number
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "figma")]`
 */
export function Pixels(inner: number): Pixels {
  return inner;
}
/** `#[codegen(tags = "figma")]` */
export type TextStyle = {
  /** e.g. `"Inter"` */
  font_family: string;
  /** e.g. `"Regular"` */
  font_style: string;
  /** e.g. `12` */
  font_size: Pixels;
  /** e.g. `15.5600004196167` */
  line_height: Pixels;
  /** e.g. `1.0202931111111` */
  letter_spacing: Pixels;
};
/** `#[codegen(tags = "figma")]` */
export function TextStyle(inner: TextStyle): TextStyle {
  return inner;
}
/** `#[codegen(tags = "figma")]` */
export type DesignSystem = {
  variables: Array<Named<FigmaVariable>>;
  text_styles: Array<Named<TextStyle>>;
};
/** `#[codegen(tags = "figma")]` */
export function DesignSystem(inner: DesignSystem): DesignSystem {
  return inner;
}
/** `#[codegen(tags = "figma")]` */
export type FigmaColor = {
  r: number;
  g: number;
  b: number;
  a: number;
};
/** `#[codegen(tags = "figma")]` */
export function FigmaColor(inner: FigmaColor): FigmaColor {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "figma")]`
 */
export type FigmaLength = number
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "figma")]`
 */
export function FigmaLength(inner: number): FigmaLength {
  return inner;
}