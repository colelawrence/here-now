import type { HasHtmlInput } from "./createInputTraversal";

export function attemptToFocusOnInput(sourceMaybe: HasHtmlInput | null | undefined, selectionIndex?: number) {
  let attempt = 3;
  if (sourceMaybe == null) return;
  requestAnimationFrame(run);
  const source = sourceMaybe;
  function run() {
    const input = source.htmlInputElement;
    if (input) {
      input.focus();
      const sel = Math.min(input.value.length, selectionIndex ?? Infinity);
      input.setSelectionRange(sel, sel);
      return;
    }

    if (attempt-- > 0) {
      requestAnimationFrame(run);
    }
  }
}
