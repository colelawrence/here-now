<script lang="ts">
  import TodoItem from "$lib/TodoItem.svelte";
  import { call } from "$lib/call";
  import { createApp } from "$lib/createApp.svelte";
  import { invoke } from "@tauri-apps/api";
  import { getCurrent } from "@tauri-apps/api/window";
  import { ArrowUp } from "phosphor-svelte";

  const app = createApp(
    {
      notify: {
        reportError(message, info) {
          console.error(message, info);
          alert(message);
        },
      },
    },
    {
      filter: "SHOW_ACTIVE",
    },
  );

  function expandIntoPlanner() {
    call(async () => {
      try {
        await invoke("stop_work_session");
        await getCurrent().close();
      } catch (error) {
        console.error(error);
        alert(error);
      }
    });
  }

  const nextTodo = $derived(app.todos[0]);
</script>

<main class="p-2 grow flex gap-2 justify-stretch" data-tauri-drag-region>
  {#if nextTodo != null}
    <TodoItem todo={nextTodo} nonEditableText />
  {:else}
    <div class="grow text-sys-on-primary text-opacity-50" data-tauri-drag-region>All done.</div>
  {/if}
  <button on:click={expandIntoPlanner}>
    <ArrowUp />
  </button>
</main>
