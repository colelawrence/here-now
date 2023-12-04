import type { ExtendedColors, SystemColorSettings } from "./color/systemColors.cjs";
import { generateColorSettings, generateColorSettingsFromHTMLImage } from "./color/systemColors.cjs";
import { artpromptExtendedColors } from "./design-system-settings.cjs";

declare global {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface HTMLImageElement {}
}

export default "Artprompt design";
export type { Scheme, Theme } from "@material/material-color-utilities";
export type { SystemColorSettings } from "./color/systemColors.cjs";
export type { TrackingRule, TypographyConfig, TypographySettings } from "./typography/systemTypography.cjs";

export { hexFromArgb } from "@material/material-color-utilities";
export {
  artpromptExtendedColors,
  designSystemColorSettings,
  designSystemTypographySettings,
} from "./design-system-settings.cjs";

export function createArtpromptColorsFromSourceHex(hex: string, extended?: ExtendedColors): SystemColorSettings {
  return generateColorSettings({
    Primary: { Hex: hex },
    extended: extended ?? artpromptExtendedColors,
  });
}

export function createArtpromptColorsFromHTMLImage(
  image: HTMLImageElement,
  extended?: ExtendedColors,
): Promise<SystemColorSettings> {
  return generateColorSettingsFromHTMLImage(image, {
    extended: extended ?? artpromptExtendedColors,
  });
}
