<script lang="ts">
  import type { ITodo } from "./createApp.svelte";

  const { todo } = $props<{
    todo: ITodo;
  }>();
</script>

<!-- 
  use:tooltip={{
    content: todo.completed ? `Mark as incomplete` : `Mark as completed`,
  }}
 -->
<input type="checkbox" bind:checked={todo.completed} id={todo.htmlCheckboxId} class="w-4 h-4 grow-0" />
