<script lang="ts">
  import { createApp } from "./lib/createApp.svelte";

  const app = createApp();
</script>

<main class="container">
  <h1>Todos</h1>

  {#each app.todos as todo}
    <div class="todo">
      <input type="checkbox" bind:checked={todo.completed} />
      <input type="text" bind:value={todo.text} />
      <button on:click={todo.delete}>Remove</button>
    </div>
  {/each}

  <form
    on:submit|preventDefault={() => {
      app.addTodo.add();
    }}
  >
    <input type="text" bind:value={app.addTodo.text} />
    <button type="submit">Add</button>
  </form>

  <div class="row">
    <button on:click={() => (app.visibilityFilter = "SHOW_ACTIVE")}>Active</button>
    <button on:click={() => (app.visibilityFilter = "SHOW_COMPLETED")}>Completed</button>
    <button on:click={() => (app.visibilityFilter = "SHOW_ALL")}>All</button>
  </div>
  {app.visibilityFilter}
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  .todo {
    align-items: center;
    margin: 0.5em 0;
    gap: 0.25em;
  }
</style>
