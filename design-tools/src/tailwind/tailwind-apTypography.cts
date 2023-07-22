import type { CSSRuleObject } from "tailwindcss/types/config.js";
import { TypographySettings } from "../typography/systemTypography.cjs";

// eslint-disable-next-line
const plugin = require("tailwindcss/plugin") as typeof import("tailwindcss/plugin");

export type FontSizes = {
  targetLineHeight: number;
  sizesOfCap: [size: string, cap: number][];
};

export function defineFontSizes(v: FontSizes): FontSizes {
  return v;
}

export function extendWithTextSizes({
  settings,
  webFallbackFonts,
}: {
  settings: TypographySettings;
  webFallbackFonts: { [ref$: string]: string[] };
}) {
  if (!settings) throw new Error("Undefined settings");
  const utilObj: CSSRuleObject = {};
  for (const font of settings.Fonts) {
    const foundFamily = settings.FontFamilies.find((a) => a.ID === font.Value.FontFamily.ref$);
    // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
    if (!foundFamily) throw new Error(`Failed to find family by ref "${font.Value.FontFamily.ref$}"`);
    // eslint-disable-next-line @typescript-eslint/restrict-plus-operands
    utilObj[".text-" + font.ID] = {
      letterSpacing: font.Value.Tracking,
      lineHeight: font.Value.LineHeight,
      fontSize: font.Value.FontSize,
      fontFamily: [foundFamily.HTMLFontFamilyName]
        .concat(webFallbackFonts[font.Value.FontFamily.ref$] ?? [])
        .map((a) => (a.includes(" ") ? JSON.stringify(a) : a))
        .join(", "),
    };
  }

  return plugin((utils) => {
    utils.addComponents(utilObj);
  });
}
