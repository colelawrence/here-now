<script lang="ts">
  import { DotsSixVertical } from "phosphor-svelte";

  import TodoItem from "$lib/TodoItem.svelte";
  import ScrollbarContainer from "$lib/components/ScrollbarContainer.svelte";
  import type { AppState, ITodo } from "$lib/createApp.svelte";
  import DevInfo from "./DevInfo.svelte";

  const { app } = $props<{
    app: AppState;
  }>();

  let currentDndTarget = $state<":first" | string>();
  let ignoreDndTarget = $state<string>();
  let currentDraggedItem = $state<ITodo>();
  function dragEnd() {
    if (currentDndTarget && currentDraggedItem) {
      const todos = app.todos;
      const target = currentDndTarget;
      const targetTodo = currentDraggedItem;
      if (target === ":first") {
        targetTodo.ord = todos[0]?.ord - 1;
      } else {
        const dropIndex = todos.findIndex((t) => t.id === target);
        let afterOrd = todos[dropIndex + 1]?.ord;
        if (afterOrd) {
          targetTodo.ord = (todos[dropIndex]!.ord + afterOrd) / 2;
        } else {
          targetTodo.ord = todos[dropIndex]!.ord + 1;
        }
      }
    }
    currentDraggedItem = undefined;
    currentDndTarget = undefined;
  }
</script>

<ScrollbarContainer>
  <div role="list" class="flex flex-col gap-2 items-stretch p-2" class:dragging={currentDraggedItem != null}>
    <div
      class="transition-all"
      class:py-0={currentDraggedItem == null}
      class:py-1={currentDraggedItem != null}
      on:dragenter|self={() => {
        currentDndTarget = ":first";
      }}
    >
      {#if currentDndTarget === ":first"}
        <div class="h-2 -my-1 bg-sys-primary-container rounded-md" />
      {/if}
    </div>

    {#each app.todos as todo, idx (todo.id)}
      <div
        class="flex items-center justify-stretch drag-item"
        class:opacity-50={currentDraggedItem?.id === todo.id}
        role="listitem"
        on:dragstart|self={() => {
          currentDraggedItem = todo;
          ignoreDndTarget = app.todos[idx - 1]?.id;
        }}
        on:dragenter={() => {
          if (currentDraggedItem?.id === todo.id || ignoreDndTarget === todo.id) {
            currentDndTarget = undefined;
            return;
          }
          currentDndTarget = todo.id;
        }}
        on:dragexit|self={() => {
          if (currentDndTarget === todo.id) {
            currentDndTarget = undefined;
          }
        }}
        on:dragend={dragEnd}
        draggable="true"
      >
        <DotsSixVertical />
        <TodoItem bind:todo />
      </div>
      {#if currentDndTarget === todo.id && todo.id !== currentDraggedItem?.id}
        <div class="h-2 bg-sys-primary-container rounded-md -my-1" />
      {/if}
    {/each}
    <!-- <DevInfo info={app.todos.map(a => ({ id: a.id, ord: a.ord, text: a.text }))} /> -->
    <!-- <DevInfo info={currentDndTarget} /> -->
  </div>
</ScrollbarContainer>

<style>
  /* .dragging .drag-item > * > * {
    pointer-events: none;
  } */
</style>
