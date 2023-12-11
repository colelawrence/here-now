import { shallowEqual } from "fast-equals";
import type { Atom } from "jotai/vanilla";
import { atom } from "jotai/vanilla";
import { attemptToFocusOnInput } from "./attemptToFocusOnInput";
import { call } from "./call";
import { createInputTraversal, type HasHtmlInput, type HasInputTraversal } from "./createInputTraversal";
import { DisposePool } from "./DisposePool";
import type { JotaiStore } from "./jotai-types";
import { ui } from "./ui";

export type TodoTimeEstimate = HasHtmlInput &
  HasInputTraversal & {
    /** Flexible time text */
    text: string;
    /** Value is understood as this amount */
    readonly understoodAs?: string;
    /** Submit the current text */
    enter(): void;
    /** Unfocus, thereby resetting the value to current */
    blur(): void;
  };

export type ITodo = HasHtmlInput &
  HasInputTraversal & {
    readonly id: string;
    /** For linking a label to the checkbox input */
    readonly htmlCheckboxId: string;
    readonly timeEstimate: TodoTimeEstimate;
    /** e.g. #bucket, [[Right Now]] */
    readonly tagsInText: string[];
    /**
     * Fractional index for the ordering of this Todo item in the list.
     *
     * Future: It would probably be better if this wasn't exposed to the view at all
     * and instead we exposed functions for reordering operations on the {@link AppState}
     */
    ord: number;
    /** Notice this is not readonly, as it should be used in an `<input bind:value={app.addTodo.text}>`  */
    text: string;
    completed: boolean;
    delete(): void;
    addTodoAfter(text: string): void;
    joinTodoBackwards(): void;
  };

export type VisibilityFilter = "SHOW_ALL" | "SHOW_COMPLETED" | "SHOW_ACTIVE";

export type AddTodo = HasHtmlInput &
  HasInputTraversal & {
    /** Notice this is not readonly, as it should be used in an `<input bind:value={app.addTodo.text}>`  */
    text: string;
    add(): void;
  };

export type TimerInfo = {
  /** Unix seconds since EPOCH */
  readonly endsAtUnix: number;
  /** e.g. "Time until break" */
  readonly labelCountingDown: string;
  /** Unix seconds since EPOCH */
  readonly startedAtUnix: number;
  /** e.g. "Time working" */
  readonly labelCountingUp: string;
};

export type WorkStateWorking = {
  readonly state: "working";
  readonly timer: TimerInfo;
  collapseIntoTracker(): void;
  expandIntoPlanner(): void;
  takeBreak(): void;
  stopSession(): void;
};

export type WorkStatePlanning = {
  readonly state: "planning";
  startSession(): void;
};

export type WorkStateBreak = {
  readonly state: "break";
  readonly timer: TimerInfo;
  collapseIntoTracker(): void;
  expandIntoPlanner(): void;
  continueWorking(): void;
  stopSession(): void;
};

export type AppState = {
  readonly todos: ITodo[];
  readonly todoFilters: IAppStateFilters;
  readonly addTodo: AddTodo;
  readonly isReady: boolean;
  readonly workState: WorkStatePlanning | WorkStateWorking | WorkStateBreak;
  readonly dev: unknown;
};

export type IAppStateFilters = {
  visibilityFilter: VisibilityFilter;
  readonly filters: IFilter[];
  readonly canDisableAll: boolean;
  disableAll(): void;
};

export type IFilter = {
  readonly display: string;
  readonly enabled: boolean;
  toggle(): void;
};

