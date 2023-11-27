<script lang="ts">
  import { handleArrowTraversalDOM } from "$lib/handleInputTraversalDOM";
  import type { AddTodo } from "./createApp.svelte";

  const { addTodo } = $props<{
    addTodo: AddTodo;
  }>();
</script>

<form
  on:submit|preventDefault={() => {
    addTodo.add();
  }}
>
  <input
    type="text"
    bind:value={addTodo.text}
    bind:this={addTodo.htmlInputElement}
    on:keydown={(e) => {
      if (handleArrowTraversalDOM(addTodo, e)) {
        return;
      }
    }}
  />
  <button type="submit">Add</button>
</form>
