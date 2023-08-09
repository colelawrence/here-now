export function ch<
  T extends Record<
    string, {
      In: { factory(value: (value: unknown) => unknown): unknown; };
      Out: { factory(value: (value: unknown) => unknown): unknown; };
    }
  >
>(
  obj: T
): {
  in: {
    [P in keyof T]: ReturnType<T[P]["In"]["factory"]>;
  };
  expect(
    fn: (drivers: {
      [P in keyof T]: ReturnType<T[P]["Out"]["factory"]>;
    }) => void
  ): void;
} {
  throw new Error("non-existent");
}
