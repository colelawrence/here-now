import { noCase } from "no-case";
import type { SystemColorSettings } from "./systemColors.cjs";


export function getAllColorsKebab(
  { mvpTheme }: SystemColorSettings,
  options: { dark: boolean; }
) {
  const scheme = options.dark ? mvpTheme.schemes.dark : mvpTheme.schemes.light;
  return [
    ...Object.entries(scheme.toJSON()).map(([name, argb]) => {
      const kebab = noCase(name, { delimiter: "-" });
      return { id: kebab, argb: argb, ext: false };
    }),
    ...mvpTheme.customColors.flatMap((custom) => {
      const kebab = noCase(custom.color.name, { delimiter: "-" });
      const scheme = options.dark ? custom.dark : custom.light;
      return [
        { id: kebab, argb: scheme.color, ext: true },
        { id: `on-${kebab}`, argb: scheme.onColor, ext: true },
        { id: `${kebab}-container`, argb: scheme.colorContainer, ext: true },
        {
          id: `on-${kebab}-container`,
          argb: scheme.onColorContainer,
          ext: true,
        },
      ];
    }),
  ];
}
