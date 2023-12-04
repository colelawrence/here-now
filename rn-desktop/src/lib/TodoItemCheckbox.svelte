<script lang="ts">
  import type { ITodo } from "./createApp.svelte";

  const { todo } = $props<{
    todo: ITodo;
  }>();
</script>

<input type="checkbox" bind:checked={todo.completed} id={todo.htmlCheckboxId} class="w-4 h-4" />
