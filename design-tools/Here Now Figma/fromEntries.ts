export function fromEntries<V>(...valueArgs: [string, V][][]): Record<string, V> {
  const result: Record<string, V> = {};
  for (let i = 0; i < valueArgs.length; i++) {
    const valueArg = valueArgs[i];
    for (let j = 0; j < valueArg.length; j++) {
      const [key, value] = valueArg[j];
      result[key] = value;
    }
  }
  return result;
}
