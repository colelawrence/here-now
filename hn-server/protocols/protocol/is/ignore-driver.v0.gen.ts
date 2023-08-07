/** serde_json::Value */
type Value = unknown;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:9`](hn-server/src/data.rs)
 */
export type UsrString = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:9`](hn-server/src/data.rs)
 */
export function UsrString(inner: string): UsrString {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:13`](hn-server/src/data.rs)
 */
export type DevString = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:13`](hn-server/src/data.rs)
 */
export function DevString(inner: string): DevString {
  return inner;
}
/**
 * `#[codegen(as = "`${string}//${string}`", tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:17`](hn-server/src/data.rs)
 */
export type GlobalID = `${string}//${string}`;
/**
 * `#[codegen(as = "`${string}//${string}`", tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:17`](hn-server/src/data.rs)
 */
export function GlobalID(value: GlobalID): GlobalID {
  return value;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:20`](hn-server/src/data.rs)
 */
export type ChannelID = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:20`](hn-server/src/data.rs)
 */
export function ChannelID(inner: string): ChannelID {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:24`](hn-server/src/data.rs)
 */
export type Key = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:24`](hn-server/src/data.rs)
 */
export function Key(inner: string): Key {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:28`](hn-server/src/data.rs)
 */
export type KeyTarget = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:28`](hn-server/src/data.rs)
 */
export function KeyTarget(inner: string): KeyTarget {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:32`](hn-server/src/data.rs)
 */
export type LiveID = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:32`](hn-server/src/data.rs)
 */
export function LiveID(inner: string): LiveID {
  return inner;
}
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:39`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace Out {
  export type ApplyFns<R = void> = {
    // callbacks
    /** TODO: Is this replaceable by "OFFER" semantics? */
    DECLARE_SERVICE(inner: DECLARE_SERVICE["DECLARE_SERVICE"]): R,
  }
  /** Match helper for {@link Out} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: Out) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("DECLARE_SERVICE" in input) return to.DECLARE_SERVICE(input["DECLARE_SERVICE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected Out");
    };
  }
  /** Factory helper for {@link Out} */
  export function factory<R>(fn: (value: Out) => R): ApplyFns<R> {
    return {
      // factory
      DECLARE_SERVICE(value) {
        return fn(DECLARE_SERVICE(value));
      },
    };
  }
  /** Match helper for {@link Out} */
  export function match<R>(
    input: Out,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  /** TODO: Is this replaceable by "OFFER" semantics? */
  export type DECLARE_SERVICE = {
    /** TODO: Is this replaceable by "OFFER" semantics? */
    DECLARE_SERVICE: {
      title: UsrString;
      key: Key;
    };
  };
  /** TODO: Is this replaceable by "OFFER" semantics? */
  export function DECLARE_SERVICE(value: DECLARE_SERVICE["DECLARE_SERVICE"]): DECLARE_SERVICE {
    return { DECLARE_SERVICE: value }
  }
}
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:39`](hn-server/src/data.rs)
 */
export type Out =
  | Out.DECLARE_SERVICE
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:53`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace In {
  export type ApplyFns<R = void> = {
    // callbacks
    CREATE_SERVICE(inner: CREATE_SERVICE["CREATE_SERVICE"]): R,
  }
  /** Match helper for {@link In} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: In) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("CREATE_SERVICE" in input) return to.CREATE_SERVICE(input["CREATE_SERVICE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected In");
    };
  }
  /** Factory helper for {@link In} */
  export function factory<R>(fn: (value: In) => R): ApplyFns<R> {
    return {
      // factory
      CREATE_SERVICE(value) {
        return fn(CREATE_SERVICE(value));
      },
    };
  }
  /** Match helper for {@link In} */
  export function match<R>(
    input: In,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type CREATE_SERVICE = {
    CREATE_SERVICE: {
      service_key: KeyTarget;
      channel: ChannelID;
    };
  };
  export function CREATE_SERVICE(value: CREATE_SERVICE["CREATE_SERVICE"]): CREATE_SERVICE {
    return { CREATE_SERVICE: value }
  }
}
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:53`](hn-server/src/data.rs)
 */
export type In =
  | In.CREATE_SERVICE
