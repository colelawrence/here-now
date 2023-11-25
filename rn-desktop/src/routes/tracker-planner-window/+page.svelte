<script lang="ts">
  import ScrollbarContainer from "$lib/components/ScrollbarContainer.svelte";
  import { createApp } from "$lib/createApp.svelte";

  const app = createApp({
    notify: {
      reportError(message, info) {
        console.error(message, info);
        alert(message);
      },
    },
  });

  const { addTodo } = app;
</script>

<main class="flex flex-col gap-2 items-stretch">
  <h1>Todos</h1>
  <ScrollbarContainer>
    <div class="flex flex-col gap-2 items-stretch">
      {#each app.todos as todo (todo.id)}
        <div class="flex gap-1 justify-center items-center">
          <input type="checkbox" bind:checked={todo.completed} />
          <input
            type="text"
            bind:value={todo.text}
            bind:this={todo.htmlInputElement}
            on:keydown={(e) => {
              if (e.key === "Enter") {
                if (
                  e.currentTarget.selectionStart !== e.currentTarget.selectionEnd ||
                  !e.currentTarget.selectionStart
                ) {
                  // if there is a selection, or the cursor is at the end of the input, add a todo after
                  todo.addTodoAfter("");
                  return;
                }

                // split the current selection
                todo.addTodoAfter(todo.text.slice(e.currentTarget.selectionStart));
                todo.text = todo.text.slice(0, e.currentTarget.selectionStart);
                return;
              }

              if (e.shiftKey || e.ctrlKey || e.metaKey || e.altKey) {
                // disable behavior when modifier keys are pressed
                return;
              }

              if (e.key === "ArrowUp") {
                todo.escape.up();
              } else if (e.key === "ArrowDown") {
                todo.escape.down();
              } else if (
                e.key === "ArrowLeft" &&
                e.currentTarget.selectionStart === 0 &&
                e.currentTarget.selectionEnd === 0
              ) {
                todo.escape.exitFromStart();
              } else if (
                e.key === "ArrowRight" &&
                e.currentTarget.selectionStart === todo.text.length &&
                e.currentTarget.selectionEnd === todo.text.length
              ) {
                todo.escape.exitFromEnd();
              } else if (
                e.key === "Backspace" &&
                e.currentTarget.selectionStart === 0 &&
                e.currentTarget.selectionEnd === 0
              ) {
                requestAnimationFrame(() => {
                  // avoid keyup affecting a re-focus on another todo input
                  todo.joinTodoBackwards();
                });
              }
            }}
          />
          <button on:click={todo.delete}>Remove</button>
        </div>
      {/each}
    </div>
  </ScrollbarContainer>

  <form
    on:submit|preventDefault={() => {
      addTodo.add();
    }}
  >
    <input type="text" bind:value={addTodo.text} bind:this={addTodo.htmlInputElement} />
    <button type="submit">Add</button>
  </form>

  <div class="flex gap-1">
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
</style>
