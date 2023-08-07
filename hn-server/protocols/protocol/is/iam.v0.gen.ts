/** serde_json::Value */
type Value = unknown;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:8`](hn-server/src/data.rs)
 */
export type UsrString = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:8`](hn-server/src/data.rs)
 */
export function UsrString(inner: string): UsrString {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:12`](hn-server/src/data.rs)
 */
export type DevString = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:12`](hn-server/src/data.rs)
 */
export function DevString(inner: string): DevString {
  return inner;
}
/**
 * `#[codegen(as = "`${string}//${string}`", tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:16`](hn-server/src/data.rs)
 */
export type GlobalID = `${string}//${string}`;
/**
 * `#[codegen(as = "`${string}//${string}`", tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:16`](hn-server/src/data.rs)
 */
export function GlobalID(value: GlobalID): GlobalID {
  return value;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:19`](hn-server/src/data.rs)
 */
export type ChannelID = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:19`](hn-server/src/data.rs)
 */
export function ChannelID(inner: string): ChannelID {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:23`](hn-server/src/data.rs)
 */
export type Key = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:23`](hn-server/src/data.rs)
 */
export function Key(inner: string): Key {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:27`](hn-server/src/data.rs)
 */
export type KeyTarget = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:27`](hn-server/src/data.rs)
 */
export function KeyTarget(inner: string): KeyTarget {
  return inner;
}
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:31`](hn-server/src/data.rs)
 */
export type LiveID = string
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:31`](hn-server/src/data.rs)
 */
export function LiveID(inner: string): LiveID {
  return inner;
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:72`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace InputValueType {
  export type ApplyFns<R = void> = {
    // callbacks
    TEXT(inner: TEXT["TEXT"]): R,
    CHOICE(inner: CHOICE["CHOICE"]): R,
  }
  /** Match helper for {@link InputValueType} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: InputValueType) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("TEXT" in input) return to.TEXT(input["TEXT"]);
      if ("CHOICE" in input) return to.CHOICE(input["CHOICE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected InputValueType");
    };
  }
  /** Factory helper for {@link InputValueType} */
  export function factory<R>(fn: (value: InputValueType) => R): ApplyFns<R> {
    return {
      // factory
      TEXT(value) {
        return fn(TEXT(value));
      },
      CHOICE(value) {
        return fn(CHOICE(value));
      },
    };
  }
  /** Match helper for {@link InputValueType} */
  export function match<R>(
    input: InputValueType,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type TEXT = {
    TEXT: {
      text?: string | undefined | null | null | undefined;
    };
  };
  export function TEXT(value: TEXT["TEXT"]): TEXT {
    return { TEXT: value }
  }
  export type CHOICE = {
    CHOICE: {
      choice_key?: KeyTarget | undefined | null | null | undefined;
    };
  };
  export function CHOICE(value: CHOICE["CHOICE"]): CHOICE {
    return { CHOICE: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:72`](hn-server/src/data.rs)
 */
export type InputValueType =
  | InputValueType.TEXT
  | InputValueType.CHOICE
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:78`](hn-server/src/data.rs)
 */
export type InputValue = {
  input_key: KeyTarget;
  "r#type": InputValueType;
  /** Context for this value? */
  reason: DevString;
};
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:78`](hn-server/src/data.rs)
 */
export function InputValue(inner: InputValue): InputValue {
  return inner;
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:86`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace SetInputType {
  export type ApplyFns<R = void> = {
    // callbacks
    TEXT(inner: TEXT["TEXT"]): R,
    CHOICE(inner: CHOICE["CHOICE"]): R,
  }
  /** Match helper for {@link SetInputType} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: SetInputType) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("TEXT" in input) return to.TEXT(input["TEXT"]);
      if ("CHOICE" in input) return to.CHOICE(input["CHOICE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected SetInputType");
    };
  }
  /** Factory helper for {@link SetInputType} */
  export function factory<R>(fn: (value: SetInputType) => R): ApplyFns<R> {
    return {
      // factory
      TEXT(value) {
        return fn(TEXT(value));
      },
      CHOICE(value) {
        return fn(CHOICE(value));
      },
    };
  }
  /** Match helper for {@link SetInputType} */
  export function match<R>(
    input: SetInputType,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type TEXT = {
    TEXT: {
      to_text?: string | undefined | null | null | undefined;
    };
  };
  export function TEXT(value: TEXT["TEXT"]): TEXT {
    return { TEXT: value }
  }
  export type CHOICE = {
    CHOICE: {
      to_choice_key?: KeyTarget | undefined | null | null | undefined;
    };
  };
  export function CHOICE(value: CHOICE["CHOICE"]): CHOICE {
    return { CHOICE: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:86`](hn-server/src/data.rs)
 */
