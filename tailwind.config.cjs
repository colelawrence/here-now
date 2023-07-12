// @ts-check
const contentFileFilter = __dirname + "/**/*.{rs,html.j2}";
/* eslint-disable */
const {
  addSysColors,
} = require("./design-tools/out/tailwind-addSysColors.cjs");
const {
  extendWithTextSizes,
} = require("./design-tools/out/tailwind-apTypography.cjs");
const {
  addItemStateVariants,
} = require("./design-tools/out/tailwind-addItemStateVariants.cjs");
const {
  designSystemTypographySettings,
  designSystemColorSettings,
} = require("./design-tools/out/design-system-settings.cjs");

// console.log(JSON.stringify(designSystemTypographySettings, null, 2))

/** @type {Partial<import("tailwindcss").Config>} */
const config = {
  // Note that this selection determines global.css hot reload
  // so, this is why if any tsx file changes, then the entire tailwind
  // gets re-generated...
  content: [contentFileFilter],
  safelist: [
    {
      pattern: /^text-(ui|content|title|mono)/,
      variants: [],
    },
  ],
  theme: {
    extend: {},
    // remove sizes which are replaced by {extendWithTextSizes}
    fontSize: {},
    fontFamily: {},
  },
  plugins: [
    // see color-emoji.css which specifies the custom font-family
    extendWithTextSizes({
      settings: designSystemTypographySettings,
      webFallbackFonts: {
        ui: ["system-ui", "sans-serif", "color-emoji"],
        content: ["system-ui", "sans-serif", "color-emoji"],
        title: ["apui", "system-ui", "color-emoji"],
        mono: ["monospace", "color-emoji"],
      },
    }),
    addSysColors({
      settings: designSystemColorSettings,
    }),
    addItemStateVariants({ settings: {} }),
  ],
};

module.exports = config;
