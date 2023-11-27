<script lang="ts">
  import AddTodoForm from "$lib/AddTodoForm.svelte";
  import TodoList from "$lib/TodoList.svelte";
  import { createApp } from "$lib/createApp.svelte";

  const app = createApp({
    notify: {
      reportError(message, info) {
        console.error(message, info);
        alert(message);
      },
    },
  });
</script>

<main class="flex flex-col gap-2 items-stretch" data-tauri-drag-region>
  <TodoList {app} />

  <div class="flex flex-col gap-2 justify-center items-center">
    <AddTodoForm addTodo={app.addTodo} />
  </div>
</main>
