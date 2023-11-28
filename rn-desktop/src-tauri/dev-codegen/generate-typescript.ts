import { Code } from "https://deno.land/x/derive_codegen@v0.0.4-1/mod.ts";
import * as gen from "./gen.ts";

const args = {
  outputFileName: "../src/lib/rust-codegen/ui.gen.ts",
  prependText: `type UID = string;
function createInvoker(invoke: Function, prefix = ""): any {
  return new Proxy({}, {
    get(target, command, receiver) {
      if (typeof command !== "string") throw new TypeError("Expected string command");
      return function (options: any) {
        return invoke(prefix + command, options);
      }
    },
  })
}
// TODO: make this generic for other plugins
export function createRightNowInvoker(invoke: Function): RightNowStateInvoke {
  return createInvoker(invoke, "plugin:right-now-state|");
}
`,
  includeLocationsRelativeTo: "../../../",
  jsonInput: Deno.args.findLast(Boolean),
};

function convert(input: gen.Input): gen.Output {
  const outputFiles = new Map<string, { importIdent: string }>();
  const generated = new Code();

  console.error("Number of functions: ", input.functions.length);

  generated.add`export interface RightNowStateInvoke {`;
  generated.scope(($fns) => {
    for (const fnDecl of input.functions) {
      if (!fnDecl.codegen_flags?.tauri_command) {
        console.error("Skipping non-tauri command", fnDecl);
        continue;
      }

      const tauriPluginIDOpt = fnDecl.codegen_attrs?.tauri_plugin?.[0];
      if (tauriPluginIDOpt !== "right-now-state") {
        console.error("Skipping non-`right-now-state` command", fnDecl);
        continue;
      }
      // const commandPrefix = tauriPluginIDOpt ? `plugin:${tauriPluginIDOpt}|` : "";

      const { is_async: _, params: allParams, return_type, self_opt } = fnDecl.function;
      // remove tauri provided params
      const params = allParams.filter((p) => p.id !== "app" && p.id !== "window");

      if (self_opt) {
        console.error("Skipping self_opt", fnDecl);
      }
      $fns.addDocString(fnDecl);
      const ret = createFormat(return_type);
      const retSrc = `Promise<${ret.optional ? `undefined | ${ret.src}` : ret.src}.Ok["Ok"]>`;
      if (params.length === 0) {
        $fns.add`${ident(fnDecl.id)}(): ${retSrc}`;
      } else {
        $fns.add`${ident(fnDecl.id)}(options: {`;
        $fns.scope(($params) => {
          for (const param of params) {
            const { src, optional } = createFormat(param.format);
            $params.addDocString(param);
            $params.add`${param.id}${optional ? "?" : ""}: ${src},`;
          }
        });
        $fns.add`}): ${retSrc}`;
      }
    }
  });
  generated.add`}`;

  const seen = new Set<string>();
  console.error("Number of declarations: ", input.declarations.length);
  for (const decl of input.declarations) {
    if (seen.has(decl.id)) {
      console.warn("Duplicate declaration", decl.id, decl.id_location);
      continue;
    }
    seen.add(decl.id);

    const docs = Code.docString(
      decl,
      undefined,
      args.includeLocationsRelativeTo != null ? [args.includeLocationsRelativeTo, decl.id_location] : undefined,
    );

    const $decl = new Code();
    // Part of generics decl
    const generics = decl.rust_generics?.length ? `<${decl.rust_generics.map((g) => g[0]).join(", ")}>` : "";
    // after any existing generics
    const genericsCont = decl.rust_generics?.length ? `, ${decl.rust_generics.map((g) => g[0]).join(", ")}` : "";

    gen.ContainerFormat.match(decl.container_kind, {
      Struct({ fields }) {
        const structIdent = ident(decl.id);
        // type
        $decl.lines.push(...docs);
        if (decl.codegen_flags?.ts_interface_merge) {
          $decl.add`export interface ${structIdent}${generics} {`;
          typeFieldsFinish$($decl, fields, "}");
        } else {
          $decl.add`export type ${structIdent}${generics} = {`;
          typeFieldsFinish$($decl, fields);
          // create
          $decl.lines.push(...docs);
          $decl.add`export function ${structIdent}${generics}(inner: ${structIdent}${generics}): ${structIdent}${generics} {`;
          $decl.ad1`return inner;`;
          $decl.add`}`;
        }
      },
      Enum({ repr, variants }) {
        const enumIdent = ident(decl.id);
        const $nsMatchToObj = new Code(["// callbacks"]);
        const $nsMatchIfStrs = new Code(["// if-else strings"]);
        const $nsMatchIfObjs = new Code([
          "// if-else objects",
          `if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");`,
        ]);
        const $ns = new Code([
          `export type ApplyFns<R${genericsCont}> = {`,
          $nsMatchToObj,
          `}`,
          `/** Match helper for {@link ${enumIdent}} */`,
          `export function apply<R${genericsCont}>(`,
          new Code([`to: ApplyFns<R${genericsCont}>,`]),
          `): (input: ${enumIdent}${generics}) => R {`,
          new Code([
            `return function _match(input): R {`,
            $nsMatchIfStrs,
            $nsMatchIfObjs,
            new Code([
              `const _exhaust: never = input;`,
              `throw new TypeError("Unknown object when expected ${enumIdent}");`,
            ]),
            `}`,
          ]),
          `}`,
          `/** Match helper for {@link ${enumIdent}} */`,
          `export function match<R>(`,
          new Code([`input: ${enumIdent}${generics},`, `to: ApplyFns<R${genericsCont}>,`]),
          `): R {`,
          new Code([`return apply(to)(input)`]),
          `}`,
        ]);
        const typeCode = new Code([
          ...docs,
          // create / matchers
          `// deno-lint-ignore no-namespace`,
          `export namespace ${enumIdent} {`,
          $ns,
          `}`,
          // type
          ...docs,
          `export type ${enumIdent}${generics} =`,
        ]);

        // TODO: handle different representations properly

        if (decl.codegen_flags?.svelte_enum) {
          console.error("This is a svelte enum", decl);
        }

        for (const variant of variants) {
          const variantIdent = ident(variant.id);
          const variantNameField = namedField(variant);
          const variantIdentRef = `${enumIdent}.${variantIdent}`;
          typeCode.ad1`| ${variantIdentRef}`;
          const variantDocs = Code.docString(variant);
          gen.VariantFormat.match(variant.variant_format, {
            NewType(format) {
              const newTypeTs = createFormat(format);
              // type
              $ns.lines.push(...variantDocs);
              $ns.add`export type ${variantIdent}${generics} = {`;
              $ns.indented().lines.push(...variantDocs);
              $ns.ad1`${variantNameField}: ${newTypeTs.src}`;
              $ns.add`};`;
              // create
              $ns.lines.push(...variantDocs);
              $ns.add`export function ${variantIdent}${generics}(value${newTypeTs.optional && "?"}: ${
                newTypeTs.src
              }): ${variantIdent}${generics} {`;
              $ns.ad1`return { ${variantNameField}: value };`;
              $ns.add`}`;
              // match callback
              $nsMatchToObj.lines.push(...variantDocs);
              $nsMatchToObj.add`${variantIdent}(inner: ${variantIdent}${generics}[${namedStr(variant)}]): R;`;
              // match if else
              $nsMatchIfObjs.add`if (${namedStr(variant)} in input) return to.${variantNameField}(input[${namedStr(
                variant,
              )}]);`;
            },
            Tuple(formats) {
              const formatTsList = tupleFormats(formats);
              const vnStr = namedStr(variant);
              const innerTypeRef = `[${formatTsList.map((f) => f.fmt.src).join(", ")}]`;
              // type
              $ns.lines.push(...variantDocs);
              $ns.add`export type ${variantIdent}${generics} = { ${variantNameField}: ${innerTypeRef} };`;
              $ns.lines.push(...variantDocs);
              // create
              $ns.add`export function ${variantIdent}${generics}(${formatTsList
                .map((f) => `${f.id}: ${f.fmt.src}`)
                .join(", ")}): ${variantIdent}${generics} {`;
              $ns.ad1`return { ${variantNameField}: [${formatTsList.map((f) => f.id).join(", ")}] };`;
              $ns.add`}`;
              // match
              $nsMatchToObj.lines.push(...variantDocs);
              $nsMatchToObj.add`${variantIdent}(inner: ${innerTypeRef}): R,`;
              $nsMatchIfObjs.add`if (${vnStr} in input) return to.${variantIdent}(input[${vnStr}]);`;
            },
            Unit() {
              // type
              $ns.lines.push(...variantDocs);
              $ns.add`export type ${variantIdent}${generics} = ${namedStr(variant)}`;
              // create
              $ns.lines.push(...variantDocs);
              $ns.add`export function ${variantIdent}${generics}(): ${variantIdent}${generics} {`;
              $ns.ad1`return ${namedStr(variant)};`;
              $ns.add`}`;
              // match
              $nsMatchToObj.lines.push(...variantDocs);
              $nsMatchToObj.add`${variantIdent}(): R,`;
              $nsMatchIfStrs.add`if (input === ${namedStr(variant)}) return to.${variantIdent}();`;
            },
            Struct({ fields }) {
              const innerTypeRef = `${variantIdent}[${namedStr(variant)}]`;
              // type
              $ns.lines.push(...variantDocs);
              $ns.add`export type ${variantIdent}${generics} = {`;
              $ns.scope(($$) => {
                $$.lines.push(...variantDocs);
                $$.add`${variantNameField}: {`;
                typeFieldsFinish$($$, fields);
              });
              $ns.add`};`;
              // create
              $ns.lines.push(...variantDocs);
              $ns.add`export function ${variantIdent}${generics}(value: ${innerTypeRef}): ${variantIdent} {`;
              $ns.ad1`return { ${variantNameField}: value }`;
              $ns.add`}`;
              // match
              $nsMatchToObj.lines.push(...variantDocs);
              $nsMatchToObj.add`${variantIdent}(inner: ${innerTypeRef}): R,`;
              $nsMatchIfObjs.add`if (${namedStr(variant)} in input) return to.${variantIdent}(input[${namedStr(
                variant,
              )}]);`;
            },
          });
        }

        $decl.lines.push(...typeCode.lines);
      },
      NewTypeStruct(format) {
        const structIdent = ident(decl.id);
        const newTypeFormat = createFormat(format);
        // type
        $decl.lines.push(...docs);
        if (decl.serde_flags?.transparent) {
          $decl.add`export type ${structIdent}${generics} = ${newTypeFormat.src}`;
        } else {
          $decl.add`export type ${structIdent}${generics} = [${newTypeFormat.src}]`;
        }
        // create
        $decl.lines.push(...docs);
        $decl.add`export function ${structIdent}${generics}(inner${newTypeFormat.optional && "?"}: ${
          newTypeFormat.src
        }): ${structIdent}${generics} {`;
        if (decl.serde_flags?.transparent) {
          $decl.ad1`return inner;`;
        } else {
          $decl.ad1`return [inner];`;
        }
        $decl.add`}`;
      },
      TupleStruct(formats) {
        const formatTsList = tupleFormats(formats);
        const structIdent = ident(decl.id);
        // type
        $decl.lines.push(...docs);
        $decl.add`export type ${structIdent}${generics} = [${formatTsList.map((f) => f.fmt.src).join(", ")}]`;
        // create
        $decl.lines.push(...docs);
        $decl.add`export function ${structIdent}${generics}(${formatTsList
          .map((f) => `${f.id}: ${f.fmt.src}`)
          .join(", ")}): ${structIdent}${generics} {`;
        $decl.ad1`return [${formatTsList.map((f) => f.id).join(", ")}];`;
        $decl.add`}`;
      },
      UnitStruct() {
        const structIdent = ident(decl.id);
        // type
        $decl.lines.push(...docs);
        $decl.add`export interface ${structIdent}${generics} {} /* hmm unit struct? */`;
        // create
        $decl.lines.push(...docs);
        $decl.add`export function ${structIdent}${generics}(): ${structIdent} {`;
        $decl.ad1`return {};`;
        $decl.add`}`;
      },
    });

    generated.lines.push(...$decl.lines);
  }

  if (args.prependText) {
    generated.lines.unshift(args.prependText);
  }

  const sharedOutputFile = args.outputFileName ?? "_shared.gen.ts";

  return {
    errors: [],
    files: [
      {
        path: sharedOutputFile,
        source: generated.toString(),
      },
      ...Array.from(outputFiles.entries()).map(([name, { importIdent }]) => ({
        path: name,
        source: new Code([`export { ${importIdent} } from "./${sharedOutputFile}";`]).toString(),
      })),
    ],
    warnings: [],
  };
}

