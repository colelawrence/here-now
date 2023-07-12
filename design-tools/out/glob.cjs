"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.glob = exports.globSync = void 0;
// Cross platform fast-glob
const fast_glob_1 = __importDefault(require("fast-glob"));
const path_1 = __importDefault(require("path"));
/**
 * @param {string} pathPattern
 * @returns {string[]}
 */
function globSync(pathPattern) {
    if (path_1.default.sep !== path_1.default.posix.sep) {
        // fast-glob only works with posix paths
        return fast_glob_1.default.sync(pathPattern.split(path_1.default.sep).join(path_1.default.posix.sep));
    }
    else {
        return fast_glob_1.default.sync(pathPattern);
    }
}
exports.globSync = globSync;
/**
 * @param {string} pathPattern
 * @returns {Promise<string[]>}
 */
function glob(pathPattern) {
    if (path_1.default.sep !== path_1.default.posix.sep) {
        // fast-glob only works with posix paths
        return (0, fast_glob_1.default)(pathPattern.split(path_1.default.sep).join(path_1.default.posix.sep));
    }
    else {
        return (0, fast_glob_1.default)(pathPattern);
    }
}
exports.glob = glob;
//# sourceMappingURL=glob.cjs.map