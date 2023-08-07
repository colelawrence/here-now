/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:9`](hn-server/src/data.rs)
 */
export type UsrString = string;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:9`](hn-server/src/data.rs)
 */
export declare function UsrString(inner: string): UsrString;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:13`](hn-server/src/data.rs)
 */
export type DevString = string;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:13`](hn-server/src/data.rs)
 */
export declare function DevString(inner: string): DevString;
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
export declare function GlobalID(value: GlobalID): GlobalID;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:20`](hn-server/src/data.rs)
 */
export type ChannelID = string;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:20`](hn-server/src/data.rs)
 */
export declare function ChannelID(inner: string): ChannelID;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:24`](hn-server/src/data.rs)
 */
export type Key = string;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:24`](hn-server/src/data.rs)
 */
export declare function Key(inner: string): Key;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:28`](hn-server/src/data.rs)
 */
export type KeyTarget = string;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:28`](hn-server/src/data.rs)
 */
export declare function KeyTarget(inner: string): KeyTarget;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:32`](hn-server/src/data.rs)
 */
export type LiveID = string;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:32`](hn-server/src/data.rs)
 */
export declare function LiveID(inner: string): LiveID;
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:39`](hn-server/src/data.rs)
 */
export declare namespace Out {
    type ApplyFns<R = void> = {
        /** TODO: Is this replaceable by "OFFER" semantics? */
        DECLARE_SERVICE(inner: DECLARE_SERVICE["DECLARE_SERVICE"]): R;
    };
    /** Match helper for {@link Out} */
    function apply<R>(to: ApplyFns<R>): (input: Out) => R;
    /** Factory helper for {@link Out} */
    function factory<R>(fn: (value: Out) => R): ApplyFns<R>;
    /** Match helper for {@link Out} */
    function match<R>(input: Out, to: ApplyFns<R>): R;
    /** TODO: Is this replaceable by "OFFER" semantics? */
    type DECLARE_SERVICE = {
        /** TODO: Is this replaceable by "OFFER" semantics? */
        DECLARE_SERVICE: {
            // items: [..., "DECLARE_SERVICE", "DECLARE_SERVICE", "title"],
            // ln: 129, col: []
            title: UsrString;
            key: Key;
        };
    };
    /** TODO: Is this replaceable by "OFFER" semantics? */
    function DECLARE_SERVICE(value: DECLARE_SERVICE["DECLARE_SERVICE"]): DECLARE_SERVICE;
}
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:39`](hn-server/src/data.rs)
 */
export type Out = Out.DECLARE_SERVICE;
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:53`](hn-server/src/data.rs)
 */
export declare namespace In {
    type ApplyFns<R = void> = {
        CREATE_SERVICE(inner: CREATE_SERVICE["CREATE_SERVICE"]): R;
    };
    /** Match helper for {@link In} */
    function apply<R>(to: ApplyFns<R>): (input: In) => R;
    /** Factory helper for {@link In} */
    function factory<R>(fn: (value: In) => R): ApplyFns<R>;
    /** Match helper for {@link In} */
    function match<R>(input: In, to: ApplyFns<R>): R;
    type CREATE_SERVICE = {
        CREATE_SERVICE: {
            service_key: KeyTarget;
            channel: ChannelID;
        };
    };
    function CREATE_SERVICE(value: CREATE_SERVICE["CREATE_SERVICE"]): CREATE_SERVICE;
}
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:53`](hn-server/src/data.rs)
 */
export type In = In.CREATE_SERVICE;
//# sourceMappingURL=driver.v0.gen.d.ts.map