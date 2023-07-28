/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
export type LocationID = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
export function LocationID(inner: string): LocationID {
  return inner;
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type Input = {
  declarations: Array<InputDeclaration>;
};
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function Input(inner: Input): Input {
  return inner;
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type InputDeclaration = {
  id: string;
  id_location: LocationID;
  container_kind: ContainerFormat;
} // flattened fields:
/**
 * `#[serde(flatten)]`
 *
 * Flattened from `.attrs`.
 */
& Attrs;
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function InputDeclaration(inner: InputDeclaration): InputDeclaration {
  return inner;
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type Output = {
  errors: Array<OutputMessage>;
  warnings: Array<OutputMessage>;
  files: Array<OutputFile>;
};
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function Output(inner: Output): Output {
  return inner;
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type OutputFile = {
  /** Example: `./some-dir/filename.txt` */
  path: string;
  /** Example: `"Hello world"` */
  source: string;
};
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function OutputFile(inner: OutputFile): OutputFile {
  return inner;
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type OutputMessage = {
  message: string;
  /** Labelled spans */
  labels: Array<[string, LocationID]>;
};
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function OutputMessage(inner: OutputMessage): OutputMessage {
  return inner;
}
/**
 * Serde-based serialization format for anonymous "value" types.
 * This is just the path respecting serde names into the container
 * It gets replaced by the knowledge
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
// deno-lint-ignore no-namespace
export namespace Format {
  export type ApplyFns<R> = {
    // callbacks
    Incomplete(inner: Incomplete["Incomplete"]): R,
    /** The name of a container. */
    TypeName(inner: TypeName["TypeName"]): R,
    Unit(): R,
    Bool(): R,
    I8(): R,
    I16(): R,
    I32(): R,
    I64(): R,
    I128(): R,
    ISIZE(): R,
    U8(): R,
    U16(): R,
    U32(): R,
    U64(): R,
    U128(): R,
    USIZE(): R,
    F32(): R,
    F64(): R,
    Char(): R,
    Str(): R,
    Bytes(): R,
    /** The format of `Option<T>`. */
    Option(inner: Option["Option"]): R;
    /** Never actually instantiated */
    Never(): R,
    /** A sequence, e.g. the format of `Vec<Foo>`. */
    Seq(inner: Seq["Seq"]): R;
    /** A map, e.g. the format of `BTreeMap<K, V>`. */
    Map(inner: Map["Map"]): R,
    /** A tuple, e.g. the format of `(Foo, Bar)`. */
    Tuple(inner: Tuple["Tuple"]): R;
    /**
     * Alias for `(Foo, ... Foo)`.
     * E.g. the format of `[Foo; N]`.
     */
    TupleArray(inner: TupleArray["TupleArray"]): R,
  }
  /** Match helper for {@link Format} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: Format) => R {
    return function _match(input): R {
      // if-else strings
      if (input === "Unit") return to.Unit();
      if (input === "Bool") return to.Bool();
      if (input === "I8") return to.I8();
      if (input === "I16") return to.I16();
      if (input === "I32") return to.I32();
      if (input === "I64") return to.I64();
      if (input === "I128") return to.I128();
      if (input === "ISIZE") return to.ISIZE();
      if (input === "U8") return to.U8();
      if (input === "U16") return to.U16();
      if (input === "U32") return to.U32();
      if (input === "U64") return to.U64();
      if (input === "U128") return to.U128();
      if (input === "USIZE") return to.USIZE();
      if (input === "F32") return to.F32();
      if (input === "F64") return to.F64();
      if (input === "Char") return to.Char();
      if (input === "Str") return to.Str();
      if (input === "Bytes") return to.Bytes();
      if (input === "Never") return to.Never();
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Incomplete" in input) return to.Incomplete(input["Incomplete"]);
      if ("TypeName" in input) return to.TypeName(input["TypeName"]);
      if ("Option" in input) return to.Option(input["Option"]);
      if ("Seq" in input) return to.Seq(input["Seq"]);
      if ("Map" in input) return to.Map(input["Map"]);
      if ("Tuple" in input) return to.Tuple(input["Tuple"]);
      if ("TupleArray" in input) return to.TupleArray(input["TupleArray"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link Format} */
  export function match<R>(
    input: Format,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type Incomplete = {
    Incomplete: {
      debug: string;
    };
  }
  export function Incomplete(value: Incomplete["Incomplete"]): Incomplete {
    return { Incomplete: value }
  }
  /** The name of a container. */
  export type TypeName = {
    TypeName: {
      ident: string;
      generics: Array<Format>;
    };
  }
  /** The name of a container. */
  export function TypeName(value: TypeName["TypeName"]): TypeName {
    return { TypeName: value }
  }
  export type Unit = "Unit"
  export function Unit(): Unit {
    return "Unit";
  }
  export type Bool = "Bool"
  export function Bool(): Bool {
    return "Bool";
  }
  export type I8 = "I8"
  export function I8(): I8 {
    return "I8";
  }
  export type I16 = "I16"
  export function I16(): I16 {
    return "I16";
  }
  export type I32 = "I32"
  export function I32(): I32 {
    return "I32";
  }
  export type I64 = "I64"
  export function I64(): I64 {
    return "I64";
  }
  export type I128 = "I128"
  export function I128(): I128 {
    return "I128";
  }
  export type ISIZE = "ISIZE"
  export function ISIZE(): ISIZE {
    return "ISIZE";
  }
  export type U8 = "U8"
  export function U8(): U8 {
    return "U8";
  }
  export type U16 = "U16"
  export function U16(): U16 {
    return "U16";
  }
  export type U32 = "U32"
  export function U32(): U32 {
    return "U32";
  }
  export type U64 = "U64"
  export function U64(): U64 {
    return "U64";
  }
  export type U128 = "U128"
  export function U128(): U128 {
    return "U128";
  }
  export type USIZE = "USIZE"
  export function USIZE(): USIZE {
    return "USIZE";
  }
  export type F32 = "F32"
  export function F32(): F32 {
    return "F32";
  }
  export type F64 = "F64"
  export function F64(): F64 {
    return "F64";
  }
  export type Char = "Char"
  export function Char(): Char {
    return "Char";
  }
  export type Str = "Str"
  export function Str(): Str {
    return "Str";
  }
  export type Bytes = "Bytes"
  export function Bytes(): Bytes {
    return "Bytes";
  }
  /** The format of `Option<T>`. */
  export type Option = {
    /** The format of `Option<T>`. */
    Option: Format
  };
  /** The format of `Option<T>`. */
  export function Option(value: Format): Option {
    return { Option: value };
  }
  /** Never actually instantiated */
  export type Never = "Never"
  /** Never actually instantiated */
  export function Never(): Never {
    return "Never";
  }
  /** A sequence, e.g. the format of `Vec<Foo>`. */
  export type Seq = {
    /** A sequence, e.g. the format of `Vec<Foo>`. */
    Seq: Format
  };
  /** A sequence, e.g. the format of `Vec<Foo>`. */
  export function Seq(value: Format): Seq {
    return { Seq: value };
  }
  /** A map, e.g. the format of `BTreeMap<K, V>`. */
  export type Map = {
    Map: {
      key: Format;
      value: Format;
    };
  }
  /** A map, e.g. the format of `BTreeMap<K, V>`. */
  export function Map(value: Map["Map"]): Map {
    return { Map: value }
  }
  /** A tuple, e.g. the format of `(Foo, Bar)`. */
  export type Tuple = {
    /** A tuple, e.g. the format of `(Foo, Bar)`. */
    Tuple: Array<Format>
  };
  /** A tuple, e.g. the format of `(Foo, Bar)`. */
  export function Tuple(value: Array<Format>): Tuple {
    return { Tuple: value };
  }
  /**
   * Alias for `(Foo, ... Foo)`.
   * E.g. the format of `[Foo; N]`.
   */
  export type TupleArray = {
    TupleArray: {
      content: Format;
      size: number;
    };
  }
  /**
   * Alias for `(Foo, ... Foo)`.
   * E.g. the format of `[Foo; N]`.
   */
  export function TupleArray(value: TupleArray["TupleArray"]): TupleArray {
    return { TupleArray: value }
  }
}
/**
 * Serde-based serialization format for anonymous "value" types.
 * This is just the path respecting serde names into the container
 * It gets replaced by the knowledge
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
export type Format =
  | Format.Incomplete
  | Format.TypeName
  | Format.Unit
  | Format.Bool
  | Format.I8
  | Format.I16
  | Format.I32
  | Format.I64
  | Format.I128
  | Format.ISIZE
  | Format.U8
  | Format.U16
  | Format.U32
  | Format.U64
  | Format.U128
  | Format.USIZE
  | Format.F32
  | Format.F64
  | Format.Char
  | Format.Str
  | Format.Bytes
  | Format.Option
  | Format.Never
  | Format.Seq
  | Format.Map
  | Format.Tuple
  | Format.TupleArray
/**
 * Serde-based serialization format for named "container" types.
 * In Rust, those are enums and structs.
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
// deno-lint-ignore no-namespace
export namespace ContainerFormat {
  export type ApplyFns<R> = {
    // callbacks
    /** An empty struct, e.g. `struct A`. */
    UnitStruct(): R,
    /** A struct with a single unnamed parameter, e.g. `struct A(u16)` */
    NewTypeStruct(inner: NewTypeStruct["NewTypeStruct"]): R;
    /** A struct with several unnamed parameters, e.g. `struct A(u16, u32)` */
    TupleStruct(inner: TupleStruct["TupleStruct"]): R;
    /** A struct with named parameters, e.g. `struct A { a: Foo }`. */
    Struct(inner: Struct["Struct"]): R,
    /**
     * An enum, that is, an enumeration of variants.
     * Each variant has a unique name and index within the enum.
     */
    Enum(inner: Enum["Enum"]): R,
  }
  /** Match helper for {@link ContainerFormat} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: ContainerFormat) => R {
    return function _match(input): R {
      // if-else strings
      if (input === "UnitStruct") return to.UnitStruct();
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("NewTypeStruct" in input) return to.NewTypeStruct(input["NewTypeStruct"]);
      if ("TupleStruct" in input) return to.TupleStruct(input["TupleStruct"]);
      if ("Struct" in input) return to.Struct(input["Struct"]);
      if ("Enum" in input) return to.Enum(input["Enum"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link ContainerFormat} */
  export function match<R>(
    input: ContainerFormat,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  /** An empty struct, e.g. `struct A`. */
  export type UnitStruct = "UnitStruct"
  /** An empty struct, e.g. `struct A`. */
  export function UnitStruct(): UnitStruct {
    return "UnitStruct";
  }
  /** A struct with a single unnamed parameter, e.g. `struct A(u16)` */
  export type NewTypeStruct = {
    /** A struct with a single unnamed parameter, e.g. `struct A(u16)` */
    NewTypeStruct: Format
  };
  /** A struct with a single unnamed parameter, e.g. `struct A(u16)` */
  export function NewTypeStruct(value: Format): NewTypeStruct {
    return { NewTypeStruct: value };
  }
  /** A struct with several unnamed parameters, e.g. `struct A(u16, u32)` */
  export type TupleStruct = {
    /** A struct with several unnamed parameters, e.g. `struct A(u16, u32)` */
    TupleStruct: Array<Format>
  };
  /** A struct with several unnamed parameters, e.g. `struct A(u16, u32)` */
  export function TupleStruct(value: Array<Format>): TupleStruct {
    return { TupleStruct: value };
  }
  /** A struct with named parameters, e.g. `struct A { a: Foo }`. */
  export type Struct = {
    Struct: {
      fields: Array<NamedField>;
    };
  }
  /** A struct with named parameters, e.g. `struct A { a: Foo }`. */
  export function Struct(value: Struct["Struct"]): Struct {
    return { Struct: value }
  }
  /**
   * An enum, that is, an enumeration of variants.
   * Each variant has a unique name and index within the enum.
   */
  export type Enum = {
    Enum: {
      repr: EnumRepresentation;
      variants: Array<NamedVariant>;
    };
  }
  /**
   * An enum, that is, an enumeration of variants.
   * Each variant has a unique name and index within the enum.
   */
  export function Enum(value: Enum["Enum"]): Enum {
    return { Enum: value }
  }
}
/**
 * Serde-based serialization format for named "container" types.
 * In Rust, those are enums and structs.
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
export type ContainerFormat =
  | ContainerFormat.UnitStruct
  | ContainerFormat.NewTypeStruct
  | ContainerFormat.TupleStruct
  | ContainerFormat.Struct
  | ContainerFormat.Enum
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type NamedVariant = {
  id: string;
  id_location: LocationID;
  variant_format: VariantFormat;
} // flattened fields:
/**
 * `#[serde(flatten)]`
 *
 * Flattened from `.attrs`.
 */
& Attrs;
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function NamedVariant(inner: NamedVariant): NamedVariant {
  return inner;
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type NamedField = {
  id: string;
  id_location: LocationID;
  format: Format;
} // flattened fields:
/**
 * `#[serde(flatten)]`
 *
 * Flattened from `.attrs`.
 */
& Attrs;
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function NamedField(inner: NamedField): NamedField {
  return inner;
}
/**
 * Description of a variant in an enum.
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
// deno-lint-ignore no-namespace
export namespace VariantFormat {
  export type ApplyFns<R> = {
    // callbacks
    /** A variant without parameters, e.g. `A` in `enum X { A }` */
    Unit(): R,
    /** A variant with a single unnamed parameter, e.g. `A` in `enum X { A(u16) }` */
    NewType(inner: NewType["NewType"]): R;
    /** A struct with several unnamed parameters, e.g. `A` in `enum X { A(u16, u32) }` */
    Tuple(inner: Tuple["Tuple"]): R;
    /** A struct with named parameters, e.g. `A` in `enum X { A { a: Foo } }` */
    Struct(inner: Struct["Struct"]): R,
  }
  /** Match helper for {@link VariantFormat} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: VariantFormat) => R {
    return function _match(input): R {
      // if-else strings
      if (input === "Unit") return to.Unit();
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("NewType" in input) return to.NewType(input["NewType"]);
      if ("Tuple" in input) return to.Tuple(input["Tuple"]);
      if ("Struct" in input) return to.Struct(input["Struct"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link VariantFormat} */
  export function match<R>(
    input: VariantFormat,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  /** A variant without parameters, e.g. `A` in `enum X { A }` */
  export type Unit = "Unit"
  /** A variant without parameters, e.g. `A` in `enum X { A }` */
  export function Unit(): Unit {
    return "Unit";
  }
  /** A variant with a single unnamed parameter, e.g. `A` in `enum X { A(u16) }` */
  export type NewType = {
    /** A variant with a single unnamed parameter, e.g. `A` in `enum X { A(u16) }` */
    NewType: Format
  };
  /** A variant with a single unnamed parameter, e.g. `A` in `enum X { A(u16) }` */
  export function NewType(value: Format): NewType {
    return { NewType: value };
  }
  /** A struct with several unnamed parameters, e.g. `A` in `enum X { A(u16, u32) }` */
  export type Tuple = {
    /** A struct with several unnamed parameters, e.g. `A` in `enum X { A(u16, u32) }` */
    Tuple: Array<Format>
  };
  /** A struct with several unnamed parameters, e.g. `A` in `enum X { A(u16, u32) }` */
  export function Tuple(value: Array<Format>): Tuple {
    return { Tuple: value };
  }
  /** A struct with named parameters, e.g. `A` in `enum X { A { a: Foo } }` */
  export type Struct = {
    Struct: {
      fields: Array<NamedField>;
    };
  }
  /** A struct with named parameters, e.g. `A` in `enum X { A { a: Foo } }` */
  export function Struct(value: Struct["Struct"]): Struct {
    return { Struct: value }
  }
}
/**
 * Description of a variant in an enum.
 *
 * `#[codegen(tags = "derive-codegen-internal")]`
 */
export type VariantFormat =
  | VariantFormat.Unit
  | VariantFormat.NewType
  | VariantFormat.Tuple
  | VariantFormat.Struct
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type Attrs = {
  /**
   * Documentation comments like this one.
   * Future idea: Pass in tokens with links to other types.
   */
  rust_docs?: string | undefined | null | null | undefined;
  /**
   * Only specified for enums and structs
   * Future: Consider whether we should monomorphize on the codegen side...
   *
   * `#[serde(default, skip_serializing_if = "Vec::is_empty")]`
   */
  rust_generics?: Array<[string, LocationID]> | null | undefined;
  /**
   * e.g. `#[serde(rename = "newName")]`, your generator will need to describe what it supports
   *
   * `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]`
   */
  serde_attrs?: Record<string, [string, LocationID]> | null | undefined;
  /**
   * e.g. `#[serde(transparent)]`, your generator will need to describe what it supports
   *
   * `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]`
   */
  serde_flags?: Record<string, LocationID> | null | undefined;
  /**
   * e.g. `#[codegen(ts_as = "Date")]` - these are customizable for your generator's use cases.
   *
   * `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]`
   */
  codegen_attrs?: Record<string, [string, LocationID]> | null | undefined;
  /**
   * e.g. `#[codegen(hidden)]` - these are customizable for your generator's use cases.
   *
   * `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]`
   */
  codegen_flags?: Record<string, LocationID> | null | undefined;
};
/** `#[codegen(tags = "derive-codegen-internal")]` */
export function Attrs(inner: Attrs): Attrs {
  return inner;
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
// deno-lint-ignore no-namespace
export namespace EnumRepresentation {
  export type ApplyFns<R> = {
    // callbacks
    /**
     * The default
     * e.g `{ User: { id: 1200, name: "Smithy" } }`
     */
    External(): R,
    /** e.g `{ id: 1200, name: "Smithy" }` */
    Untagged(): R,
    /**
     * e.g `{ type: "User", id: 1200, name: "Smithy" }`
     * e.g `{ type: "User", content: { id: 1200, name: "Smithy" } }`
     */
    Tagged(inner: Tagged["Tagged"]): R,
  }
  /** Match helper for {@link EnumRepresentation} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: EnumRepresentation) => R {
    return function _match(input): R {
      // if-else strings
      if (input === "External") return to.External();
      if (input === "Untagged") return to.Untagged();
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Tagged" in input) return to.Tagged(input["Tagged"]);
      const _exhaust: never = input;
      return _exhaust;
    }
  }
  /** Match helper for {@link EnumRepresentation} */
  export function match<R>(
    input: EnumRepresentation,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  /**
   * The default
   * e.g `{ User: { id: 1200, name: "Smithy" } }`
   */
  export type External = "External"
  /**
   * The default
   * e.g `{ User: { id: 1200, name: "Smithy" } }`
   */
  export function External(): External {
    return "External";
  }
  /** e.g `{ id: 1200, name: "Smithy" }` */
  export type Untagged = "Untagged"
  /** e.g `{ id: 1200, name: "Smithy" }` */
  export function Untagged(): Untagged {
    return "Untagged";
  }
  /**
   * e.g `{ type: "User", id: 1200, name: "Smithy" }`
   * e.g `{ type: "User", content: { id: 1200, name: "Smithy" } }`
   */
  export type Tagged = {
    Tagged: {
      tag: string;
      tag_location: LocationID;
      content?: string | undefined | null | null | undefined;
      content_location?: LocationID | undefined | null | null | undefined;
    };
  }
  /**
   * e.g `{ type: "User", id: 1200, name: "Smithy" }`
   * e.g `{ type: "User", content: { id: 1200, name: "Smithy" } }`
   */
  export function Tagged(value: Tagged["Tagged"]): Tagged {
    return { Tagged: value }
  }
}
/** `#[codegen(tags = "derive-codegen-internal")]` */
export type EnumRepresentation =
  | EnumRepresentation.External
  | EnumRepresentation.Untagged
  | EnumRepresentation.Tagged