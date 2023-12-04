import { shallowEqual } from "fast-equals";
import type { Atom } from "jotai/vanilla";
import { atom } from "jotai/vanilla";
import { attemptToFocusOnInput } from "./attemptToFocusOnInput";
import { call } from "./call";
import { createInputTraversal, type HasHtmlInput, type HasInputTraversal } from "./createInputTraversal";
import { DisposePool } from "./DisposePool";
import type { JotaiStore } from "./jotai-types";
import { ui } from "./ui";

export type TodoItem = HasHtmlInput &
  HasInputTraversal & {
    readonly id: string;
    htmlCheckboxId: string;
    text: string;
    completed: boolean;
    delete(): void;
    addTodoAfter(text: string): void;
    joinTodoBackwards(): void;
  };

export type VisibilityFilter = "SHOW_ALL" | "SHOW_COMPLETED" | "SHOW_ACTIVE";

export type AddTodo = HasHtmlInput &
  HasInputTraversal & {
    text: string;
    add(): void;
  };

export type TimerInfo = {
  /** Unix seconds since EPOCH */
  endsAtUnix: number;
  /** Unix seconds since EPOCH */
  startedAtUnix: number;
};

export type WorkStateWorking = {
  state: "working";
  timer: TimerInfo;
  collapseIntoTracker(): void;
  expandIntoPlanner(): void;
  takeBreak(): void;
  stopSession(): void;
};

export type WorkStatePlanning = {
  state: "planning";
  startSession(): void;
};

export type WorkStateBreak = {
  state: "break";
  timer: TimerInfo;
  continueWorking(): void;
  stopSession(): void;
};

export type AppState = {
  readonly todos: TodoItem[];
  visibilityFilter: VisibilityFilter;
  addTodo: AddTodo;
  isReady: boolean;
  workState: WorkStatePlanning | WorkStateWorking | WorkStateBreak;
};

export type AppCtx = {
  store: JotaiStore;
  notify: NotifyService;
  rn: ui.RightNowTodosInvoke;
  listenToUIUpdates(handler: Partial<ui.ToUIUpdate.ApplyFns<void>>): () => void;
  sub<T>(pool: DisposePool, a: Atom<T>, fn: (val: T) => void): () => void;
};

export type NotifyService = {
  reportError(message: string, info: Record<string, unknown>): void;
};

function createCachedItem<T>(store: JotaiStore, initial: T, equal = Object.is): CachedItem<T> {
  const valueAtom = atom(initial);
  return {
    valueAtom,
    updateUnchecked(update) {
      const prev = store.get(valueAtom);
      const next = typeof update === "object" ? update.value : update(prev);
      if (equal(next, prev)) return;
      store.set(valueAtom, next);
    },
  };
}

interface CachedItem<T> {
  valueAtom: Atom<T>;
  updateUnchecked(update: { value: T } | ((prev: T) => T)): void;
}

function memoize<K, T>(fn: (key: K) => T): (key: K) => T {
  const cache = new Map<K, T>();
  return (key) => {
    if (cache.has(key)) return cache.get(key)!;
    const result = fn(key);
    cache.set(key, result);
    return result;
  };
}

let lastOrd = Math.random();

