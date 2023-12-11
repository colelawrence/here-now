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

<main class="tracker p-1 pl-8 grow flex gap-2 justify-stretch rounded-lg shadow-sm" data-tauri-drag-region>
  {#key nextTodo?.id}
    {#if nextTodo != null}
      <div class="flex grow gap-4 items-center justify-stretch" data-tauri-drag-region>
        <TodoItemCheckbox todo={nextTodo} />
        <div class="grow overflow-x-auto">
          <label for={nextTodo.htmlCheckboxId} class="todo-label">{nextTodo.text}</label>
        </div>
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

<style lang="postcss">
  .todo-label {
    @apply text-ui-sm font-medium pointer-events-none select-none;
    @apply overflow-x-auto whitespace-nowrap flex-grow;
  }
</style>
