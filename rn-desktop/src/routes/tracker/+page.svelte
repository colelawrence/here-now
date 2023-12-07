<script lang="ts">
  import Timer from "$lib/Timer.svelte";
  import TodoItemCheckbox from "$lib/TodoItemCheckbox.svelte";
  import { mountAppInSvelte } from "$lib/mountApp.svelte";
  import { useStore } from "jotai-svelte";
  import { ArrowUp, Play, Stop } from "phosphor-svelte";

  const store = useStore();
  const app = mountAppInSvelte(store);
  app.todoFilters.visibilityFilter = "SHOW_ACTIVE";

  const nextTodo = $derived(app.todos[0]);
</script>

<main class="p-2 grow flex gap-2 justify-stretch rounded-lg shadow-sm" data-tauri-drag-region>
  {#key nextTodo?.id}
    {#if nextTodo != null}
      <div class="flex grow gap-4 items-center" data-tauri-drag-region>
        <TodoItemCheckbox todo={nextTodo} />
        <label for={nextTodo.htmlCheckboxId} class="text-lg font-medium pointer-events-none select-none"
          >{nextTodo.text}</label
        >
      </div>
    {:else}
      <div class="grow text-sys-on-primary text-opacity-50" data-tauri-drag-region>All done.</div>
    {/if}
  {/key}

  {#if app.workState.state === "working" || app.workState.state === "break"}
    <Timer info={app.workState.timer} popoverPlacement="left-start" />
    <button on:click={app.workState.stopSession}>
      <Stop />
    </button>
    {#if app.workState.state === "break"}
      <button on:click={app.workState.continueWorking}>
        <Play />
      </button>
    {/if}
  {:else}
    <button on:click={app.workState.startSession}>
      <Play />
    </button>
  {/if}
  {#if app.workState.state === "working"}
    <button on:click={app.workState.expandIntoPlanner}>
      <ArrowUp />
    </button>
  {/if}
</main>