function splitByFlattened<T extends gen.Attrs>(items: T[]) {
  const flattened: T[] = [];
  const fields: T[] = [];
  for (const item of items) {
    if (item.serde_flags?.flatten) {
      flattened.push(item);
    } else {
      fields.push(item);
    }
  }
  return { flattened, fields };
}

/** Only for the types */
function typeFieldsFinish$(root: Code, fields: gen.NamedField[], finish: "}" | "};" = "};") {
  const $ = root.indented();
  const split = splitByFlattened(fields);
  for (const field of split.fields) {
    const { src, optional } = createFormat(field.format);
    const isOptional = optional || (field.serde_flags?.default && field.serde_attrs?.skip_serializing_if);
    $.addDocString(field);
    if (field.codegen_attrs?.ts_as) {
      $.add`${namedField(field)}${isOptional && "?"}: ${field.codegen_attrs.ts_as[0]};`;
    } else {
      $.add`${namedField(field)}${isOptional && "?"}: ${src}${isOptional && " | null | undefined"};`;
    }
  }

  if (split.flattened.length === 0) {
    root.lines.push(finish);
    return;
  }
  root.add`} // flattened fields:`;
  for (const flattened of split.flattened) {
    root.addDocString(flattened, `Flattened from \`.${flattened.id}\`.`);
    const format = createFormat(flattened.format);
    if (format.optional) {
      root.add`& Partial<${format.src}>`;
    } else {
      root.add`& ${format.src}`;
    }
  }
  root.lastLine += ";";
}

