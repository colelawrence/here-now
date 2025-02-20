import { path } from "@tauri-apps/api";
import { open } from "@tauri-apps/plugin-dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import type { ProjectStore } from "./store";
import { FileWatcher } from "./watcher";
import { ProjectStateEditor, type ProjectState } from "./ProjectStateEditor";
import { withError } from "./withError";

export type { ProjectState };

class ProjectError extends Error {}

export type LoadedProjectState = {
  fullPath: string;
  project: ProjectState;
  textContent: string;
  virtual: boolean;
};

type ProjectChangeCallback = (project: LoadedProjectState | undefined) => void | Promise<void>;

export class ProjectManager {
  private store: ProjectStore;
  private watcher: FileWatcher;
  private currentFile?: LoadedProjectState;
  private changeListeners: Set<ProjectChangeCallback> = new Set();

  constructor(store: ProjectStore) {
    this.store = store;
    this.watcher = new FileWatcher();
  }

  async openProject(defaultProject?: string) {
    const selected = await open({
      multiple: false,
      title: "Open TODO file",
      defaultPath: defaultProject,
      filters: [{ name: "Markdown", extensions: ["md"] }],
    });

    if (selected && !Array.isArray(selected)) {
      await this.loadProject(selected, "absolute").catch(withError(`Failed to load project (${selected})`));
    }
  }

  async loadProject(filePath: string, type: "absolute" | "virtual"): Promise<void> {
    const fullPath = type === "absolute" ? filePath : await path.resolve(await path.appDataDir(), filePath);
    const reload = async () => {
      const textContent = await readTextFile(fullPath).catch(
        withError((err) => `Error reading file (${fullPath}): ${JSON.stringify(err)}`, ProjectError),
      );
      if (this.currentFile && this.currentFile.fullPath === fullPath && this.currentFile.fullPath === textContent) {
        console.info("Skipping load of project", fullPath, "because it's already loaded");
        return;
      }
      const project = ProjectStateEditor.parse(textContent);
      this.currentFile = { fullPath, textContent, project, virtual: type === "virtual" };
      await this.notifySubscribers(this.currentFile);
    };

    await reload();
    // Update recent projects in store
    await this.store.addRecentProject(fullPath);

    // Set up file watcher for external changes
    await this.watcher.watchProject(fullPath, reload);
  }

  subscribe(callback: ProjectChangeCallback): () => void {
    callback(this.currentFile);
    this.changeListeners.add(callback);
    return () => void this.changeListeners.delete(callback);
  }

  async updateProject(fn: (project: ProjectState) => void | false) {
    if (!this.currentFile) return;
    const project = structuredClone(this.currentFile.project);
    if (fn(project) === false) return;
    this.currentFile.project = project;
    const updatedContent = ProjectStateEditor.update(this.currentFile.textContent, project);
    await writeTextFile(this.currentFile.fullPath, updatedContent);
    await this.notifySubscribers(this.currentFile);
  }

  private async notifySubscribers(project: LoadedProjectState) {
    await Promise.all(Array.from(this.changeListeners).map((listener) => Promise.resolve(listener(project))));
  }
}
