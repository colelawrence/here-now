import type { Action } from "svelte/action";
import tippy, { type Props } from "tippy.js";
import "tippy.js/dist/tippy.css"; // optional for styling

export const tooltip: Action<HTMLElement, Partial<Props>> = (node, props) => {
  const tip = tippy(node, { ...props });
  return {
    update(parameter) {
      tip.setProps(parameter);
    },
    destroy() {
      tip.destroy();
    },
  };
};
