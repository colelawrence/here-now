import { invoke } from "@tauri-apps/api";
import { attemptToFocusOnInput } from "./attemptToFocusOnInput";
import { createInputTraversal, type HasHtmlInput, type HasInputTraversal } from "./createInputTraversal";
import type * as json from "./json";

export type TodoItem = HasHtmlInput &
  HasInputTraversal & {
    readonly id: string;
    text: string;
    completed: boolean;
    delete(): void;
    addTodoAfter(text: string): void;
    joinTodoBackwards(): void;
  };

export type VisibilityFilter = "SHOW_ALL" | "SHOW_COMPLETED" | "SHOW_ACTIVE";

type AddTodo = HasHtmlInput &
  HasInputTraversal & {
    text: string;
    add(): void;
  };

export type AppState = {
  readonly todos: TodoItem[];
  visibilityFilter: VisibilityFilter;
  addTodo: AddTodo;
};

export type NotifyService = {
  reportError(message: string, info: Record<string, unknown>): void;
};

export function createApp(context: { notify: NotifyService }): AppState {
  const inputTraversalNav = createInputTraversal(() => [...todos, addTodo]);
  let todos = $state<TodoItem[]>([
    newTodo("Basic Todo App", true),
    newTodo("Add Tailwind"),
    newTodo("Synchronize state to Tauri"),
  ]);
  let visibilityFilter = $state<VisibilityFilter>("SHOW_ALL");
  let addTodoText = $state("");

  const addTodo: AddTodo = {
    get text() {
      return addTodoText;
    },
    set text(updatedText) {
      addTodoText = updatedText;
    },
    htmlInputElement: null,
    add() {
      const added = newTodo(addTodoText);
      todos = [...todos, added];
      addTodoText = "";
    },
    inputTraversal: inputTraversalNav.getEscapeInput(() => addTodo),
  };

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
    addTodo,
  };

  function newTodo(initText: string, initDone = false): TodoItem {
    const id = "T" + Date.now().toString(36) + Math.random().toString(36).slice(1);
    invoke("add_todo", { id, text: initText, completed: initDone });
    return createTodo({
      id,
      text: initText,
      completed: initDone,
    });
  }
  function createTodo(init: json.Todo): TodoItem {
    let text = $state(init.text);
    let completed = $state(init.completed);
    function syncTodo() {
      invoke("update_todo", { id: init.id, text, completed });
    }

    const self: TodoItem = {
      id: init.id,
      htmlInputElement: null,
      get text() {
        return text;
      },
      set text(updatedText: string) {
        text = updatedText;
        syncTodo();
      },
      get completed() {
        return completed;
      },
      set completed(updated) {
        completed = updated;
        syncTodo();
      },
      inputTraversal: inputTraversalNav.getEscapeInput(() => self),
      delete() {
        const input = self.htmlInputElement;
        if (input) {
          if (input.contains(document.activeElement) || input === document.activeElement) {
            const before = todos.indexOf(self) - 1;
            attemptToFocusOnInput(todos[before], text.length);
          }
        }
        todos = todos.filter((todo) => todo.id !== self.id);
      },
      addTodoAfter(text) {
        const indexOfSelf = todos.indexOf(self);
        const added = newTodo(text);
        todos = [...todos.slice(0, indexOfSelf + 1), added, ...todos.slice(indexOfSelf + 1)];
        attemptToFocusOnInput(added, 0);
      },
      joinTodoBackwards() {
        const before = todos.indexOf(self) - 1;
        if (before >= 0) {
          const beforeTodo = todos[before];
          const originalLength = beforeTodo.text.length;
          beforeTodo.text += self.text;
          todos = [...todos.slice(0, before + 1), ...todos.slice(before + 2)];
          attemptToFocusOnInput(beforeTodo, originalLength);
        }
      },
    };

    return self;
  }
}
