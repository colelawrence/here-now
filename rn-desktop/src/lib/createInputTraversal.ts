import { attemptToFocusOnInput } from "./attemptToFocusOnInput";

export type HasHtmlInput = {
  htmlInputElement: HTMLInputElement | null;
};
export type HasInputTraversal = {
  inputTraversal: InputTraversal;
};

export type InputTraversal = {
  up(): void;
  down(): void;
  exitFromStart(): void;
  exitFromEnd(): void;
};

export function createInputTraversal(getInputs: () => HasHtmlInput[]) {
  function currentSelectionOr(self: HasHtmlInput): number | null {
    return self.htmlInputElement?.selectionStart ?? null;
  }
  function getRelativeInput(self: HasHtmlInput, dir: number): HasHtmlInput {
    const inputs = getInputs();
    const index = inputs.indexOf(self);
    const isLast = index === inputs.length - 1;
    if (isLast && dir > 0) {
      // console.log({ index, dir, inputs });
      // focus on first input
      return inputs[0];
    }
    const isFirst = index === 0;
    if (isFirst && dir < 0) {
      // focus on last input
      return inputs[inputs.length - 1];
    }

    return inputs[index + (dir % inputs.length)];
  }

  return {
    getEscapeInput(getSelf: () => HasHtmlInput): InputTraversal {
      return {
        down() {
          const self = getSelf();
          attemptToFocusOnInput(getRelativeInput(self, 1), currentSelectionOr(self) ?? 0);
        },
        up() {
          const self = getSelf();
          attemptToFocusOnInput(getRelativeInput(self, -1), currentSelectionOr(self) ?? Infinity);
        },
        exitFromStart() {
          const self = getSelf();
          attemptToFocusOnInput(getRelativeInput(self, -1), Infinity);
        },
        exitFromEnd() {
          const self = getSelf();
          attemptToFocusOnInput(getRelativeInput(self, 1), 0);
        },
      };
    },
  };
}
