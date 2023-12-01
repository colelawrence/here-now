<script lang="ts">
  import TodoItem from "$lib/TodoItem.svelte";
  import { mountAppInSvelte } from "$lib/mountApp.svelte";
  import { useStore } from "jotai-svelte";
  import { ArrowUp, Stop } from "phosphor-svelte";

  const store = useStore();
  const app = mountAppInSvelte(store);
  app.visibilityFilter = "SHOW_ACTIVE";

  const nextTodo = $derived(app.todos[0]);
</script>

<main
  class="p-2 grow flex gap-2 justify-stretch border-2 border-sys-primary-container rounded-lg"
  data-tauri-drag-region
>
  {#if nextTodo != null}
    {#key nextTodo.id}
      <TodoItem todo={nextTodo} nonEditableText />
    {/key}
  {:else}
    <div class="grow text-sys-on-primary text-opacity-50" data-tauri-drag-region>All done.</div>
  {/if}
  {#if app.workState.state === "working"}
    <button on:click={app.workState.stop}>
      <Stop />
    </button>
    <button on:click={app.workState.expandIntoPlanner}>
      <ArrowUp />
    </button>
  {/if}
</main>
