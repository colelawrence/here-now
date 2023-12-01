type UID = string;
function createInvoker(invoke: Function, prefix = ""): any {
  return new Proxy(
    {},
    {
      get(target, command, receiver) {
        if (typeof command !== "string") throw new TypeError("Expected string command");
        return function (options: any) {
          return invoke(prefix + command, options);
        };
      },
    },
  );
}
// TODO: make this generic for other plugins
export function createRightNowInvoker(invoke: Function): RightNowStateInvoke {
  return createInvoker(invoke, "plugin:RightNowTodos|");
}

export interface RightNowStateInvoke {
  /**
   * `invoke("get_all_todos", {})`
   *
   * `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]`
   */
  get_all_todos(): Promise<Result_OkTodo_List_ErrError.Ok["Ok"]>;
  /**
   * `invoke("update_todo_fields", { uid, fields, template })`
   *
   * `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]`
   */
  update_todo_fields(options: {
    uid: UID;
    fields: TodoFields;
    template: boolean;
  }): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /**
   * `invoke("update_todo_completed", { uid, completed })`
   *
   * `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]`
   */
  update_todo_completed(options: { uid: UID; completed: boolean }): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /** `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]` */
  update_todo_ord(options: { uid: UID; ord: number; template: boolean }): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /** `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]` */
  add_todo(options: {
    uid: UID;
    ord: number;
    fields: TodoFields;
    template: boolean;
  }): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /** `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]` */
  delete_todo(options: { uid: UID; template: boolean }): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /** `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]` */
  start_session(): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /** `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]` */
  take_a_break(): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /** `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]` */
  toggle_size(options: { big: boolean }): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
  /** `#[codegen(tauri_command, tags = "rn-ui", tauri_plugin = "RightNowTodos")]` */
  stop_session(): Promise<Result_OkTuple_ErrError.Ok["Ok"]>;
}
/**
 * Future: Store this as the only state stored to disk for this app
 *
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:7`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type AppSettings = {
  /** How long should breaks last (5m in pomodoro) */
  break_secs: number;
  /** How long should work sessions last (25m in pomodoro) */
  working_secs: number;
  /** Todo items that can be re-used */
  template_todos: Array<TemplateTodo>;
};
/**
 * Future: Store this as the only state stored to disk for this app
 *
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:7`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export function AppSettings(inner: AppSettings): AppSettings {
  return inner;
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:18`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
// deno-lint-ignore no-namespace
export namespace ToUITodoUpdate {
  export type ApplyFns<R> = {
    // callbacks
    UpdateFields(inner: UpdateFields["UpdateFields"]): R;
    UpdateCompletedAt(inner: UpdateCompletedAt["UpdateCompletedAt"]): R;
    AddWorkDuration(inner: AddWorkDuration["AddWorkDuration"]): R;
    UpdateOrd(inner: UpdateOrd["UpdateOrd"]): R;
  };
  /** Match helper for {@link ToUITodoUpdate} */
  export function apply<R>(to: ApplyFns<R>): (input: ToUITodoUpdate) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("UpdateFields" in input) return to.UpdateFields(input["UpdateFields"]);
      if ("UpdateCompletedAt" in input) return to.UpdateCompletedAt(input["UpdateCompletedAt"]);
      if ("AddWorkDuration" in input) return to.AddWorkDuration(input["AddWorkDuration"]);
      if ("UpdateOrd" in input) return to.UpdateOrd(input["UpdateOrd"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected ToUITodoUpdate");
    };
  }
  /** Match helper for {@link ToUITodoUpdate} */
  export function match<R>(input: ToUITodoUpdate, to: ApplyFns<R>): R {
    return apply(to)(input);
  }
  export type UpdateFields = {
    UpdateFields: TodoFields;
  };
  export function UpdateFields(value: TodoFields): UpdateFields {
    return { UpdateFields: value };
  }
  export type UpdateCompletedAt = {
    UpdateCompletedAt: number | undefined | null;
  };
  export function UpdateCompletedAt(value?: number | undefined | null): UpdateCompletedAt {
    return { UpdateCompletedAt: value };
  }
  export type AddWorkDuration = {
    AddWorkDuration: TodoWorkDuration;
  };
  export function AddWorkDuration(value: TodoWorkDuration): AddWorkDuration {
    return { AddWorkDuration: value };
  }
  export type UpdateOrd = {
    UpdateOrd: number;
  };
  export function UpdateOrd(value: number): UpdateOrd {
    return { UpdateOrd: value };
  }
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:18`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type ToUITodoUpdate =
  | ToUITodoUpdate.UpdateFields
  | ToUITodoUpdate.UpdateCompletedAt
  | ToUITodoUpdate.AddWorkDuration
  | ToUITodoUpdate.UpdateOrd;
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:33`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
// deno-lint-ignore no-namespace
export namespace ToUITemplateTodoUpdate {
  export type ApplyFns<R> = {
    // callbacks
    UpdateFields(inner: UpdateFields["UpdateFields"]): R;
    UpdateOrd(inner: UpdateOrd["UpdateOrd"]): R;
  };
  /** Match helper for {@link ToUITemplateTodoUpdate} */
  export function apply<R>(to: ApplyFns<R>): (input: ToUITemplateTodoUpdate) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("UpdateFields" in input) return to.UpdateFields(input["UpdateFields"]);
      if ("UpdateOrd" in input) return to.UpdateOrd(input["UpdateOrd"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected ToUITemplateTodoUpdate");
    };
  }
  /** Match helper for {@link ToUITemplateTodoUpdate} */
  export function match<R>(input: ToUITemplateTodoUpdate, to: ApplyFns<R>): R {
    return apply(to)(input);
  }
  export type UpdateFields = {
    UpdateFields: TodoFields;
  };
  export function UpdateFields(value: TodoFields): UpdateFields {
    return { UpdateFields: value };
  }
  export type UpdateOrd = {
    UpdateOrd: number;
  };
  export function UpdateOrd(value: number): UpdateOrd {
    return { UpdateOrd: value };
  }
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:33`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type ToUITemplateTodoUpdate = ToUITemplateTodoUpdate.UpdateFields | ToUITemplateTodoUpdate.UpdateOrd;
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:46`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
// deno-lint-ignore no-namespace
export namespace ToUIUpdate {
  export type ApplyFns<R> = {
    // callbacks
    /** Initial load */
    LoadTodos(inner: LoadTodos["LoadTodos"]): R;
    UpdateWorkState(inner: UpdateWorkState["UpdateWorkState"]): R;
    AddTodo(inner: AddTodo["AddTodo"]): R;
    UpdateTodo(inner: [UID, ToUITodoUpdate]): R;
    RemoveTodo(inner: RemoveTodo["RemoveTodo"]): R;
    AddTemplateTodo(inner: AddTemplateTodo["AddTemplateTodo"]): R;
    UpdateTemplateTodo(inner: [UID, ToUITemplateTodoUpdate]): R;
    RemoveTemplateTodo(inner: RemoveTemplateTodo["RemoveTemplateTodo"]): R;
  };
  /** Match helper for {@link ToUIUpdate} */
  export function apply<R>(to: ApplyFns<R>): (input: ToUIUpdate) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("LoadTodos" in input) return to.LoadTodos(input["LoadTodos"]);
      if ("UpdateWorkState" in input) return to.UpdateWorkState(input["UpdateWorkState"]);
      if ("AddTodo" in input) return to.AddTodo(input["AddTodo"]);
      if ("UpdateTodo" in input) return to.UpdateTodo(input["UpdateTodo"]);
      if ("RemoveTodo" in input) return to.RemoveTodo(input["RemoveTodo"]);
      if ("AddTemplateTodo" in input) return to.AddTemplateTodo(input["AddTemplateTodo"]);
      if ("UpdateTemplateTodo" in input) return to.UpdateTemplateTodo(input["UpdateTemplateTodo"]);
      if ("RemoveTemplateTodo" in input) return to.RemoveTemplateTodo(input["RemoveTemplateTodo"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected ToUIUpdate");
    };
  }
  /** Match helper for {@link ToUIUpdate} */
  export function match<R>(input: ToUIUpdate, to: ApplyFns<R>): R {
    return apply(to)(input);
  }
  /** Initial load */
  export type LoadTodos = {
    /** Initial load */
    LoadTodos: {
      todos: Array<Todo>;
      template_todos: Array<TemplateTodo>;
    };
  };
  /** Initial load */
  export function LoadTodos(value: LoadTodos["LoadTodos"]): LoadTodos {
    return { LoadTodos: value };
  }
  export type UpdateWorkState = {
    UpdateWorkState: WorkState;
  };
  export function UpdateWorkState(value: WorkState): UpdateWorkState {
    return { UpdateWorkState: value };
  }
  export type AddTodo = {
    AddTodo: Todo;
  };
  export function AddTodo(value: Todo): AddTodo {
    return { AddTodo: value };
  }
  export type UpdateTodo = { UpdateTodo: [UID, ToUITodoUpdate] };
  export function UpdateTodo(a: UID, b: ToUITodoUpdate): UpdateTodo {
    return { UpdateTodo: [a, b] };
  }
  export type RemoveTodo = {
    RemoveTodo: UID;
  };
  export function RemoveTodo(value: UID): RemoveTodo {
    return { RemoveTodo: value };
  }
  export type AddTemplateTodo = {
    AddTemplateTodo: TemplateTodo;
  };
  export function AddTemplateTodo(value: TemplateTodo): AddTemplateTodo {
    return { AddTemplateTodo: value };
  }
  export type UpdateTemplateTodo = { UpdateTemplateTodo: [UID, ToUITemplateTodoUpdate] };
  export function UpdateTemplateTodo(a: UID, b: ToUITemplateTodoUpdate): UpdateTemplateTodo {
    return { UpdateTemplateTodo: [a, b] };
  }
  export type RemoveTemplateTodo = {
    RemoveTemplateTodo: UID;
  };
  export function RemoveTemplateTodo(value: UID): RemoveTemplateTodo {
    return { RemoveTemplateTodo: value };
  }
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:46`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type ToUIUpdate =
  | ToUIUpdate.LoadTodos
  | ToUIUpdate.UpdateWorkState
  | ToUIUpdate.AddTodo
  | ToUIUpdate.UpdateTodo
  | ToUIUpdate.RemoveTodo
  | ToUIUpdate.AddTemplateTodo
  | ToUIUpdate.UpdateTemplateTodo
  | ToUIUpdate.RemoveTemplateTodo;
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:63`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
// deno-lint-ignore no-namespace
export namespace WorkState {
  export type ApplyFns<R> = {
    // callbacks
    Planning(): R;
    Break(inner: Break["Break"]): R;
    Working(inner: Working["Working"]): R;
  };
  /** Match helper for {@link WorkState} */
  export function apply<R>(to: ApplyFns<R>): (input: WorkState) => R {
    return function _match(input): R {
      // if-else strings
      if (input === "Planning") return to.Planning();
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Break" in input) return to.Break(input["Break"]);
      if ("Working" in input) return to.Working(input["Working"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected WorkState");
    };
  }
  /** Match helper for {@link WorkState} */
  export function match<R>(input: WorkState, to: ApplyFns<R>): R {
    return apply(to)(input);
  }
  export type Planning = "Planning";
  export function Planning(): Planning {
    return "Planning";
  }
  export type Break = {
    Break: {
      /** Time the break is over */
      ends_at_unix: number;
      /** Time the break started */
      started_at_unix: number;
    };
  };
  export function Break(value: Break["Break"]): Break {
    return { Break: value };
  }
  export type Working = {
    Working: {
      /** Time the work session is over */
      ends_at_unix: number;
      /** Time the work session started */
      started_at_unix: number;
    };
  };
  export function Working(value: Working["Working"]): Working {
    return { Working: value };
  }
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:63`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type WorkState = WorkState.Planning | WorkState.Break | WorkState.Working;
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:81`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type TodoFields = {
  /**
   * Future: Full text content of the todo (first new line separates the title from description)
   * Future: Can link to media IDs
   */
  title: string;
  /** In minutes */
  time_estimate_mins: number;
  /**
   * Tags for categorization and quick organization
   * e.g. `["user:Passport", "when:later", "user:Important"]`
   */
  mvp_tags: Array<string>;
};
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:81`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export function TodoFields(inner: TodoFields): TodoFields {
  return inner;
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:94`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type Todo = {
  uid: UID;
  /** What order is this todo item in the universal ordering */
  ord: number;
  /** Seconds since Unix epoch */
  completed_at?: number | undefined | null | null | undefined;
  /** Segments of work performed */
  worked: Array<TodoWorkDuration>;
  fields: TodoFields;
};
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:94`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export function Todo(inner: Todo): Todo {
  return inner;
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:107`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type TodoWorkDuration = {
  started_at_unix: number;
  stopped_at_unix: number;
};
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:107`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export function TodoWorkDuration(inner: TodoWorkDuration): TodoWorkDuration {
  return inner;
}
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:114`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export type TemplateTodo = {
  uid: UID;
  /** What order is this in the template list */
  ord_in_template_list: number;
  fields: TodoFields;
};
/**
 * `#[codegen(tags = "rn-ui")]`
 *
 * [Source `rn-desktop/src-tauri/src/ui.rs:114`](../../../rn-desktop/src-tauri/src/ui.rs)
 */
export function TemplateTodo(inner: TemplateTodo): TemplateTodo {
  return inner;
}
/**
 * `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
 *
 * [Source `rn-desktop/src-tauri/src/rn_todos_plugin.rs:45`](../../../rn-desktop/src-tauri/src/rn_todos_plugin.rs)
 */
// deno-lint-ignore no-namespace
export namespace Result_OkTodo_List_ErrError {
  export type ApplyFns<R> = {
    // callbacks
    /** Contains the success value */
    Ok(inner: Ok["Ok"]): R;
    /** Contains the error value */
    Err(inner: Err["Err"]): R;
  };
  /** Match helper for {@link Result_OkTodo_List_ErrError} */
  export function apply<R>(to: ApplyFns<R>): (input: Result_OkTodo_List_ErrError) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Ok" in input) return to.Ok(input["Ok"]);
      if ("Err" in input) return to.Err(input["Err"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected Result_OkTodo_List_ErrError");
    };
  }
  /** Match helper for {@link Result_OkTodo_List_ErrError} */
  export function match<R>(input: Result_OkTodo_List_ErrError, to: ApplyFns<R>): R {
    return apply(to)(input);
  }
  /** Contains the success value */
  export type Ok = {
    /** Contains the success value */
    Ok: Array<Todo>;
  };
  /** Contains the success value */
  export function Ok(value: Array<Todo>): Ok {
    return { Ok: value };
  }
  /** Contains the error value */
  export type Err = {
    /** Contains the error value */
    Err: Error;
  };
  /** Contains the error value */
  export function Err(value: Error): Err {
    return { Err: value };
  }
}
/**
 * `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
 *
 * [Source `rn-desktop/src-tauri/src/rn_todos_plugin.rs:45`](../../../rn-desktop/src-tauri/src/rn_todos_plugin.rs)
 */
export type Result_OkTodo_List_ErrError = Result_OkTodo_List_ErrError.Ok | Result_OkTodo_List_ErrError.Err;
/**
 * `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
 *
 * [Source `rn-desktop/src-tauri/src/rn_todos_plugin.rs:54`](../../../rn-desktop/src-tauri/src/rn_todos_plugin.rs)
 */
// deno-lint-ignore no-namespace
export namespace Result_OkTuple_ErrError {
  export type ApplyFns<R> = {
    // callbacks
    /** Contains the success value */
    Ok(inner: Ok["Ok"]): R;
    /** Contains the error value */
    Err(inner: Err["Err"]): R;
  };
  /** Match helper for {@link Result_OkTuple_ErrError} */
  export function apply<R>(to: ApplyFns<R>): (input: Result_OkTuple_ErrError) => R {
    return function _match(input): R {
      // if-else strings
      // if-else objects
      if (typeof input !== "object" || input == null) throw new TypeError("Unexpected non-object for input");
      if ("Ok" in input) return to.Ok(input["Ok"]);
      if ("Err" in input) return to.Err(input["Err"]);
      const _exhaust: never = input;
      throw new TypeError("Unknown object when expected Result_OkTuple_ErrError");
    };
  }
  /** Match helper for {@link Result_OkTuple_ErrError} */
  export function match<R>(input: Result_OkTuple_ErrError, to: ApplyFns<R>): R {
    return apply(to)(input);
  }
  /** Contains the success value */
  export type Ok = {
    /** Contains the success value */
    Ok: [];
  };
  /** Contains the success value */
  export function Ok(value: []): Ok {
    return { Ok: value };
  }
  /** Contains the error value */
  export type Err = {
    /** Contains the error value */
    Err: Error;
  };
  /** Contains the error value */
  export function Err(value: Error): Err {
    return { Err: value };
  }
}
/**
 * `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
 *
 * [Source `rn-desktop/src-tauri/src/rn_todos_plugin.rs:54`](../../../rn-desktop/src-tauri/src/rn_todos_plugin.rs)
 */
export type Result_OkTuple_ErrError = Result_OkTuple_ErrError.Ok | Result_OkTuple_ErrError.Err;
