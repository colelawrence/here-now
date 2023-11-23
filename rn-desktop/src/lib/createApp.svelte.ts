import {} from "svelte";
export type TodoItem = {
  readonly id: string;
  text: string;
  completed: boolean;
  delete(): void;
};

export type VisibilityFilter = "SHOW_ALL" | "SHOW_COMPLETED" | "SHOW_ACTIVE";

export type AppState = {
  readonly todos: TodoItem[];
  visibilityFilter: VisibilityFilter;
  addTodo: {
    text: string;
    add(): void;
  };
};

export function createApp(): AppState {
  let todos = $state<TodoItem[]>([
    createTodo("Basic Todo App", true),
    createTodo("Add Tailwind"),
    createTodo("Synchronize state to Tauri"),
  ]);
  let visibilityFilter = $state<VisibilityFilter>("SHOW_ALL");
  let addTodoText = $state("");

  return {
    get todos() {
      if (visibilityFilter === "SHOW_COMPLETED") return todos.filter((todo) => todo.completed);
      if (visibilityFilter === "SHOW_ACTIVE") return todos.filter((todo) => !todo.completed);
      return todos;
    },
    get visibilityFilter() {
      return visibilityFilter;
    },
    set visibilityFilter(updatedFilter) {
      visibilityFilter = updatedFilter;
    },
    addTodo: {
      get text() {
        return addTodoText;
      },
      set text(updatedText) {
        addTodoText = updatedText;
      },
      add() {
        todos = [...todos, createTodo(addTodoText)];
      },
    },
  };

  function createTodo(initText: string, initDone = false): TodoItem {
    const id = "T" + Date.now().toString(36) + Math.random().toString(36).slice(1);
    let text = $state(initText);
    let completed = $state(initDone);

    return {
      id,
      get text() {
        return text;
      },
      set text(updatedText: string) {
        text = updatedText;
      },
      get completed() {
        return completed;
      },
      set completed(updated) {
        completed = updated;
      },
      delete() {
        todos = todos.filter((todo) => todo.id !== id);
      },
    };
  }
}
