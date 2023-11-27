export function call<R>(f: () => R): R {
  return f();
}
