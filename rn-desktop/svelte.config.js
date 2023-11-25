import adapterStatic from "@sveltejs/adapter-static";
// import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import("@sveltejs/kit").Config} */
export default {
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  compilerOptions: {
    sourcemap: true,
  },
  vitePlugin: {
    dynamicCompileOptions({ filename }) {
      if (filename.includes("node_modules")) {
        return { runes: false }; // or false, check what works
      }

      return { runes: true, sourcemap: true };
    },
  },
  kit: {
    files: {
      assets: "static",
    },
    // prerender: {
    //   entries: ["*"],
    // },
    adapter: adapterStatic({
      // precompress: true,
    }),
  },
};
