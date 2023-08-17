import { Code, gen } from "https://deno.land/x/derive_codegen@v0.0.4-2/mod.ts"
import { parse } from "https://deno.land/std@0.194.0/flags/mod.ts"

const {
  _: [jsonInput],
  ...args
} = parse(Deno.args, {
  string: ["fileName", "includeLocationsRelativeTo"],
  default: {
    fileName: "protocol/is/driver.v0.gen.ts",
    // for example "../" or ""
    includeLocationsRelativeTo: undefined,
  },
})

Code.docStringSettings.skip_rust_attrs = true

function convert(input: gen.Input): gen.Output {
  const outputFiles = new Map<string, { importIdent: string }>()
  const generated = new Code()

  console.error("Number of declarations: ", input.declarations.length)
  for (const decl of input.declarations) {
    const docs = Code.docString(
      decl,
      undefined,
      args.includeLocationsRelativeTo != null
        ? [args.includeLocationsRelativeTo, decl.id_location]
        : undefined,
    )

    if (decl.codegen_attrs?.template) {
      outputFiles.set(decl.codegen_attrs.template[0] + ".props.ts", {
        importIdent: ident(decl.id),
      })
    }

    if (decl.codegen_attrs?.as) {
      const scalarIdent = ident(decl.id)
      generated.lines.push(...docs)
      generated.add`export type ${scalarIdent} = ${decl.codegen_attrs.as[0]};`
      generated.lines.push(...docs)
      generated.add`export function ${scalarIdent}(value: ${scalarIdent}): ${scalarIdent} {`
      generated.ad1`return value;`
      generated.add`}`
      // skipped for as attr
      continue
    }

    const $decl = new Code()
    // Part of generics decl
    const generics = decl.rust_generics?.length
      ? `<${decl.rust_generics.map((g) => g[0]).join(", ")}>`
      : ""
    // after any existing generics
    const genericsCont = decl.rust_generics?.length
      ? `, ${decl.rust_generics.map((g) => g[0]).join(", ")}`
      : ""

    gen.ContainerFormat.match(decl.container_kind, {
      Struct({ fields }) {
        const structIdent = ident(decl.id)
        // type
        $decl.lines.push(...docs)
        if (decl.codegen_flags?.ts_interface_merge) {
          $decl.add`export interface ${structIdent}${generics} {`
          typeFieldsFinish$($decl, fields, "}")
        } else {
          $decl.add`export type ${structIdent}${generics} = {`
          typeFieldsFinish$($decl, fields)
          // create
          $decl.lines.push(...docs)
          $decl.add`export function ${structIdent}${generics}(inner: ${structIdent}${generics}): ${structIdent}${generics} {`
          $decl.ad1`return inner;`
          $decl.add`}`
        }
      },
      Enum({ repr, variants }) {
        checkEnum(repr)
        const enumIdent = ident(decl.id)
        const $nsFactory = new Code(["// factory"])
        const $nsMatchToObj = new Code(["// callbacks"])
        const $nsMatchIfStrs = new Code(["// if-else strings"])
        const $nsMatchIfObjs = new Code([
          "// if-else objects",
          `if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");`,
        ])
        const $ns = new Code([
          `export type ApplyFns<R = void${genericsCont}> = {`,
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
            `};`,
          ]),
          `}`,
          `/** Factory helper for {@link ${enumIdent}} */`,
          `export function factory<R${genericsCont}>(fn: (value: ${enumIdent}) => R): ApplyFns<R${genericsCont}> {`,
          new Code([`return {`, $nsFactory, `};`]),
          `}`,
          `/** Match helper for {@link ${enumIdent}} */`,
          `export function match<R${genericsCont}>(`,
          new Code([
            `input: ${enumIdent}${generics},`,
            `to: ApplyFns<R${genericsCont}>,`,
          ]),
          `): R {`,
          new Code([`return apply(to)(input)`]),
          `}`,
        ])
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
        ])

        // TODO: handle different representations properly

        if (decl.codegen_flags?.svelte_enum) {
          console.error("This is a svelte enum", decl)
        }

        for (const variant of variants) {
          const variantIdent = ident(variant.id)
          const variantNameField = namedField(variant)
          const variantIdentRef = `${enumIdent}.${variantIdent}`
          typeCode.ad1`| ${variantIdentRef}`
          const variantDocs = Code.docString(variant)

          $nsFactory.add`${variantIdent}(value) {`
          $nsFactory.ad1`return fn(${variantIdent}(value));`
          $nsFactory.add`},`

          gen.VariantFormat.match(variant.variant_format, {
            NewType(format) {
              const newTypeTs = createFormat(format)
              // type
              $ns.lines.push(...variantDocs)
              $ns.add`export type ${variantIdent}${generics} = {`
              $ns.indented().lines.push(...variantDocs)
              $ns.ad1`${variantNameField}: ${newTypeTs.src}`
              $ns.add`};`
              // create
              $ns.lines.push(...variantDocs)
              $ns.add`export function ${variantIdent}${generics}(value${
                newTypeTs.optional && "?"
              }: ${newTypeTs.src}): ${variantIdent}${generics} {`
              $ns.ad1`return { ${variantNameField}: value };`
              $ns.add`}`
              // create content
              $ns.lines.push(...variantDocs)
              $ns.add`${variantIdent}.content = ${generics}(value${
                newTypeTs.optional && "?"
              }: ${newTypeTs.src}) => value;`
              // match callback
              $nsMatchToObj.lines.push(...variantDocs)
              $nsMatchToObj.add`${variantIdent}(inner: ${variantIdent}${generics}[${namedStr(
                variant,
              )}]): R;`
              // match if else
              $nsMatchIfObjs.add`if (${namedStr(
                variant,
              )} in input) return to.${variantNameField}(input[${namedStr(
                variant,
              )}]);`
            },
            Tuple(formats) {
              const formatTsList = tupleFormats(formats)
              const vnStr = namedStr(variant)
              const innerTypeRef = `[${formatTsList
                .map((f) => f.fmt.src)
                .join(", ")}]`
              // type
              $ns.lines.push(...variantDocs)
              $ns.add`export type ${variantIdent}${generics} = { ${variantNameField}: ${innerTypeRef} };`
              $ns.lines.push(...variantDocs)
              // create
              $ns.add`export function ${variantIdent}${generics}(${formatTsList
                .map((f) => `${f.id}: ${f.fmt.src}`)
                .join(", ")}): ${variantIdent}${generics} {`
              $ns.ad1`return { ${variantNameField}: [${formatTsList
                .map((f) => f.id)
                .join(", ")}] };`
              $ns.add`}`
              // create content
              $ns.lines.push(...variantDocs)
              $ns.add`${variantIdent}.content = ${generics}(${formatTsList
                .map((f) => `${f.id}: ${f.fmt.src}`)
                .join(", ")}) => [${formatTsList.map((f) => f.id).join(", ")}];`
              // match
              $nsMatchToObj.lines.push(...variantDocs)
              $nsMatchToObj.add`${variantIdent}(inner: ${innerTypeRef}): R,`
              $nsMatchIfObjs.add`if (${vnStr} in input) return to.${variantIdent}(input[${vnStr}]);`
            },
            Unit() {
              // type
              $ns.lines.push(...variantDocs)
              $ns.add`export type ${variantIdent}${generics} = ${namedStr(
                variant,
              )}`
              // create
              $ns.lines.push(...variantDocs)
              $ns.add`export function ${variantIdent}${generics}(): ${variantIdent}${generics} {`
              $ns.ad1`return ${namedStr(variant)};`
              $ns.add`}`
              // match
              $nsMatchToObj.lines.push(...variantDocs)
              $nsMatchToObj.add`${variantIdent}(): R,`
              $nsMatchIfStrs.add`if (input === ${namedStr(
                variant,
              )}) return to.${variantIdent}();`
            },
            Struct({ fields }) {
              const innerTypeRef = `${variantIdent}[${namedStr(variant)}]`
              // type
              $ns.lines.push(...variantDocs)
              $ns.add`export type ${variantIdent}${generics} = {`
              $ns.scope(($$) => {
                $$.lines.push(...variantDocs)
                $$.add`${variantNameField}: {`
                typeFieldsFinish$($$, fields)
              })
              $ns.add`};`
              // create
              $ns.lines.push(...variantDocs)
              $ns.add`export function ${variantIdent}${generics}(value: ${innerTypeRef}): ${variantIdent} {`
              $ns.ad1`return { ${variantNameField}: value }`
              $ns.add`}`
              // create content
              $ns.lines.push(...variantDocs)
              $ns.add`${variantIdent}.content = ${generics}(value: ${innerTypeRef}) => value;`
              // match
              $nsMatchToObj.lines.push(...variantDocs)
              $nsMatchToObj.add`${variantIdent}(inner: ${innerTypeRef}): R,`
              $nsMatchIfObjs.add`if (${namedStr(
                variant,
              )} in input) return to.${variantIdent}(input[${namedStr(
                variant,
              )}]);`
            },
          })
        }

        $decl.lines.push(...typeCode.lines)
      },
      NewTypeStruct(format) {
        const structIdent = ident(decl.id)
        const newTypeFormat = createFormat(format)
        // type
        $decl.lines.push(...docs)
        if (decl.serde_flags?.transparent) {
          $decl.add`export type ${structIdent}${generics} = ${newTypeFormat.src}`
        } else {
          $decl.add`export type ${structIdent}${generics} = [${newTypeFormat.src}]`
        }
        // create
        $decl.lines.push(...docs)
        $decl.add`export function ${structIdent}${generics}(inner${
          newTypeFormat.optional && "?"
        }: ${newTypeFormat.src}): ${structIdent}${generics} {`
        if (decl.serde_flags?.transparent) {
          $decl.ad1`return inner;`
        } else {
          $decl.ad1`return [inner];`
        }
        $decl.add`}`
      },
      TupleStruct(formats) {
        const formatTsList = tupleFormats(formats)
        const structIdent = ident(decl.id)
        // type
        $decl.lines.push(...docs)
        $decl.add`export type ${structIdent}${generics} = [${formatTsList
          .map((f) => f.fmt.src)
          .join(", ")}]`
        // create
        $decl.lines.push(...docs)
        $decl.add`export function ${structIdent}${generics}(${formatTsList
          .map((f) => `${f.id}: ${f.fmt.src}`)
          .join(", ")}): ${structIdent}${generics} {`
        $decl.ad1`return [${formatTsList.map((f) => f.id).join(", ")}];`
        $decl.add`}`
      },
      UnitStruct() {
        const structIdent = ident(decl.id)
        // type
        $decl.lines.push(...docs)
        $decl.add`export interface ${structIdent}${generics} {} /* hmm unit struct? */`
        // create
        $decl.lines.push(...docs)
        $decl.add`export function ${structIdent}${generics}(): ${structIdent} {`
        $decl.ad1`return {};`
        $decl.add`}`
      },
    })

    generated.lines.push(...$decl.lines)
  }

  // serde_json value is unknown / any
  generated.lines.unshift("/** serde_json::Value */\ntype Value = unknown;")

  const sharedOutputFile = args.fileName ?? "_shared.gen.ts"

  return {
    errors: [],
    files: [
      {
        path: sharedOutputFile,
        source: generated.toString(),
      },
      ...Array.from(outputFiles.entries()).map(([name, { importIdent }]) => ({
        path: name,
        source: new Code([
          `export { ${importIdent} } from "./${sharedOutputFile}";`,
        ]).toString(),
      })),
    ],
    warnings: [],
  }
}

