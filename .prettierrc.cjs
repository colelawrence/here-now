/** @type {import("prettier").Config} */
module.exports = {
  printWidth: 120,
  plugins: [require.resolve("prettier-plugin-svelte"), require.resolve("prettier-plugin-organize-imports")],
  overrides: [
    { files: ["*.svelte"], options: { parser: "svelte" } },
    { files: "*.html.j2", options: { parser: "hubl" } },
  ],
};
