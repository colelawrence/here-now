import type { HasInputTraversal } from "./createInputTraversal";

export function handleArrowTraversalDOM(
  traversable: HasInputTraversal,
  e: KeyboardEvent & {
    currentTarget: EventTarget & HTMLInputElement;
  },
) {
  if (e.shiftKey || e.ctrlKey || e.metaKey || e.altKey) {
    // disable behavior when modifier keys are pressed
    return false;
  }
  if (e.key === "ArrowUp") {
    traversable.inputTraversal.up();
    return true;
  } else if (e.key === "ArrowDown") {
    traversable.inputTraversal.down();
    return true;
  } else if (e.key === "ArrowLeft" && e.currentTarget.selectionStart === 0 && e.currentTarget.selectionEnd === 0) {
    traversable.inputTraversal.exitFromStart();
    return true;
  } else if (
    e.key === "ArrowRight" &&
    e.currentTarget.selectionStart === e.currentTarget.value.length &&
    e.currentTarget.selectionEnd === e.currentTarget.value.length
  ) {
    traversable.inputTraversal.exitFromEnd();
    return true;
  }

  return false;
}