const checkEnum = gen.EnumRepresentation.apply({
  External() {},
  Tagged() {
    throw new Error("Tagged representation not handled by generator")
  },
  Untagged() {
    throw new Error("Untagged representation not handled by generator")
  },
})
function splitByFlattened<T extends gen.Attrs>(items: T[]) {
  const flattened: T[] = []
  const fields: T[] = []
  for (const item of items) {
    if (item.serde_flags?.flatten) {
      flattened.push(item)
    } else {
      fields.push(item)
    }
  }
  return { flattened, fields }
}

/** Only for the types */
function typeFieldsFinish$(
  root: Code,
  fields: gen.NamedField[],
  finish: "}" | "};" = "};",
) {
  const $ = root.indented()
  const split = splitByFlattened(fields)
  for (const field of split.fields) {
    const { src, optional } = createFormat(field.format)
    const isOptional =
      optional ||
      (field.serde_flags?.default && field.serde_attrs?.skip_serializing_if)
    $.addDocString(field)
    if (field.codegen_attrs?.ts_as) {
      $.add`${namedField(field)}${isOptional && "?"}: ${
        field.codegen_attrs.ts_as[0]
      };`
    } else {
      $.add`${namedField(field)}${isOptional && "?"}: ${src}${
        isOptional && " | null | undefined"
      };`
    }
  }

  if (split.flattened.length === 0) {
    root.lines.push(finish)
    return
  }
  root.add`} // flattened fields:`
  for (const flattened of split.flattened) {
    root.addDocString(flattened, `Flattened from \`.${flattened.id}\`.`)
    const format = createFormat(flattened.format)
    if (format.optional) {
      root.add`& Partial<${format.src}>`
    } else {
      root.add`& ${format.src}`
    }
  }
  root.lastLine += ";"
}

