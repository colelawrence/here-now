export function extend<A, B>(a: A, b: B): A & B {
  const target = {} as any;
  for (const k in a) {
    target[k] = a[k];
  }
  for (const k in b) {
    target[k] = b[k];
  }
  return target;
}
