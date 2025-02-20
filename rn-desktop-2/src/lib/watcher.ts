import { watchImmediate, type UnwatchFn } from "@tauri-apps/plugin-fs";

export class FileWatcher {
  private watchers = new Map<string, UnwatchFn>();

  async watchProject(path: string, onChange: () => Promise<void>) {
    // Stop existing watcher if any
    const existingWatcher = this.watchers.get(path);
    if (existingWatcher) {
      existingWatcher();
      this.watchers.delete(path);
    }

    // Set up new watcher with immediate callback
    const stopWatching = await watchImmediate(path, async ({ type }) => {
      if (type === "any" || (typeof type === "object" && "modify" in type)) {
        await onChange();
      }
    });

    this.watchers.set(path, stopWatching);
  }

  cleanup() {
    // Stop all active watchers
    for (const [path, stop] of this.watchers.entries()) {
      stop();
      this.watchers.delete(path);
    }
  }
}