function tupleFormats(formats: gen.Format[]) {
  const aCp = "a".codePointAt(0)!
  return formats.map((f, idx) => ({
    fmt: createFormat(f),
    id: String.fromCodePoint(aCp + idx),
  }))
}

const num = () => ({ src: "number" })

const createFormat: (format: gen.Format) => {
  src: string
  optional?: boolean
} = gen.Format.apply({
  TypeName: (value) => {
    const generics = value.generics.length
      ? `<${value.generics.map((g) => createFormat(g).src).join(", ")}>`
      : ""
    return { src: `${ident(value.ident)}${generics}` }
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
    const inner = createFormat(format)
    if (inner.optional) return inner
    return {
      src: `${inner.src} | undefined | null`,
      optional: true,
    }
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
})

function ident(id: string): string {
  return id.replace(/[^a-zA-Z0-9\$\_]/g, "$").replace(/^(\d)/, "$$1")
}

function namedField(named: { id: string } & gen.Attrs): string {
  const nam = named.serde_attrs?.["rename"]?.[0] ?? named.id
  if (/^[\w$][\w\d$]*$/.test(nam)) return nam
  else return JSON.stringify(nam)
}

function namedStr(named: { id: string } & gen.Attrs): string {
  const nam = named.serde_attrs?.["rename"]?.[0] ?? named.id
  return JSON.stringify(nam)
}

console.log(
  JSON.stringify(convert(JSON.parse((jsonInput as string)!)), null, 2),
)