export function createApp(ctx: AppCtx): AppState {
  const rootPool = new DisposePool();
  const inputTraversalNav = createInputTraversal(() => [...todos, addTodo]);
  let todos = $state<TodoItem[]>([]);
  let isReady = $state(false);
  let workState: ui.WorkState = $state(ui.WorkState.Planning());
  const memoTodoAndCache = memoize((uid: string) => {
    const cached = createCachedItem<ui.Todo>(
      ctx.store,
      { uid, fields: { mvp_tags: [], time_estimate_mins: 25, title: "" }, completed_at: null, ord: 0, worked: [] },
      shallowEqual,
    );
    const pool = rootPool.child();
    return {
      vm: createTodo(ctx, pool, cached),
      pool,
      cached,
    };
  });
  rootPool.addfn(
    ctx.listenToUIUpdates({
      LoadTodos(inner) {
        loadTodos(inner.todos);
      },
      UpdateWorkState(update) {
        console.debug("UpdateWorkState", update);
        workState = update;
      },
      AddTodo(todo) {
        const { cached, vm } = memoTodoAndCache(todo.uid);
        cached.updateUnchecked({ value: todo });
        if (!todos.find((a) => a.id === vm.id)) {
          todos = [...todos, vm];
        }
      },
      RemoveTodo(uid) {
        todos = todos.filter((a) => a.id !== uid);
      },
      UpdateTodo([uid, update]) {
        const { cached } = memoTodoAndCache(uid);
        ui.ToUITodoUpdate.match(update, {
          AddWorkDuration(duration) {
            cached.updateUnchecked((prev) => ({ ...prev, worked: [...prev.worked, duration] }));
          },
          UpdateFields(fields) {
            cached.updateUnchecked((prev) => ({ ...prev, fields }));
          },
          UpdateCompletedAt(completed_at) {
            cached.updateUnchecked((prev) => ({ ...prev, completed_at }));
          },
          UpdateOrd(ord) {
            cached.updateUnchecked((prev) => ({ ...prev, ord }));
          },
        });
      },
    }),
  );
  call(async () => {
    try {
      await ctx.rn.load_self();
    } catch (error) {
      ctx.notify.reportError("Failed initial load", { error });
    }
  });
  function loadTodos(allTodos: ui.Todo[]) {
    todos = allTodos.map((serverTodo) => {
      const { cached, vm } = memoTodoAndCache(serverTodo.uid);
      cached.updateUnchecked({ value: serverTodo });
      return vm;
    });
    isReady = true;
    console.log("loaded todos", allTodos);
  }

  async function refreshTodos() {
    try {
      const allTodos = await ctx.rn.get_all_todos();
      loadTodos(allTodos);
    } catch (error) {
      ctx.notify.reportError("Failed to load todos", { error });
    }
  }

  refreshTodos();

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

  function toggleBig(big: boolean) {
    call(async () => {
      try {
        await ctx.rn.toggle_size({ big });
      } catch (error) {
        ctx.notify.reportError("Failed to toggle size", { error });
      }
    });
  }

  return {
    get todos() {
      if (visibilityFilter === "SHOW_COMPLETED") return todos.filter((todo) => todo.completed);
      if (visibilityFilter === "SHOW_ACTIVE") return todos.filter((todo) => !todo.completed);
      return todos;
    },
    get isReady() {
      return isReady;
    },
    get visibilityFilter() {
      return visibilityFilter;
    },
    set visibilityFilter(updatedFilter) {
      visibilityFilter = updatedFilter;
    },
    addTodo,
    get workState() {
      return ui.WorkState.match<WorkStatePlanning | WorkStateWorking | WorkStateBreak>(workState, {
        Planning: (): WorkStatePlanning => ({
          state: "planning",
          startSession() {
            call(async () => {
              try {
                await ctx.rn.start_session();
                await ctx.rn.toggle_size({ big: false });
              } catch (error) {
                ctx.notify.reportError("Failed to start session", { error });
              }
            });
          },
        }),
        Break: (inner): WorkStateBreak => ({
          state: "break",
          timer: {
            endsAtUnix: inner.ends_at_unix,
            startedAtUnix: inner.started_at_unix,
          },
          continueWorking() {
            call(async () => {
              try {
                await ctx.rn.continue_working();
                await ctx.rn.toggle_size({ big: false });
              } catch (error) {
                ctx.notify.reportError("Failed to continue working", { error });
              }
            });
          },
          stopSession() {
            call(async () => {
              try {
                await ctx.rn.stop_session();
                await ctx.rn.toggle_size({ big: true });
              } catch (error) {
                ctx.notify.reportError("Failed to stop session", { error });
              }
            });
          },
        }),
        Working: (inner): WorkStateWorking => ({
          state: "working",
          timer: {
            endsAtUnix: inner.ends_at_unix,
            startedAtUnix: inner.started_at_unix,
          },
          stopSession() {
            call(async () => {
              try {
                await ctx.rn.stop_session();
                await ctx.rn.toggle_size({ big: true });
              } catch (error) {
                ctx.notify.reportError("Failed to stop session", { error });
              }
            });
          },
          collapseIntoTracker() {
            toggleBig(false);
          },
          expandIntoPlanner() {
            toggleBig(true);
          },
          takeBreak() {
            call(async () => {
              try {
                await ctx.rn.take_a_break();
                await ctx.rn.toggle_size({ big: true });
              } catch (error) {
                ctx.notify.reportError("Failed to take break", { error });
              }
            });
          },
        }),
      });
    },
  };

  function newTodo(initText: string, initDone = false): TodoItem {
    const uid = "T" + Date.now().toString(36) + Math.random().toString(36).slice(1);
    const ord = ++lastOrd;
    const fields: ui.TodoFields = { mvp_tags: [], time_estimate_mins: 25, title: initText };
    call(async () => {
      await ctx.rn.add_todo({
        uid,
        fields,
        ord,
        template: false,
      });
      if (initDone) {
        await ctx.rn.update_todo_completed({ uid, completed: true });
      }
    });
    const { cached, vm } = memoTodoAndCache(uid);
    cached.updateUnchecked({ value: { completed_at: null, uid, fields, ord, worked: [] } });
    return vm;
  }

  function createTodo(ctx: AppCtx, pool: DisposePool, cached: CachedItem<ui.Todo>): TodoItem {
    const init = ctx.store.get(cached.valueAtom);
    let text = $state(init.fields.title);
    let completed = $state(init.completed_at != null);
    let totalMinuteEstimate = $state(init.fields.time_estimate_mins);
    let mvpTags = $state(init.fields.mvp_tags);
    ctx.sub(pool, cached.valueAtom, (upd) => {
      text = upd.fields.title;
      completed = upd.completed_at != null;
      totalMinuteEstimate = upd.fields.time_estimate_mins;
    });
    function syncTodoFields() {
      call(async () => {
        try {
          await ctx.rn.update_todo_fields({
            template: false,
            uid: init.uid,
            fields: {
              title: text,
              time_estimate_mins: totalMinuteEstimate,
              mvp_tags: mvpTags,
            },
          });
        } catch (error) {
          ctx.notify.reportError("Failed to update todo fields", { error });
        }
      });
    }

    const self: TodoItem = {
      id: init.uid,
      htmlCheckboxId: `todo-checkbox-${init.uid}`,
      htmlInputElement: null,
      get text() {
        return text;
      },
      set text(updatedText: string) {
        text = updatedText;
        syncTodoFields();
      },
      get completed() {
        return completed;
      },
      set completed(updated) {
        completed = updated;
        call(async () => {
          await ctx.rn.update_todo_completed({ uid: init.uid, completed });
        });
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
        const prev = todos;
        todos = todos.filter((todo) => todo.id !== self.id);
        call(async () => {
          try {
            await ctx.rn.delete_todo({
              uid: init.uid,
              template: false,
            });
          } catch (error) {
            ctx.notify.reportError("Failed to delete todo", { error });
            todos = prev;
          }
        });
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
