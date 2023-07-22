"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ratioInterval = void 0;
/**
 * @param {number} base example `16`
 * @param {number} rel example `-1`, `0`, `1`
 */
function ratioInterval(base, rel) {
    const GOLDEN = 1.618033988749894;
    const GOLDEN_INTERVAL = Math.sqrt(GOLDEN);
    return Math.round(base * Math.pow(GOLDEN_INTERVAL, rel));
}
exports.ratioInterval = ratioInterval;
//# sourceMappingURL=ratioInterval.cjs.map