// @ts-check
/** @type {import("lint-staged").Config} */
module.exports = {
  "*": ["prettier --write --ignore-unknown"],
  "**/*.rs": (files) => {
    const crates = new Set();
    for (const file of files) {
      // crates generally start with hn- or rn-
      const match = file.match(/\/((?:hn|rn)-[^/]+)/);
      if (match) crates.add(match[1]);
    }
    return [
      ...Array.from(crates).map((a) => `cargo clippy --fix --allow-staged --color always --package ${a} --no-deps`),
      ...files.map((filepath) => `rustfmt --edition 2021 "${filepath}"`),
    ];
  },
};