export type AppCtx = {
  readonly store: JotaiStore;
  readonly notify: NotifyService;
  readonly rn: ui.RightNowTodosInvoke;
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

export function createApp(ctx: AppCtx): AppState {
  const rootPool = new DisposePool();
  const inputTraversalNav = createInputTraversal(() => [...todos.flatMap((a) => [a, a.timeEstimate]), addTodo]);
  let sourceTodos = $state<ITodo[]>([]);
  const todos = $derived(sourceTodos.toSorted((a, b) => a.ord - b.ord));
  let enabledFilters = $state<string[]>([]);
  const todosFiltered = $derived(
    call(() => {
      const filters = enabledFilters;
      if (filters.length === 0) return todos;
      return todos.filter((todo) => todo.tagsInText.some((a) => filters.includes(a)));
    }),
  );
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
        workState = update;
      },
      AddTodo(todo) {
        const { cached, vm } = memoTodoAndCache(todo.uid);
        cached.updateUnchecked({ value: todo });
        if (!todos.find((a) => a.id === vm.id)) {
          sourceTodos = [...todos, vm];
        }
      },
      RemoveTodo(uid) {
        sourceTodos = todos.filter((a) => a.id !== uid);
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
    sourceTodos = allTodos.map((serverTodo) => {
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
  const tagsRE = /(?:#(\S+)|\[\[([^\]]+)\]\])/g;
  const allHashTags = $derived(
    sourceTodos
      .flatMap((todo) => (todo.completed ? [] : todo.tagsInText))
      .reduce((totals, tag) => {
        totals.set(tag, (totals.get(tag) ?? 0) + 1);
        return totals;
      }, new Map<string, number>()),
  );
  const filters = $derived(
    [...allHashTags.entries()].map(
      ([tag, count]): IFilter => ({
        toggle() {
          const curr = enabledFilters;
          requestAnimationFrame(() => {
            enabledFilters = curr.includes(tag) ? curr.filter((a) => a !== tag) : [...curr, tag];
          });
        },
        display: `${tag} (${count})`,
        get enabled() {
          return enabledFilters.includes(tag);
        },
      }),
    ),
  );
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
      const added = newTodo(addTodoText, { position: "last" });
      sourceTodos = [...todos, added];
      addTodoText = "";
    },
    inputTraversal: inputTraversalNav.getEscapeInput(() => addTodo, { up: -2 }),
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
    get dev() {
      return {
        enabledFilters,
      };
    },
    get todos() {
      if (visibilityFilter === "SHOW_COMPLETED") return todosFiltered.filter((todo) => todo.completed);
      if (visibilityFilter === "SHOW_ACTIVE") return todosFiltered.filter((todo) => !todo.completed);
      return todosFiltered;
    },
    get isReady() {
      return isReady;
    },
    addTodo,
    todoFilters: {
      get visibilityFilter() {
        return visibilityFilter;
      },
      set visibilityFilter(updatedFilter) {
        visibilityFilter = updatedFilter;
      },
      get filters() {
        return filters;
      },
      get canDisableAll() {
        return enabledFilters.length > 0;
      },
      disableAll() {
        enabledFilters = [];
      },
    },
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
            labelCountingDown: "Time until end of break",
            endsAtUnix: inner.ends_at_unix,
            labelCountingUp: "Time on break",
            startedAtUnix: inner.started_at_unix,
          },
          collapseIntoTracker() {
            toggleBig(false);
          },
          expandIntoPlanner() {
            toggleBig(true);
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
            labelCountingDown: "Time until break",
            endsAtUnix: inner.ends_at_unix,
            labelCountingUp: "Time working",
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

  function newTodo(
    initText: string,
    options: { position: "last" | "first" | [ITodo, ITodo]; initDone?: boolean },
  ): ITodo {
    const uid = "T" + Date.now().toString(36) + Math.random().toString(36).slice(1);
    const ord =
      options.position === "first"
        ? (todos[0]?.ord ?? 1) - saltyHalf() * 10
        : options.position === "last"
          ? (todos[todos.length - 1]?.ord ?? 1) + saltyHalf() * 10
          : lerp(options.position[0].ord, options.position[1].ord, saltyHalf());

    const fields: ui.TodoFields = { mvp_tags: [], time_estimate_mins: 25, title: initText };
    call(async () => {
      await ctx.rn.add_todo({
        uid,
        fields,
        ord,
        template: false,
      });
      if (options.initDone) {
        await ctx.rn.update_todo_completed({ uid, completed: true });
      }
    });
    const { cached, vm } = memoTodoAndCache(uid);
    cached.updateUnchecked({ value: { completed_at: null, uid, fields, ord, worked: [] } });
    return vm;
  }

  function createTodo(ctx: AppCtx, pool: DisposePool, cached: CachedItem<ui.Todo>): ITodo {
    const init = ctx.store.get(cached.valueAtom);
    let text = $state(init.fields.title);
    let completed = $state(init.completed_at != null);
    let totalMinuteEstimate = $state(init.fields.time_estimate_mins);
    let ord = $state(init.ord);
    let mvpTags = $state(init.fields.mvp_tags);
    ctx.sub(pool, cached.valueAtom, (upd) => {
      ord = upd.ord;
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

    const tags = $derived([...text.matchAll(tagsRE)].map((a) => a[1] ?? a[2]));

    const self: ITodo = {
      id: init.uid,
      htmlCheckboxId: `todo-checkbox-${init.uid}`,
      htmlInputElement: null,
      get ord() {
        return ord;
      },
      set ord(updated) {
        ord = updated;
        call(async () => {
          await ctx.rn.update_todo_ord({ uid: init.uid, ord, template: false });
        });
      },
      get text() {
        return text;
      },
      set text(updatedText: string) {
        text = updatedText;
        syncTodoFields();
      },
      get tagsInText() {
        return tags;
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
      inputTraversal: inputTraversalNav.getEscapeInput(() => self, { up: -2, down: 2 }),
      timeEstimate: createTimeEstimate(),
      delete() {
        const input = self.htmlInputElement;
        if (input) {
          if (input.contains(document.activeElement) || input === document.activeElement) {
            const before = todos.indexOf(self) - 1;
            attemptToFocusOnInput(todos[before], text.length);
          }
        }
        const prev = todos;
        sourceTodos = todos.filter((todo) => todo.id !== self.id);
        call(async () => {
          try {
            await ctx.rn.delete_todo({
              uid: init.uid,
              template: false,
            });
          } catch (error) {
            ctx.notify.reportError("Failed to delete todo", { error });
            sourceTodos = prev;
          }
        });
      },
      addTodoAfter(text) {
        const indexOfSelf = todos.indexOf(self);
        const afterSelf = todos[indexOfSelf + 1];
        const added = newTodo(text, { position: afterSelf ? [self, afterSelf] : "last" });
        sourceTodos = [...todos.slice(0, indexOfSelf + 1), added, ...todos.slice(indexOfSelf + 1)];
        attemptToFocusOnInput(added, 0);
      },
      joinTodoBackwards() {
        const before = todos.indexOf(self) - 1;

        if (before >= 0) {
          const beforeTodo = todos[before];
          const originalLength = beforeTodo.text.length;
          beforeTodo.text += self.text;
          self.delete();
          attemptToFocusOnInput(beforeTodo, originalLength);
        }
      },
    };

    function createTimeEstimate(): TodoTimeEstimate {
      let understoodMins = $state<number>();
      let currentEditing = $state<string>();
      const est: TodoTimeEstimate = {
        blur() {
          // apply
          if (understoodMins) {
            totalMinuteEstimate = understoodMins;
            syncTodoFields();
          }
          currentEditing = undefined;
        },
        enter() {
          if (understoodMins) {
            totalMinuteEstimate = understoodMins;
            syncTodoFields();
          }
          currentEditing = undefined;
        },
        htmlInputElement: null,
        // pretend we're the input and we'll snap to the nearby inputs?
        inputTraversal: inputTraversalNav.getEscapeInput(() => est, { up: -2, down: 2 }),
        get text() {
          return currentEditing ?? `${totalMinuteEstimate}m`;
        },
        set text(updatedText) {
          currentEditing = updatedText;
          understoodMins = humanParseDuration(updatedText)?.minutes;
        },
        get understoodAs() {
          return understoodMins ? `${understoodMins}m` : undefined;
        },
      };

      return est;
    }

    return self;
  }
}

/* Random number between 0.4 and 0.6 for ordering mostly */
function saltyHalf() {
  return Math.random() * 0.2 + 0.4;
}

function lerp(a: number, b: number, t: number) {
  return a + (b - a) * t;
}

function humanParseDuration(input: string): undefined | { minutes: number } {
  {
    const match = input.match(/^(?<minutes>\d+)m$/);
    if (match) {
      return { minutes: parseInt(match.groups!.minutes, 10) };
    }
  }
  return undefined;
}
