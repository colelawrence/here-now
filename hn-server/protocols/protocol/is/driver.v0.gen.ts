/** serde_json::Value */
type Value = unknown;
/** `#[codegen(as = "`${string}//${string}`", tags = "protocol-agent")]` */
export type GlobalID = `${string}//${string}`;
/** `#[codegen(as = "`${string}//${string}`", tags = "protocol-agent")]` */
export function GlobalID(value: GlobalID): GlobalID {
  return value;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-agent")]`
 */
export type ChannelID = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-agent")]`
 */
export function ChannelID(inner: string): ChannelID {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-agent")]`
 */
export type LocalKey = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-agent")]`
 */
export function LocalKey(inner: string): LocalKey {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-agent")]`
 */
export type LiveID = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-agent")]`
 */
export function LiveID(inner: string): LiveID {
  return inner;
}
/** `#[codegen(tags = "protocol-agent")]` */
// deno-lint-ignore no-namespace
export namespace Out {
  export type ApplyFns<R> = {
    // callbacks
    IDENTIFY(inner: IDENTIFY["IDENTIFY"]): R,
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
      if ("IDENTIFY" in input) return to.IDENTIFY(input["IDENTIFY"]);
      if ("DECLARE_SERVICE" in input) return to.DECLARE_SERVICE(input["DECLARE_SERVICE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected Out");
    }
  }
  /** Match helper for {@link Out} */
  export function match<R>(
    input: Out,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type IDENTIFY = {
    IDENTIFY: {
      key: LocalKey;
    };
  };
  export function IDENTIFY(value: IDENTIFY["IDENTIFY"]): IDENTIFY {
    return { IDENTIFY: value }
  }
  export type DECLARE_SERVICE = {
    DECLARE_SERVICE: {
      key: LocalKey;
      /** Protocols are kinda like traits in Rust */
      protocols: Array<GlobalID>;
    };
  };
  export function DECLARE_SERVICE(value: DECLARE_SERVICE["DECLARE_SERVICE"]): DECLARE_SERVICE {
    return { DECLARE_SERVICE: value }
  }
}
/** `#[codegen(tags = "protocol-agent")]` */
export type Out =
  | Out.IDENTIFY
  | Out.DECLARE_SERVICE
/** `#[codegen(tags = "protocol-agent")]` */
// deno-lint-ignore no-namespace
export namespace In {
  export type ApplyFns<R> = {
    // callbacks
    CREATE_SERVICE(inner: CREATE_SERVICE["CREATE_SERVICE"]): R,
    RESUME_SERVICE(inner: RESUME_SERVICE["RESUME_SERVICE"]): R,
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
      if ("RESUME_SERVICE" in input) return to.RESUME_SERVICE(input["RESUME_SERVICE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected In");
    }
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
      key: LocalKey;
      channel: ChannelID;
    };
  };
  export function CREATE_SERVICE(value: CREATE_SERVICE["CREATE_SERVICE"]): CREATE_SERVICE {
    return { CREATE_SERVICE: value }
  }
  export type RESUME_SERVICE = {
    RESUME_SERVICE: {
      key: LocalKey;
      channel: ChannelID;
      state: Value;
    };
  };
  export function RESUME_SERVICE(value: RESUME_SERVICE["RESUME_SERVICE"]): RESUME_SERVICE {
    return { RESUME_SERVICE: value }
  }
}
/** `#[codegen(tags = "protocol-agent")]` */
export type In =
  | In.CREATE_SERVICE
  | In.RESUME_SERVICE