/** serde_json::Value */
type Value = unknown;
/** [Source `hn-server/src/data.rs:72`](hn-server/src/data.rs) */
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
  TEXT.content = (value: TEXT["TEXT"]) => value;
  export type CHOICE = {
    CHOICE: {
      choice_key?: KeyTarget | undefined | null | null | undefined;
    };
  };
  export function CHOICE(value: CHOICE["CHOICE"]): CHOICE {
    return { CHOICE: value }
  }
  CHOICE.content = (value: CHOICE["CHOICE"]) => value;
}
/** [Source `hn-server/src/data.rs:72`](hn-server/src/data.rs) */
export type InputValueType =
  | InputValueType.TEXT
  | InputValueType.CHOICE
/** [Source `hn-server/src/data.rs:78`](hn-server/src/data.rs) */
export type InputValue = {
  input_key: KeyTarget;
  "r#type": InputValueType;
  /** Context for this value? */
  reason: DevString;
};
/** [Source `hn-server/src/data.rs:78`](hn-server/src/data.rs) */
export function InputValue(inner: InputValue): InputValue {
  return inner;
}
/** [Source `hn-server/src/data.rs:104`](hn-server/src/data.rs) */
// deno-lint-ignore no-namespace
export namespace In {
  export type ApplyFns<R = void> = {
    // callbacks
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
      if ("ASK" in input) return to.ASK(input["ASK"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected In");
    };
  }
  /** Factory helper for {@link In} */
  export function factory<R>(fn: (value: In) => R): ApplyFns<R> {
    return {
      // factory
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
  ASK.content = (value: ASK["ASK"]) => value;
}
/** [Source `hn-server/src/data.rs:104`](hn-server/src/data.rs) */
export type In =
  | In.ASK
/** [Source `hn-server/src/data.rs:123`](hn-server/src/data.rs) */
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
  IDENTIFY.content = (value: IDENTIFY["IDENTIFY"]) => value;
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
  UI.content = (value: UI["UI"]) => value;
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
  /** Alternative of DECLARE_SERVICE? */
  OFFER.content = (value: OFFER["OFFER"]) => value;
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
  RAISE.content = (value: RAISE["RAISE"]) => value;
  export type RESOLVE = {
    RESOLVE: {
      raise_key: KeyTarget;
      reason: DevString;
    };
  };
  export function RESOLVE(value: RESOLVE["RESOLVE"]): RESOLVE {
    return { RESOLVE: value }
  }
  RESOLVE.content = (value: RESOLVE["RESOLVE"]) => value;
}
/** [Source `hn-server/src/data.rs:123`](hn-server/src/data.rs) */
export type Out =
  | Out.IDENTIFY
  | Out.UI
  | Out.OFFER
  | Out.RAISE
  | Out.RESOLVE
/** [Source `hn-server/src/data.rs:168`](hn-server/src/data.rs) */
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
  INPUT.content = (value: UIInput) => value;
  export type CONTENT = {
    CONTENT: UIContent
  };
  export function CONTENT(value: UIContent): CONTENT {
    return { CONTENT: value };
  }
  CONTENT.content = (value: UIContent) => value;
  export type WARNING = {
    WARNING: {
      summarized?: UIContent | undefined | null | null | undefined;
      content: Array<UIContent>;
    };
  };
  export function WARNING(value: WARNING["WARNING"]): WARNING {
    return { WARNING: value }
  }
  WARNING.content = (value: WARNING["WARNING"]) => value;
}
/** [Source `hn-server/src/data.rs:168`](hn-server/src/data.rs) */
export type UIItem =
  | UIItem.INPUT
  | UIItem.CONTENT
  | UIItem.WARNING
/** [Source `hn-server/src/data.rs:178`](hn-server/src/data.rs) */
export type UIInput = {
  key: Key;
  label: UsrString;
  type: UIInputType;
};
/** [Source `hn-server/src/data.rs:178`](hn-server/src/data.rs) */
export function UIInput(inner: UIInput): UIInput {
  return inner;
}
/** [Source `hn-server/src/data.rs:186`](hn-server/src/data.rs) */
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
  HEADING.content = (value: HEADING["HEADING"]) => value;
  export type PARAGRAPH = {
    PARAGRAPH: {
      content: UsrString;
    };
  };
  export function PARAGRAPH(value: PARAGRAPH["PARAGRAPH"]): PARAGRAPH {
    return { PARAGRAPH: value }
  }
  PARAGRAPH.content = (value: PARAGRAPH["PARAGRAPH"]) => value;
}
/** [Source `hn-server/src/data.rs:186`](hn-server/src/data.rs) */
export type UIContent =
  | UIContent.HEADING
  | UIContent.PARAGRAPH