export type SetInputType =
  | SetInputType.TEXT
  | SetInputType.CHOICE
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:92`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace InteractionType {
  export type ApplyFns<R = void> = {
    // callbacks
    SET_INPUT(inner: SET_INPUT["SET_INPUT"]): R,
  }
  /** Match helper for {@link InteractionType} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: InteractionType) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("SET_INPUT" in input) return to.SET_INPUT(input["SET_INPUT"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected InteractionType");
    };
  }
  /** Factory helper for {@link InteractionType} */
  export function factory<R>(fn: (value: InteractionType) => R): ApplyFns<R> {
    return {
      // factory
      SET_INPUT(value) {
        return fn(SET_INPUT(value));
      },
    };
  }
  /** Match helper for {@link InteractionType} */
  export function match<R>(
    input: InteractionType,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type SET_INPUT = {
    SET_INPUT: {
      /** [UIInput] key. */
      input_key: KeyTarget;
      /** `#[serde(rename = "type")]` */
      type: SetInputType;
    };
  };
  export function SET_INPUT(value: SET_INPUT["SET_INPUT"]): SET_INPUT {
    return { SET_INPUT: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:92`](hn-server/src/data.rs)
 */
export type InteractionType =
  | InteractionType.SET_INPUT
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:102`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace In {
  export type ApplyFns<R = void> = {
    // callbacks
    /** should "interact" be just "ASK"? */
    INTERACT(inner: INTERACT["INTERACT"]): R,
    ASK(inner: ASK["ASK"]): R,
  }
  /** Match helper for {@link In} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: In) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("INTERACT" in input) return to.INTERACT(input["INTERACT"]);
      if ("ASK" in input) return to.ASK(input["ASK"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected In");
    };
  }
  /** Factory helper for {@link In} */
  export function factory<R>(fn: (value: In) => R): ApplyFns<R> {
    return {
      // factory
      INTERACT(value) {
        return fn(INTERACT(value));
      },
      ASK(value) {
        return fn(ASK(value));
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
  /** should "interact" be just "ASK"? */
  export type INTERACT = {
    /** should "interact" be just "ASK"? */
    INTERACT: {
      /** [Out::UI] key. */
      ui_key: KeyTarget;
      /** `#[serde(rename = "type")]` */
      type: InteractionType;
    };
  };
  /** should "interact" be just "ASK"? */
  export function INTERACT(value: INTERACT["INTERACT"]): INTERACT {
    return { INTERACT: value }
  }
  export type ASK = {
    ASK: {
      offer_key: KeyTarget;
      channel: ChannelID;
      given_params: Array<InputValueType>;
    };
  };
  export function ASK(value: ASK["ASK"]): ASK {
    return { ASK: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:102`](hn-server/src/data.rs)
 */
export type In =
  | In.INTERACT
  | In.ASK
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:120`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace Out {
  export type ApplyFns<R = void> = {
    // callbacks
    IDENTIFY(inner: IDENTIFY["IDENTIFY"]): R,
    UI(inner: UI["UI"]): R,
    /** Alternative of DECLARE_SERVICE? */
    OFFER(inner: OFFER["OFFER"]): R,
    RAISE(inner: RAISE["RAISE"]): R,
    RESOLVE(inner: RESOLVE["RESOLVE"]): R,
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
      if ("UI" in input) return to.UI(input["UI"]);
      if ("OFFER" in input) return to.OFFER(input["OFFER"]);
      if ("RAISE" in input) return to.RAISE(input["RAISE"]);
      if ("RESOLVE" in input) return to.RESOLVE(input["RESOLVE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected Out");
    };
  }
  /** Factory helper for {@link Out} */
  export function factory<R>(fn: (value: Out) => R): ApplyFns<R> {
    return {
      // factory
      IDENTIFY(value) {
        return fn(IDENTIFY(value));
      },
      UI(value) {
        return fn(UI(value));
      },
      OFFER(value) {
        return fn(OFFER(value));
      },
      RAISE(value) {
        return fn(RAISE(value));
      },
      RESOLVE(value) {
        return fn(RESOLVE(value));
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
  export type IDENTIFY = {
    IDENTIFY: {
      title: UsrString;
    };
  };
  export function IDENTIFY(value: IDENTIFY["IDENTIFY"]): IDENTIFY {
    return { IDENTIFY: value }
  }
  export type UI = {
    UI: {
      key: Key;
      title: UsrString;
      order: number;
      items: Array<UIItem>;
    };
  };
  export function UI(value: UI["UI"]): UI {
    return { UI: value }
  }
  /** Alternative of DECLARE_SERVICE? */
  export type OFFER = {
    /** Alternative of DECLARE_SERVICE? */
    OFFER: {
      key: Key;
      title: UsrString;
      known_params: Array<UIItem>;
    };
  };
  /** Alternative of DECLARE_SERVICE? */
  export function OFFER(value: OFFER["OFFER"]): OFFER {
    return { OFFER: value }
  }
  export type RAISE = {
    RAISE: {
      key: Key;
      ui_key: KeyTarget;
      related_input_keys: Array<KeyTarget>;
      summary: UsrString;
      /** The origin of this raise */
      reason: DevString;
    };
  };
  export function RAISE(value: RAISE["RAISE"]): RAISE {
    return { RAISE: value }
  }
  export type RESOLVE = {
    RESOLVE: {
      raise_key: KeyTarget;
      reason: DevString;
    };
  };
  export function RESOLVE(value: RESOLVE["RESOLVE"]): RESOLVE {
    return { RESOLVE: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:120`](hn-server/src/data.rs)
 */
export type Out =
  | Out.IDENTIFY
  | Out.UI
  | Out.OFFER
  | Out.RAISE
  | Out.RESOLVE
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:165`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace UIItem {
  export type ApplyFns<R = void> = {
    // callbacks
    INPUT(inner: INPUT["INPUT"]): R;
    CONTENT(inner: CONTENT["CONTENT"]): R;
    WARNING(inner: WARNING["WARNING"]): R,
  }
  /** Match helper for {@link UIItem} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: UIItem) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("INPUT" in input) return to.INPUT(input["INPUT"]);
      if ("CONTENT" in input) return to.CONTENT(input["CONTENT"]);
      if ("WARNING" in input) return to.WARNING(input["WARNING"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected UIItem");
    };
  }
  /** Factory helper for {@link UIItem} */
  export function factory<R>(fn: (value: UIItem) => R): ApplyFns<R> {
    return {
      // factory
      INPUT(value) {
        return fn(INPUT(value));
      },
      CONTENT(value) {
        return fn(CONTENT(value));
      },
      WARNING(value) {
        return fn(WARNING(value));
      },
    };
  }
  /** Match helper for {@link UIItem} */
  export function match<R>(
    input: UIItem,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type INPUT = {
    INPUT: UIInput
  };
  export function INPUT(value: UIInput): INPUT {
    return { INPUT: value };
  }
  export type CONTENT = {
    CONTENT: UIContent
  };
  export function CONTENT(value: UIContent): CONTENT {
    return { CONTENT: value };
  }
  export type WARNING = {
    WARNING: {
      summarized?: UIContent | undefined | null | null | undefined;
      content: Array<UIContent>;
    };
  };
  export function WARNING(value: WARNING["WARNING"]): WARNING {
    return { WARNING: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:165`](hn-server/src/data.rs)
 */
export type UIItem =
  | UIItem.INPUT
  | UIItem.CONTENT
  | UIItem.WARNING
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:175`](hn-server/src/data.rs)
 */
export type UIInput = {
  key: Key;
  label: UsrString;
  /** `#[serde(rename = "type")]` */
  type: UIInputType;
};
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:175`](hn-server/src/data.rs)
 */
export function UIInput(inner: UIInput): UIInput {
  return inner;
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:183`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace UIContent {
  export type ApplyFns<R = void> = {
    // callbacks
    HEADING(inner: HEADING["HEADING"]): R,
    PARAGRAPH(inner: PARAGRAPH["PARAGRAPH"]): R,
  }
  /** Match helper for {@link UIContent} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: UIContent) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("HEADING" in input) return to.HEADING(input["HEADING"]);
      if ("PARAGRAPH" in input) return to.PARAGRAPH(input["PARAGRAPH"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected UIContent");
    };
  }
  /** Factory helper for {@link UIContent} */
  export function factory<R>(fn: (value: UIContent) => R): ApplyFns<R> {
    return {
      // factory
      HEADING(value) {
        return fn(HEADING(value));
      },
      PARAGRAPH(value) {
        return fn(PARAGRAPH(value));
      },
    };
  }
  /** Match helper for {@link UIContent} */
  export function match<R>(
    input: UIContent,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type HEADING = {
    HEADING: {
      content: UsrString;
    };
  };
  export function HEADING(value: HEADING["HEADING"]): HEADING {
    return { HEADING: value }
  }
  export type PARAGRAPH = {
    PARAGRAPH: {
      content: UsrString;
    };
  };
  export function PARAGRAPH(value: PARAGRAPH["PARAGRAPH"]): PARAGRAPH {
    return { PARAGRAPH: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:183`](hn-server/src/data.rs)
 */
export type UIContent =
  | UIContent.HEADING
  | UIContent.PARAGRAPH
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:194`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
export namespace UIInputType {
  export type ApplyFns<R = void> = {
    // callbacks
    TEXT(inner: TEXT["TEXT"]): R,
    CHOICE(inner: CHOICE["CHOICE"]): R,
  }
  /** Match helper for {@link UIInputType} */
  export function apply<R>(
    to: ApplyFns<R>,
  ): (input: UIInputType) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("TEXT" in input) return to.TEXT(input["TEXT"]);
      if ("CHOICE" in input) return to.CHOICE(input["CHOICE"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected UIInputType");
    };
  }
  /** Factory helper for {@link UIInputType} */
  export function factory<R>(fn: (value: UIInputType) => R): ApplyFns<R> {
    return {
      // factory
      TEXT(value) {
        return fn(TEXT(value));
      },
      CHOICE(value) {
        return fn(CHOICE(value));
      },
    };
  }
  /** Match helper for {@link UIInputType} */
  export function match<R>(
    input: UIInputType,
    to: ApplyFns<R>,
  ): R {
    return apply(to)(input)
  }
  export type TEXT = {
    TEXT: {
      /**
       * e.g. `"+1 (913) 555-1234"`, `"+1 (917) 555-4000"`, `"+1 (816) 555-9000"`
       *
       * `#[serde(default, skip_serializing_if = "Vec::is_empty")]`
       */
      examples?: Array<string> | null | undefined;
      /**
       * e.g. `"Full phone number with country code"`
       *
       * `#[serde(default, skip_serializing_if = "Option::is_none")]`
       */
      format_description?: string | undefined | null | null | undefined;
    };
  };
  export function TEXT(value: TEXT["TEXT"]): TEXT {
    return { TEXT: value }
  }
  export type CHOICE = {
    CHOICE: {
      choices: Array<UIInputChoice>;
    };
  };
  export function CHOICE(value: CHOICE["CHOICE"]): CHOICE {
    return { CHOICE: value }
  }
}
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:194`](hn-server/src/data.rs)
 */
export type UIInputType =
  | UIInputType.TEXT
  | UIInputType.CHOICE
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:215`](hn-server/src/data.rs)
 */
export type UIInputChoice = {
  key: Key;
  /** Label for this choice such as `"On"` or `"Off"`. */
  label: UsrString;
  /**
   * Optionally supply additional inputs
   * that can be set for when this choice is selected.
   * This enables a configuration approach similar to enums in Rust.
   *
   * `#[serde(default, skip_serializing_if = "Vec::is_empty")]`
   */
  inputs?: Array<UIInput> | null | undefined;
};
/**
 * `#[codegen(tags = "protocol-iam")]`
 *
 * [Source `hn-server/src/data.rs:215`](hn-server/src/data.rs)
 */
export function UIInputChoice(inner: UIInputChoice): UIInputChoice {
  return inner;
}