"use strict";
/* eslint-disable */
Object.defineProperty(exports, "__esModule", { value: true });
exports.addSysColors = void 0;
const getColorCSSPropertiesForTailwind_cjs_1 = require("./getColorCSSPropertiesForTailwind.cjs");
const plugin = require("tailwindcss/plugin");
/** Only works with px values at the moment */
function addSysColors(options) {
    const colors = {
        transparent: "transparent",
    };
    for (const { twID, cssProperty, valueRGBSpaced } of (0, getColorCSSPropertiesForTailwind_cjs_1.getColorCSSPropertiesForTailwind)(options.settings, { dark: false })) {
        // Example proof of concept for opacityVariable / opacityValue
        // https://github.com/adamwathan/tailwind-css-variable-text-opacity-demo/blob/master/tailwind.config.js
        colors[twID] = ({ opacityVariable, opacityValue }) => {
            if (opacityValue !== undefined) {
                // return `rgba(var(--color-primary), ${opacityValue})`
                return `rgb(var(${cssProperty}, ${valueRGBSpaced}) / ${opacityValue})`;
            }
            if (opacityVariable !== undefined) {
                // return `rgba(var(--color-primary), var(${opacityVariable}, 1))`
                return `rgb(var(${cssProperty}, ${valueRGBSpaced}) / var(${opacityVariable}, 1))`;
            }
            return `rgb(var(${cssProperty}, ${valueRGBSpaced}))`;
        };
    }
    return plugin(({ theme, matchUtilities, addComponents }) => {
        addComponents({
            ".themed": {
                "transition-property": "color, background-color, border-color, text-decoration-color, fill, stroke",
                "transition-timing-function": "cubic-bezier(0.4, 0, 0.2, 1)",
                "transition-duration": "1000ms",
            },
        });
        // // Want to match utilities so the background be overwritten
        // matchUtilities(
        //   {
        //     bg: (value) => ({
        //       "--tw-bg-opacity": "1",
        //       "background-color": value,
        //     }),
        //     // TODO: Border colors?
        //     text: (value) => ({
        //       "--tw-text-opacity": "1",
        //       // kinda janky that we override the existing color rule
        //       color: value.replace("--tw-bg-opacity", "--tw-text-opacity"),
        //     }),
        //   },
        //   {
        //     values: theme("colors"),
        //   },
        // );
        // matchUtilities({
        //   bg: (value) => {
        //     console.log(value);
        //     return {
        //       transition: "back"
        //     };
        //   },
        // });
    }, {
        // types error
        content: undefined,
        theme: {
            // @ts-ignore ... doesn't seem to be documented
            colors,
        },
    });
}
exports.addSysColors = addSysColors;
//# sourceMappingURL=tailwind-addSysColors.cjs.map