/** [Source `hn-server/src/data.rs:197`](hn-server/src/data.rs) */
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
      /** e.g. `"+1 (913) 555-1234"`, `"+1 (917) 555-4000"`, `"+1 (816) 555-9000"` */
      examples?: Array<string> | null | undefined;
      /** e.g. `"Full phone number with country code"` */
      format_description?: string | undefined | null | null | undefined;
    };
  };
  export function TEXT(value: TEXT["TEXT"]): TEXT {
    return { TEXT: value }
  }
  TEXT.content = (value: TEXT["TEXT"]) => value;
  export type CHOICE = {
    CHOICE: {
      choices: Array<UIInputChoice>;
    };
  };
  export function CHOICE(value: CHOICE["CHOICE"]): CHOICE {
    return { CHOICE: value }
  }
  CHOICE.content = (value: CHOICE["CHOICE"]) => value;
}
/** [Source `hn-server/src/data.rs:197`](hn-server/src/data.rs) */
export type UIInputType =
  | UIInputType.TEXT
  | UIInputType.CHOICE
/** [Source `hn-server/src/data.rs:218`](hn-server/src/data.rs) */
export type UIInputChoice = {
  key: Key;
  /** Label for this choice such as `"On"` or `"Off"`. */
  label: UsrString;
  /**
   * Optionally supply additional inputs
   * that can be set for when this choice is selected.
   * This enables a configuration approach similar to enums in Rust.
   */
  inputs?: Array<UIInput> | null | undefined;
};
/** [Source `hn-server/src/data.rs:218`](hn-server/src/data.rs) */
export function UIInputChoice(inner: UIInputChoice): UIInputChoice {
  return inner;
}
/** [Source `hn-server/src/data.rs:8`](hn-server/src/data.rs) */
export type UsrString = string
/** [Source `hn-server/src/data.rs:8`](hn-server/src/data.rs) */
export function UsrString(inner: string): UsrString {
  return inner;
}
/** [Source `hn-server/src/data.rs:12`](hn-server/src/data.rs) */
export type DevString = string
/** [Source `hn-server/src/data.rs:12`](hn-server/src/data.rs) */
export function DevString(inner: string): DevString {
  return inner;
}
/** [Source `hn-server/src/data.rs:16`](hn-server/src/data.rs) */
export type GlobalID = `${string}//${string}`;
/** [Source `hn-server/src/data.rs:16`](hn-server/src/data.rs) */
export function GlobalID(value: GlobalID): GlobalID {
  return value;
}
/** [Source `hn-server/src/data.rs:19`](hn-server/src/data.rs) */
export type ChannelID = string
/** [Source `hn-server/src/data.rs:19`](hn-server/src/data.rs) */
export function ChannelID(inner: string): ChannelID {
  return inner;
}
/** [Source `hn-server/src/data.rs:23`](hn-server/src/data.rs) */
export type Key = string
/** [Source `hn-server/src/data.rs:23`](hn-server/src/data.rs) */
export function Key(inner: string): Key {
  return inner;
}
/** [Source `hn-server/src/data.rs:27`](hn-server/src/data.rs) */
export type KeyTarget = string
/** [Source `hn-server/src/data.rs:27`](hn-server/src/data.rs) */
export function KeyTarget(inner: string): KeyTarget {
  return inner;
}
/** [Source `hn-server/src/data.rs:31`](hn-server/src/data.rs) */
export type LiveID = string
/** [Source `hn-server/src/data.rs:31`](hn-server/src/data.rs) */
export function LiveID(inner: string): LiveID {
  return inner;
}