import { blueFromArgb, greenFromArgb, hexFromArgb, redFromArgb } from "@material/material-color-utilities";
import { getAllColorsKebab } from "../color/getAllColorsKebab.cjs";
import type { SystemColorSettings } from "../color/systemColors.cjs";

export function getColorCSSPropertiesForTailwind(colorSettings: SystemColorSettings, options: { dark: boolean }) {
  const kebab = getAllColorsKebab(colorSettings, options);
  return kebab.map((a) => ({
    twID: `${a.ext ? "ext" : "sys"}-${a.id}`,
    cssProperty: `--${a.ext ? "ext" : "sys"}-color-${a.id}`,
    valueHex: hexFromArgb(a.argb),
    valueRGBSpaced: `${redFromArgb(a.argb)} ${greenFromArgb(a.argb)} ${blueFromArgb(a.argb)}`,
  }));
}
