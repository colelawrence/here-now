/**
 * @param {number} base example `16`
 * @param {number} rel example `-1`, `0`, `1`
 */
export function ratioInterval(base: number, rel: number) {
  const GOLDEN = 1.618033988749894;
  const GOLDEN_INTERVAL = Math.sqrt(GOLDEN);
  return Math.round(base * Math.pow(GOLDEN_INTERVAL, rel));
}
