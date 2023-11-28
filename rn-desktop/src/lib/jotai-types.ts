import type { Atom, WritableAtom } from "jotai/vanilla";

type AnyAtom = Atom<unknown>;

export interface JotaiStore {
  get: <Value>(atom: Atom<Value>) => Value;
  set: <Value_1, Args extends unknown[], Result>(atom: WritableAtom<Value_1, Args, Result>, ...args: Args) => Result;
  sub: (atom: AnyAtom, listener: () => void) => () => void;
}
