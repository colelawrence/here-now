import type { Theme } from "@artprompt/material-color-utilities";
import { hexFromArgb, themeFromImage, argbFromHex, themeFromSourceColor } from "@artprompt/material-color-utilities";

export type ExtendedColors = {
  ID: string;
  Seed: {
    Hex: string;
  };
}[];

export type ColorConfig = {
  Primary: { Hex: string };
  extended: ExtendedColors;
};

export type SystemColorSettings = {
  mvpTheme: Theme;
  config: ColorConfig;
};

export function generateColorSettings(colorConfig: ColorConfig): SystemColorSettings {
  // Get the theme from a hex color
  return {
    config: colorConfig,
    mvpTheme: themeFromSourceColor(argbFromHex(colorConfig.Primary.Hex), [...customColorsFromColorConfig(colorConfig)]),
  };
}

export async function generateColorSettingsFromHTMLImage(
  image: HTMLImageElement,
  colorConfig: Omit<ColorConfig, "Primary">,
): Promise<SystemColorSettings> {
  const theme = await themeFromImage(image, [...customColorsFromColorConfig(colorConfig)]);
  // Get the theme from a hex color
  return {
    config: {
      Primary: { Hex: hexFromArgb(theme.source) },
      ...colorConfig,
    },
    mvpTheme: theme,
  };
}

function customColorsFromColorConfig(colorConfig: Pick<ColorConfig, "extended">) {
  return colorConfig.extended.map((ext) => ({
    name: ext.ID,
    value: argbFromHex(ext.Seed.Hex),
    // I think this is "harmonize"
    blend: true,
  }));
}
