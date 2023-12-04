<script lang="ts">
  import { handleArrowTraversalDOM } from "$lib/handleInputTraversalDOM";
  import type { TodoTimeEstimate } from "./createApp.svelte";

  const { estimate, nonEditableText } = $props<{
    estimate: TodoTimeEstimate;
    nonEditableText?: boolean;
  }>();
</script>

<input
  type="text"
  style="width: 60px;"
  class:pointer-events-none={nonEditableText}
  disabled={nonEditableText}
  bind:value={estimate.text}
  bind:this={estimate.htmlInputElement}
  on:blur={estimate.blur}
  on:keydown={(e) => {
    if (e.key === "Enter") {
      estimate.enter();
      return;
    }

    if (handleArrowTraversalDOM(estimate, e)) {
      return;
    }

    if (e.shiftKey || e.ctrlKey || e.metaKey || e.altKey) {
      // disable behavior when modifier keys are pressed
      return;
    }
  }}
/>
