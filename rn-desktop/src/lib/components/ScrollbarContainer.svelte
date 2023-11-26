<script lang="ts">
  import { OverlayScrollbars } from "overlayscrollbars";
  import { onDestroy, onMount, type Snippet } from "svelte";

  const {
    scroll = true,
    margin = "0",
    children,
  } = $props<{
    scroll?: boolean;
    margin?: string;
    children: Snippet;
  }>();
  // export let scroll = true;
  // export let margin = "0";

  let component: HTMLDivElement;
  let osInstance: OverlayScrollbars;

  export function scrollTo(options: { left?: number; top?: number; behavior?: "auto" | "smooth" }) {
    osInstance.elements().viewport.scroll(options);
  }

  onMount(() => {
    osInstance = OverlayScrollbars(component, {
      overflow: {
        x: "hidden",
      },
      scrollbars: {
        autoHide: "leave",
        autoHideDelay: 0,
      },
    });
  });

  onDestroy(() => {
    osInstance?.destroy();
  });
</script>

{#if scroll}
  <div style="--margin: {margin}" bind:this={component}>
    {@render children()}
  </div>
{:else}
  {@render children()}
{/if}

<style lang="postcss">
  :global([data-overlayscrollbars]) {
    width: 100%;
    height: 100%;
  }

  :global([data-theme="dark"] .os-scrollbar) {
    --os-handle-bg: transparent;
    --os-handle-bg-hover: #d5d5d5;
    --os-handle-bg-active: #c8c8c8;
  }
  :global([data-theme="light"] .os-scrollbar) {
    --os-handle-bg: transparent;
    --os-handle-bg-hover: #2d2d2d;
    --os-handle-bg-active: #343434;
  }
  :global(.os-scrollbar) {
    z-index: 10;
    margin: var(--margin);
  }
</style>
