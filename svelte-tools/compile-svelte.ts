import { glob } from "npm:glob";
import { OutputChunk, rollup } from "npm:rollup";
import _typescript from "npm:rollup-plugin-esbuild";
const typescript: typeof import("npm:rollup-plugin-esbuild").default = _typescript as any;
import _svelte from "npm:rollup-plugin-svelte";
const svelte: typeof import("npm:rollup-plugin-svelte").default = _svelte as any;
import _sveltePreprocess from "npm:svelte-preprocess";
const sveltePreprocess: typeof import("npm:svelte-preprocess").default = _sveltePreprocess as any;
import _resolve from "npm:@rollup/plugin-node-resolve";
const resolve: typeof import("npm:svelte-preprocess").default = _resolve as any;

/**
 * Svelte component bundler
 *
 * Takes all .svelte files and run them through:
 * - Svelte compiler:
 *   - preprocess them to format the typescript (<script lang="ts">) part as ESM
 *     This is required for the props to properly work with svelte compiler
 *   - compile the whole file as an SSR component
 *   - doesn't emit the CSS, meaning, a separate [name].css file is not going to
 *     be generated in the bundle. The CSS is going to be part of the SSR component.
 * - node module resolution in order to rewrite the resolutions
 *
 * The 'external' block is here to tell if a file is external or not. In this case, all
 * non .svelte is.
 * Meaning, the .ts files (for example) are going to be treated as external. They
 * are going to be part of the module resolution, but not the bundling.
 */
glob.glob("**/*.template.svelte", {}).then(async (files: string[]) => {
  await Promise.allSettled(files.map(compileSvelteFile)).then((results) => {
    const rejects = results.filter((a) => a.status === "rejected");
    if (rejects.length > 0) {
      console.error("Failed to compile all svelte files:");
      rejects.forEach((rej) => console.error(rej));
      Deno.exit(1);
    }
  });
});

async function compileSvelteFile(file: string) {
  const bundle = await rollup({
    input: [file],
    external: (source, importer, isResolved) => {
      // const isSvelte = /.*\.svelte$/.test(source)
      // return !isSvelte;
      // what happens if everything is not external? Will it bundle it all for us?
      return false;
    },
    plugins: [
      svelte({
        preprocess: sveltePreprocess({
          typescript: {
            compilerOptions: {
              module: "esnext",
            },
          },
        }),
        compilerOptions: {
          // generate: "dom",
          generate: "ssr",
          // // only applies to "dom" generation, I think...
          // hydratable: true,
        },
        // dropping .css (it'll be in the SSR code)
        emitCss: false,
      }),
      // rewrite resolutions
      resolve({
        // jail prevents node_modules resolution, or not, depending on what you want here
        // this will ensure that anything imported from node_modules will continue to import from node_modules.
        // jail: "node_modules",
      }),
      typescript({}),
    ],
  });

  // generating code w/o writing it
  // generates as cjs for the runtime to be able to consume them
  const gen = await bundle.generate({
    format: "cjs",
    // format: "iife",
    // in quickjs, cannot eval and give us a better handle
  });

  // Safeguard to drop all non .svelte related files.
  // we only take chunks (as opposed to assets), and .svelte files
  const bundledSvelteFiles = gen.output.filter(
    (f) => f.type === "chunk" && f.facadeModuleId?.includes(".svelte"),
  ) as OutputChunk[];

  // For each .svelte file, we rename the filename and target them in the proper dist directory
  // const cwd = process.cwd();
  const cwd = Deno.cwd();
  const tsconfig = JSON.parse(await Deno.readTextFile(cwd + "/tsconfig.json")) as {
    compilerOptions: { outDir: string; rootDir: string };
  };
  const { rootDir, outDir } = tsconfig.compilerOptions;
  for (const f of bundledSvelteFiles) {
    const { facadeModuleId, fileName, code } = f;

    assert(facadeModuleId, "Missing facadeModuleId", f);
    // pwd + (orignal path, stripped of pwd, targetted to dist) + (filename stripped of ext + '.svelte-preview-component.js')
    const bundleFileName = fileName.replace(".js", ".gen.cjs");
    const bundleDir = `${cwd}/${facadeModuleId
      .replace(cwd, "")
      .split("/")
      .slice(0, -1)
      .join("/")
      .replace(`/${rootDir}/`, `/${outDir}/`)}`;
    // fs.writeFileSync(`${bundleDir}/${bundleFileName}`, code);
    await Deno.writeTextFile(`${bundleDir}/${bundleFileName}`, code);
  }
}

// corrected type def for console.assert
function assert(value: any, ...args: any): asserts value {
  console.assert(value, ...args);
}
