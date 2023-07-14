import * as output from "./output.gen.ts";

const out = output.TypographyExport(JSON.parse(Deno.args.findLast(Boolean)!));
// console.log(JSON.stringify(settings));
console.log(out);
