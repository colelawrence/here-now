<script lang="ts">
  import TodoItem from "$lib/TodoItem.svelte";
  import ScrollbarContainer from "$lib/components/ScrollbarContainer.svelte";
  import type { AppState } from "$lib/createApp.svelte";

  const { app } = $props<{
    app: AppState;
  }>();
</script>

<ScrollbarContainer>
  <div class="flex flex-col gap-2 items-stretch p-2">
    {#each app.todos as todo (todo.id)}
      <TodoItem {todo} />
    {/each}
  </div>
</ScrollbarContainer>
