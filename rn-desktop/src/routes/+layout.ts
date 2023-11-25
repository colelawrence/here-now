import "./styles.css";
// Reference for future https://github.com/colinlienard/gitlight/blob/main/src/routes/%2Blayout.ts

// Needed for svelte kit static adapter
export const prerender = true;
// Needed so we can assume window is defined for Tauri plugins
export const ssr = false;
