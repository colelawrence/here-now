<script lang="ts">
  import AddTodoForm from "$lib/AddTodoForm.svelte";
  import TodoList from "$lib/TodoList.svelte";
  import { mountAppInSvelte } from "$lib/mountApp.svelte";
  import { useStore } from "jotai-svelte";
  import { ArrowDown, Play, Stop } from "phosphor-svelte";

  const store = useStore();
  const app = mountAppInSvelte(store);
</script>

<main class="flex flex-col items-stretch" data-tauri-drag-region>
  <div class="flex justify-stretch cursor-default" data-tauri-drag-region>
    <div>
      <!-- placeholder for window buttons -->
    </div>
    <h1 class="text-ui-base font-semi grow py-2" data-tauri-drag-region>Planner</h1>
    {#if app.workState.state === "planning"}
      <button on:click={app.workState.start}>
        <Play />
      </button>
    {:else if app.workState.state === "working"}
      <div class="flex gap-1">
        <button on:click={app.workState.stop}>
          <Stop />
        </button>
        <button on:click={app.workState.collapseIntoTracker}>
          <ArrowDown />
        </button>
      </div>
    {/if}
  </div>
  <TodoList {app} />
  <div class="flex flex-col gap-2 justify-center items-center">
    <AddTodoForm addTodo={app.addTodo} />

    <div class="flex gap-1">
      <button
        on:click={() => (app.visibilityFilter = "SHOW_ACTIVE")}
        class:opacity-50={app.visibilityFilter === "SHOW_ACTIVE"}>Active</button
      >
      <button
        on:click={() => (app.visibilityFilter = "SHOW_COMPLETED")}
        class:opacity-50={app.visibilityFilter === "SHOW_COMPLETED"}>Completed</button
      >
      <button
        on:click={() => (app.visibilityFilter = "SHOW_ALL")}
        class:opacity-50={app.visibilityFilter === "SHOW_ALL"}>All</button
      >
    </div>
  </div>
</main>
