<script lang="ts">
  import AddTodoForm from "$lib/AddTodoForm.svelte";
  import DevInfo from "$lib/DevInfo.svelte";
  import Timer from "$lib/Timer.svelte";
  import TodoList from "$lib/TodoList.svelte";
  import { mountAppInSvelte } from "$lib/mountApp.svelte";
  import { useStore } from "jotai-svelte";
  import { ArrowDown, Play, Stop, X } from "phosphor-svelte";

  const store = useStore();
  const app = mountAppInSvelte(store);

  // t.on("drag:start", function () {
  //   u.default.Single.play("cubeUp");
  // }),
  // t.on("drag:over", function () {
  //   n && u.default.Single.play("cubeOver");
  // }),
  // t.on("drag:out", function () {
  //   n = !0;
  // }),
  // t.on("drag:stop", function () {
  //   u.default.Single.play("cubeDown"), (n = !1);
  // }),
</script>

<main class="flex flex-col items-stretch select-none" data-tauri-drag-region>
  <div class="flex justify-stretch cursor-default" data-tauri-drag-region>
    <div class="h-4">
      <!-- placeholder for window buttons -->
    </div>
    {#if app.workState.state === "planning"}
      <h1 class="text-ui-base font-semi grow py-2 select-none" data-tauri-drag-region>Planner</h1>
      <button on:click={app.workState.startSession}>
        <Play />
      </button>
    {:else if app.workState.state === "working"}
      <div class="flex flex-grow justify-center" data-tauri-drag-region>
        <Timer info={app.workState.timer} popoverPlacement="bottom" />
      </div>
      <div class="flex gap-1">
        <button on:click={app.workState.stopSession}>
          <Stop />
        </button>
        <button on:click={app.workState.collapseIntoTracker}>
          <ArrowDown />
        </button>
      </div>
    {:else if app.workState.state === "break"}
      <div class="flex flex-grow justify-center" data-tauri-drag-region>
        <Timer info={app.workState.timer} popoverPlacement="bottom" />
      </div>
      <div class="flex gap-1">
        <button on:click={app.workState.continueWorking}>
          <Play />
        </button>
        <button on:click={app.workState.collapseIntoTracker}>
          <ArrowDown />
        </button>
      </div>
    {/if}
  </div>
  <TodoList {app} />
  <div class="flex flex-col gap-2 justify-center items-center">
    <AddTodoForm bind:addTodo={app.addTodo} />
    <!-- 
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
    </div> -->
  </div>

  <div class="flex flex-wrap gap-2 p-2">
    {#each app.todoFilters.filters as filter (filter.display)}
      <button class="filter-button" class:selected={filter.enabled} on:click={filter.toggle}>
        {filter.display}
      </button>
    {/each}
    {#if app.todoFilters.canDisableAll}
      <button class="filter-button" on:click={app.todoFilters.disableAll}>
        <X />
      </button>
    {/if}
  </div>
  <!-- <DevInfo info={app.dev} /> -->
</main>

<style lang="postcss">
  .filter-button {
    @apply p-2 rounded-md opacity-50 hover:opacity-80 bg-transparent;
  }
  .filter-button.selected {
    @apply opacity-100 outline outline-sys-primary-container;
  }
</style>
