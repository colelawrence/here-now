/** marker used with {@link assign} */
export const KEEP = {} as any;
export function assign<T>(on: T, wit: Partial<T>) {
  for (const key in wit) {
    if (wit[key] !== KEEP) {
      // @ts-ignore
      on[key] = wit[key];
    }
  }
}
