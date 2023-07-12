/* eslint-disable */

const plugin = require("tailwindcss/plugin") as typeof import("tailwindcss/plugin");

export function addItemStateVariants(options: { settings: {} }) {
  // const items = [{ Name: "generator-page", States: ["full-size"] }];
  const states = [
    { Name: "mini-1", Dev: "This item is mini size / tiny content (1)" },
    { Name: "dev-layout", Dev: "Show dev sizing info" },
  ];
  // See https://tailwindcss.com/docs/plugins#adding-variants / https://v2.tailwindcss.com/docs/plugins#adding-variants
  return plugin(({ theme, matchUtilities, addComponents, addVariant, e }) => {
    for (const { Name, Dev } of states) {
      // addComponents({
      //   [`.is-${Name}`]: {
      //     "--dev-parent-item": Dev,
      //   },
      // });
      // for (const state of States) {
      //   const itemClassStateActive = `.is-${Name}-${state}`;
      const itemClassStateActive = `.is-${Name}`;
      addComponents({
        [itemClassStateActive]: {
          [`--dev-parent-state-${Name}`]: Dev,
        },
      });

      // // V3 Doc examples:
      // addVariant('optional', '&:optional')
      // addVariant('hocus', ['&:hover', '&:focus'])
      // addVariant('supports-grid', '@supports (display: grid)')
      // // Or child selectors
      // addVariant('group-optional', ':merge(.group):optional &')
      // addVariant('peer-optional', ':merge(.peer):optional ~ &')
      // addVariant(`${Name}-${state}`, `${itemClassStateActive} &`);
      addVariant(Name, `${itemClassStateActive} &`);
      // }
    }
  });
}
