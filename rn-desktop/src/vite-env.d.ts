/// <reference types="svelte" />
/// <reference types="vite/client" />

declare global {
  interface HTMLAttributes<Element> {
    "on:consider"?: (evt: { detail: { items: any[] } }) => void;
    "on:finalize"?: (evt: { detail: { items: any[] } }) => void;
  }
}
