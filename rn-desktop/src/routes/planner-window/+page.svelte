<script lang="ts">
  import AddTodoForm from "$lib/AddTodoForm.svelte";
  import TodoList from "$lib/TodoList.svelte";
  import { call } from "$lib/call";
  import { createApp } from "$lib/createApp.svelte";
  import { invoke } from "@tauri-apps/api";
  import { getCurrent } from "@tauri-apps/api/window";
  import { ArrowDown } from "phosphor-svelte";

  const app = createApp({
    notify: {
      reportError(message, info) {
        console.error(message, info);
        alert(message);
      },
    },
  });

  function collapseIntoTracker() {
    call(async () => {
      try {
        await invoke("start_work_session");
        await getCurrent().close();
      } catch (error) {
        console.error(error);
        alert(error);
      }
    });
  }
</script>

<main class="flex flex-col items-stretch" data-tauri-drag-region>
  <div class="flex justify-stretch cursor-default" data-tauri-drag-region>
    <div>
      <!-- placeholder for window buttons -->
    </div>
    <h1 class="text-ui-base font-semi grow py-2" data-tauri-drag-region>Planner</h1>
    <button on:click={collapseIntoTracker}>
      <ArrowDown />
    </button>
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
