"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.In = exports.Out = exports.LiveID = exports.KeyTarget = exports.Key = exports.ChannelID = exports.GlobalID = exports.DevString = exports.UsrString = void 0;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:9`](hn-server/src/data.rs)
 */
function UsrString(inner) {
    return inner;
}
exports.UsrString = UsrString;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:13`](hn-server/src/data.rs)
 */
function DevString(inner) {
    return inner;
}
exports.DevString = DevString;
/**
 * `#[codegen(as = "`${string}//${string}`", tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:17`](hn-server/src/data.rs)
 */
function GlobalID(value) {
    return value;
}
exports.GlobalID = GlobalID;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:20`](hn-server/src/data.rs)
 */
function ChannelID(inner) {
    return inner;
}
exports.ChannelID = ChannelID;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:24`](hn-server/src/data.rs)
 */
function Key(inner) {
    return inner;
}
exports.Key = Key;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:28`](hn-server/src/data.rs)
 */
function KeyTarget(inner) {
    return inner;
}
exports.KeyTarget = KeyTarget;
/**
 * `#[serde(transparent)]`
 *
 * `#[codegen(tags = "protocol-global")]`
 *
 * [Source `hn-server/src/data.rs:32`](hn-server/src/data.rs)
 */
function LiveID(inner) {
    return inner;
}
exports.LiveID = LiveID;
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:39`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
var Out;
(function (Out) {
    /** Match helper for {@link Out} */
    function apply(to) {
        return function _match(input) {
            // if-else strings
            // if-else objects
            if (typeof input !== "object" || input == null)
                throw new TypeError("Unexpected non-object for input");
            if ("DECLARE_SERVICE" in input)
                return to.DECLARE_SERVICE(input["DECLARE_SERVICE"]);
            var _exhaust = input;
            throw new TypeError("Unknown object when expected Out");
        };
    }
    Out.apply = apply;
    /** Factory helper for {@link Out} */
    function factory(fn) {
        return {
            // factory
            DECLARE_SERVICE: function (value) {
                return fn(DECLARE_SERVICE(value));
            },
        };
    }
    Out.factory = factory;
    /** Match helper for {@link Out} */
    function match(input, to) {
        return apply(to)(input);
    }
    Out.match = match;
    /** TODO: Is this replaceable by "OFFER" semantics? */
    function DECLARE_SERVICE(value) {
        return { DECLARE_SERVICE: value };
    }
    Out.DECLARE_SERVICE = DECLARE_SERVICE;
})(Out || (exports.Out = Out = {}));
/**
 * `#[codegen(tags = "protocol-driver")]`
 *
 * [Source `hn-server/src/data.rs:53`](hn-server/src/data.rs)
 */
// deno-lint-ignore no-namespace
var In;
(function (In) {
    /** Match helper for {@link In} */
    function apply(to) {
        return function _match(input) {
            // if-else strings
            // if-else objects
            if (typeof input !== "object" || input == null)
                throw new TypeError("Unexpected non-object for input");
            if ("CREATE_SERVICE" in input)
                return to.CREATE_SERVICE(input["CREATE_SERVICE"]);
            var _exhaust = input;
            throw new TypeError("Unknown object when expected In");
        };
    }
    In.apply = apply;
    /** Factory helper for {@link In} */
    function factory(fn) {
        return {
            // factory
            CREATE_SERVICE: function (value) {
                return fn(CREATE_SERVICE(value));
            },
        };
    }
    In.factory = factory;
    /** Match helper for {@link In} */
    function match(input, to) {
        return apply(to)(input);
    }
    In.match = match;
    function CREATE_SERVICE(value) {
        return { CREATE_SERVICE: value };
    }
    In.CREATE_SERVICE = CREATE_SERVICE;
})(In || (exports.In = In = {}));
//# sourceMappingURL=driver.v0.gen.ts.map
