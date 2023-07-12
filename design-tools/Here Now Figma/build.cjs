// @ts-check
const watch = process.argv.includes("--watch");

const esbuild = require("esbuild");

esbuild
  .context({
    bundle: true,
    tsconfig: "tsconfig.json",
    outfile: "plugin.js",
    entryPoints: ["./entry.ts"],
    target: "ES2019" // If QuickJS doesn't support, then continue lowering this.
  })
  .then((ctx) => {
    if (watch) {
      return ctx.watch();
    } else {
      return ctx.rebuild().then((result) => {
        console.info(result);
        return ctx.dispose();
      });
    }
  })
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
