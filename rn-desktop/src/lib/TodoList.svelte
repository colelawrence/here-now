<script lang="ts">
  import { flip } from "svelte/animate";
  import { DotsSixVertical } from "phosphor-svelte";
  import { draggable } from "@neodrag/svelte";
  import type { DragOptions } from "@neodrag/svelte";

  import TodoItem from "$lib/TodoItem.svelte";
  import ScrollbarContainer from "$lib/components/ScrollbarContainer.svelte";
  import type { AppState, ITodo } from "$lib/createApp.svelte";

  const { app } = $props<{
    app: AppState;
  }>();

  let currentDraggedDrop = $state<":first" | ":last" | string>();
  let currentDraggedTodo = $state<ITodo>();
  const dragOptions = (todo: ITodo): DragOptions => ({
    axis: "y",
    bounds: "parent",
    onDragStart(data) {
      currentDraggedTodo = todo;
    },
    onDrag(data) {
      const relIndex = Math.round(data.offsetY / 30);
      if (relIndex === 0) {
        return;
      }
      const currentIndex = app.todos.findIndex((t) => t.id === todo.id);
      const newIndex = currentIndex + relIndex;
      if (newIndex <= 0) {
        currentDraggedDrop = ":first";
      } else if (newIndex >= app.todos.length) {
        currentDraggedDrop = app.todos[app.todos.length - 1]?.id;
      } else {
        currentDraggedDrop = app.todos[newIndex - 1]?.id;
      }
    },
    onDragEnd(data) {
      if (currentDraggedDrop && currentDraggedTodo) {
        if (currentDraggedDrop === ":first") {
          currentDraggedTodo.ord = app.todos[0]?.ord - 1;
        } else {
          const dropIndex = app.todos.findIndex((t) => t.id === currentDraggedDrop);
          let afterOrd = app.todos[dropIndex + 1]?.ord;
          if (afterOrd) {
            currentDraggedTodo.ord = (app.todos[dropIndex]!.ord + afterOrd) / 2;
          } else {
            currentDraggedTodo.ord = app.todos[dropIndex]!.ord + 1;
          }
        }
      }
      currentDraggedTodo = undefined;
      currentDraggedDrop = undefined;
      data.currentNode.style.transform = "";
    },
  });
</script>

<ScrollbarContainer>
  <div role="list" class="flex flex-col gap-2 items-stretch p-2">
    {#if currentDraggedDrop === ":last"}
      <div class="h-1 bg-sys-primary-container rounded-md" />
    {/if}
    {#each app.todos as todo (todo.id)}
      <div
        class="flex items-center justify-stretch"
        class:opacity-50={currentDraggedTodo?.id === todo.id}
        animate:flip={{ duration: 200 }}
        use:draggable={dragOptions(todo)}
      >
        <DotsSixVertical />
        <TodoItem {todo} />
      </div>
      {#if currentDraggedDrop === todo.id && todo.id !== currentDraggedTodo?.id}
        <div class="h-2 bg-sys-primary-container rounded-md -my-1" />
      {/if}
    {/each}
  </div>
</ScrollbarContainer>