function tupleFormats(formats: gen.Format[]) {
  const aCp = "a".codePointAt(0)!;
  return formats.map((f, idx) => ({
    fmt: createFormat(f),
    id: String.fromCodePoint(aCp + idx),
  }));
}

const num = () => ({ src: "number" });

const createFormat: (format: gen.Format) => {
  src: string;
  optional?: boolean;
} = gen.Format.apply({
  TypeName: (value) => {
    const generics = value.generics.length ? `<${value.generics.map((g) => createFormat(g).src).join(", ")}>` : "";
    return { src: `${ident(value.ident)}${generics}` };
  },
  I8: num,
  I16: num,
  I32: num,
  I64: num,
  I128: num,
  ISIZE: num,
  U8: num,
  U16: num,
  U32: num,
  U64: num,
  U128: num,
  USIZE: num,
  F32: num,
  F64: num,
  Bool: () => ({ src: "boolean" }),
  Bytes: () => ({ src: "/* bytes? */ string" }),
  Never: () => ({ src: "never" }),
  Char: () => ({ src: "/* char */ string" }),
  Map: ({ key, value }) => ({
    src: `Record<${createFormat(key).src}, ${createFormat(value).src}>`,
  }),
  Unit: () => ({ src: "/* unit */ null" }),
  Option: (format) => {
    const inner = createFormat(format);
    if (inner.optional) return inner;
    return {
      src: `${inner.src} | undefined | null`,
      optional: true,
    };
  },
  Incomplete: ({ debug }) => ({ src: `/* Incomplete: ${debug} */ unknown` }),
  Seq: (seq) => ({
    src: `Array<${createFormat(seq).src}>`,
  }),
  Tuple: (tuple) => ({
    src: `[${tuple.map((tup) => createFormat(tup).src).join(", ")}]`,
  }),
  TupleArray: ({ content, size }) => ({
    src: `[<${new Array(size).fill(createFormat(content).src).join(", ")}]`,
  }),
  Str: () => ({ src: "string" }),
});

function ident(id: string): string {
  return id.replace(/[^a-zA-Z0-9\$\_]/g, "$").replace(/^(\d)/, "$$1");
}

function namedField(named: { id: string } & gen.Attrs): string {
  const nam = named.serde_attrs?.["rename"]?.[0] ?? named.id;
  if (/^[\w$][\w\d$]*$/.test(nam)) return nam;
  else return JSON.stringify(nam);
}

function namedStr(named: { id: string } & gen.Attrs): string {
  const nam = named.serde_attrs?.["rename"]?.[0] ?? named.id;
  return JSON.stringify(nam);
}

console.log(JSON.stringify(convert(JSON.parse(args.jsonInput!)), null, 2));
