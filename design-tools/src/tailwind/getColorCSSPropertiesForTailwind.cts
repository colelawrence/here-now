import { blueFromArgb, greenFromArgb, hexFromArgb, redFromArgb } from "@artprompt/material-color-utilities";
import type { SystemColorSettings } from "../color/systemColors.cjs";
import { getAllColorsKebab } from "src/color/getAllColorsKebab.cjs";


export function getColorCSSPropertiesForTailwind(colorSettings: SystemColorSettings, options: { dark: boolean; }) {
  const kebab = getAllColorsKebab(colorSettings, options);
  return kebab.map((a) => ({
    twID: `${a.ext ? "ext" : "sys"}-${a.id}`,
    cssProperty: `--${a.ext ? "ext" : "sys"}-color-${a.id}`,
    valueHex: hexFromArgb(a.argb),
    valueRGBSpaced: `${redFromArgb(a.argb)} ${greenFromArgb(a.argb)} ${blueFromArgb(a.argb)}`,
  }));
}
