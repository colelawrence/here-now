<script lang="ts">
  import AddTodoForm from "$lib/AddTodoForm.svelte";
  import TodoList from "$lib/TodoList.svelte";
  import { mountAppInSvelte } from "$lib/mountApp.svelte";
  import { useStore } from "jotai-svelte";
  const store = useStore();
  const app = mountAppInSvelte(store);
</script>

<main class="flex flex-col gap-2 items-stretch" data-tauri-drag-region>
  <TodoList {app} />

  <div class="flex flex-col gap-2 justify-center items-center">
    <AddTodoForm addTodo={app.addTodo} />
  </div>
</main>
