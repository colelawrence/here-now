import { invoke } from "@tauri-apps/api";
import * as json from "./json";
export type TodoItem = HasHtmlInput & {
  readonly id: string;
  text: string;
  completed: boolean;
  delete(): void;
  addTodoAfter(text: string): void;
  joinTodoBackwards(): void;
  escape: {
    up(): void;
    down(): void;
    exitFromStart(): void;
    exitFromEnd(): void;
  };
};

export type VisibilityFilter = "SHOW_ALL" | "SHOW_COMPLETED" | "SHOW_ACTIVE";

type AddTodo = HasHtmlInput & {
  text: string;
  add(): void;
};
type HasHtmlInput = {
  htmlInputElement: HTMLInputElement | null;
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

    function currentSelectionOr(): number | null {
      return self.htmlInputElement?.selectionStart ?? null;
    }
    function getRelativeInput(dir: number): HasHtmlInput {
      const index = todos.indexOf(self);
      const isLast = index === todos.length - 1;
      if (isLast && dir > 0) {
        console.log({ index, dir, todos });
        // focus on end add todo
        return addTodo;
      }

      return todos[index + (dir % todos.length)];
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
      escape: {
        down() {
          attemptToFocusOnInput(getRelativeInput(1), currentSelectionOr() ?? 0);
        },
        up() {
          attemptToFocusOnInput(getRelativeInput(-1), currentSelectionOr() ?? Infinity);
        },
        exitFromStart() {
          attemptToFocusOnInput(getRelativeInput(-1), Infinity);
        },
        exitFromEnd() {
          attemptToFocusOnInput(getRelativeInput(1), 0);
        },
      },
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

function attemptToFocusOnInput(sourceMaybe: HasHtmlInput | null | undefined, selectionIndex?: number) {
  let attempt = 3;
  if (sourceMaybe == null) return;
  requestAnimationFrame(run);
  const source = sourceMaybe;
  function run() {
    const input = source.htmlInputElement;
    if (input) {
      input.focus();
      const sel = Math.min(input.value.length, selectionIndex ?? Infinity);
      input.setSelectionRange(sel, sel);
      return;
    }

    if (attempt-- > 0) {
      requestAnimationFrame(run);
    }
  }
}
