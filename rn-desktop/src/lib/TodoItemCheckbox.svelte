<script lang="ts">
  import type { TodoItem } from "./createApp.svelte";

  const { todo } = $props<{
    todo: TodoItem;
  }>();
</script>

<input type="checkbox" bind:checked={todo.completed} id={todo.htmlCheckboxId} class="w-4 h-4" />
