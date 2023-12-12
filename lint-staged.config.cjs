// @ts-check
/** @type {import("lint-staged").Config} */
module.exports = {
  "*": ["prettier --write --ignore-unknown"],
  "**/Cargo.toml": () => {
    // regenerate workspace-hack toml
    return "cargo xtask hakari";
  },
  "**/*.rs": (files) => {
    const crates = new Set();
    for (const file of files) {
      // crates generally start with hn- or rn-
      const match = file.match(/\/((?:hn|rn)-[^/]+)/);
      if (match) crates.add(match[1]);
    }
    return [
      ...(crates.size > 0
        ? [`cargo clippy --fix --allow-staged ${[...crates].map((a) => `--package ${a}`).join(" ")} --no-deps`]
        : []),
      ...files.map((filepath) => `rustfmt --edition 2021 "${filepath}"`),
    ];
  },
  // "**/*.sql": "cargo xtask format-sql",
};
