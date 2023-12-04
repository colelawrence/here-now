<script lang="ts">
  import type { TodoItem } from "./createApp.svelte";

  const { todo } = $props<{
    todo: TodoItem;
  }>();
</script>

<input type="checkbox" bind:checked={todo.completed} id={todo.htmlCheckboxId} class="w-4 h-4" />

<style>
  input[type="checkbox"] {
    /* -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none; */
    outline: none;
    border: 2px solid var(--sys-primary-container);
    border-radius: 0.25rem;
    background-color: transparent;
    transition: background-color 0.2s ease-in-out;
  }

  input[type="checkbox"]:checked {
    background-color: var(--sys-primary-container);
  }

  input[type="checkbox"]:focus {
    box-shadow: 0 0 0 2px var(--sys-primary-container);
  }
</style>
