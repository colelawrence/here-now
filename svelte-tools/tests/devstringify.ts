import { tightJsonStringify } from "./tightJsonStringify.ts";

/**
 * Stringifies any value given. If an object is given and `indentJSON` is true,
 * then a developer-readable, command line friendly (not too spaced out, but with
 * enough whitespace to be readable).
 */
export function devStringify(input: any, display = true): string {
  try {
    if (typeof input === "string") {
      if (input[0] === "{" || input[0] === "[") {
        try {
          return devStringify(JSON.parse(input), display);
        } catch {
          // I guess it wasn't JSON. No biggie!
        }
      }
      return input;
    } else if (typeof input === "function") {
      return input.toString();
    } else {
      const replacer = (_key: string, value: any): any => {
        try {
          if (value && value.toJSON === undefined) {
            if (value instanceof Error) {
              return {
                error: value.toString(),
                stack: value.stack ?? null,
                // @ts-ignore
                cause: value.cause ? replacer("cause", value.cause) : undefined,
              };
            }
          }
        } catch {}

        return value;
      };
      const json = tightJsonStringify(input, replacer);
      return display ? cleanNewlinesAndStacks(json.replace(/(\\?")([^"]+)\1:/g, "$2:")) : json;
    }
  } catch (err) {
    return input?.name || String(input);
  }
}

function cleanNewlinesAndStacks(stack: string): string {
  // return stack;
  return (
    stack
      // replace private paths before node_modules
      .replace(/(\(|\sat )\/[^\)\s]+node_modules\//gm, "$1node_modules/")
      // replace escaped newlines in strings
      // .replace(/^(.+?)"(.*\\n(.(?!\\"))+|\\")*"$/gm, (_fullMatch, beforeQuote, inside) => {
      .replace(/([^"]+?)"((?:\\.|[^\"])*)"/g, (_fullMatch, beforeQuote, inside: string | undefined) => {
        return (
          beforeQuote +
          (inside
            ? `"${inside
                .split(/\\n/g)
                // .map((line) => line.replace(/\\"/g, '"'))
                .join("\n" + " ".repeat(beforeQuote.length))}"`
            : '""')
        );
      })
  );
}
