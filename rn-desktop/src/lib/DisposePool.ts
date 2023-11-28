export interface IDisposable {
  dispose(): void;
}

export class DisposePool implements IDisposable {
  #fns: null | (() => void)[] = [];
  #pool: null | IDisposable[] = [];
  get disposed() {
    return this.#pool === null;
  }
  dispose() {
    if (this.#pool == null) return;
    const pool = this.#pool;
    this.#pool = null;
    const fns = this.#fns!;
    this.#fns = null;
    for (let i = 0; i < pool.length; i++) {
      pool[i].dispose();
    }
    for (let i = 0; i < fns.length; i++) {
      fns[i]();
    }
  }
  child() {
    const child = new DisposePool();
    this.add(child);
    return child;
  }
  addfn(fn: () => void) {
    if (this.#fns) {
      this.#fns.push(fn);
    } else {
      fn();
    }
  }
  add<T extends IDisposable>(value: T): T {
    if (this.#pool) {
      this.#pool.push(value);
    } else {
      value.dispose();
    }
    return value;
  }
}

export function isDisposable(value: any): value is IDisposable {
  return value && typeof value.dispose === "function";
}
