<script lang="ts">
  import { handleArrowTraversalDOM } from "$lib/handleInputTraversalDOM";
  import { Trash } from "phosphor-svelte";
  import type { TodoItem } from "./createApp.svelte";
  import TodoItemCheckbox from "./TodoItemCheckbox.svelte";

  const { todo, nonEditableText } = $props<{
    todo: TodoItem;
    nonEditableText?: boolean;
  }>();
</script>

<div class="flex grow justify-stretch gap-2 items-center" data-tauri-drag-region>
  <TodoItemCheckbox {todo} />
  <input
    type="text"
    class="grow"
    class:pointer-events-none={nonEditableText}
    disabled={nonEditableText}
    bind:value={todo.text}
    bind:this={todo.htmlInputElement}
    on:keydown={(e) => {
      if (e.key === "Enter") {
        if (e.currentTarget.selectionStart !== e.currentTarget.selectionEnd || !e.currentTarget.selectionStart) {
          // if there is a selection, or the cursor is at the end of the input, add a todo after
          todo.addTodoAfter("");
          return;
        }

        // split the current selection
        todo.addTodoAfter(todo.text.slice(e.currentTarget.selectionStart));
        todo.text = todo.text.slice(0, e.currentTarget.selectionStart);
        return;
      }

      if (handleArrowTraversalDOM(todo, e)) {
        return;
      }

      if (e.shiftKey || e.ctrlKey || e.metaKey || e.altKey) {
        // disable behavior when modifier keys are pressed
        return;
      }

      if (e.key === "Backspace" && e.currentTarget.selectionStart === 0 && e.currentTarget.selectionEnd === 0) {
        requestAnimationFrame(() => {
          // avoid keyup affecting a re-focus on another todo input
          todo.joinTodoBackwards();
        });
      }
    }}
  />
  <button on:click={todo.delete}><Trash /></button>
</div>
