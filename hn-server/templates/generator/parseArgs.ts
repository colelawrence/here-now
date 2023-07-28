export function parseArgs<T extends Record<string, string>>(namedArgs: T) {
  const unrecognized: string[] = [];
  const recognized: Partial<Record<keyof T, string>> = {};
  const inputJSON = Deno.args.findLast(Boolean);
  for (const arg of Deno.args.slice(0, -1)) {
    if (arg.startsWith("--")) {
      const eqIdx = arg.indexOf("=");
      if (eqIdx === -1)
        throw new Error(
          `Expected = as delimiter in arg: ${JSON.stringify(arg)}`
        );
      const key = arg.slice(2, eqIdx);
      if (key in namedArgs) {
        recognized[key as keyof T] = arg.slice(eqIdx + 1);
      } else {
        unrecognized.push(arg);
      }
    } else {
      unrecognized.push(arg);
    }
  }
  if (unrecognized.length)
    throw new Error(
      `Unrecognized arguments ${JSON.stringify(
        unrecognized,
        null,
        4
      )}, allowed args: ${JSON.stringify(namedArgs, null, 2)}`
    );

  return {
    jsonInput: inputJSON,
    ...recognized,
  };
}
