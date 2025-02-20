import { Buffer } from "buffer";
import React from "react";
import ReactDOM from "react-dom/client";
import { Provider as JotaiProvider } from "jotai/react";

import "./styles.css";
import AppReady from "./App";
import { ProjectStore } from "./lib/store";
import { ProjectManager } from "./lib/project";
import { AppWindows } from "./lib/windows";
import { getDefaultStore } from "jotai";

if (typeof globalThis.Buffer === "undefined") {
  globalThis.Buffer = Buffer;
}

interface AppControllers {
  projectManager: ProjectManager;
  appWindows: AppWindows;
  store: ProjectStore;
}

// Initialize app controllers
async function initializeApp() {
  let controllers: AppControllers | undefined;
  let error: Error | undefined;

  try {
    console.info("Initializing app");
    // Initialize core services
    const store = await ProjectStore.initialize();
    console.info("Store initialized");
    const projectManager = new ProjectManager(store);
    console.info("Project manager initialized");
    const appWindows = new AppWindows();
    await appWindows.initialize();
    console.info("App windows initialized");

    // Wire up project change listeners
    projectManager.subscribe(async (loaded) => {
      console.log("Project changed", loaded);
      if (!loaded) {
        appWindows.setTitle(null);
        await appWindows.expandToPlanner(); // Always expand when no project
        return;
      }

      const { project } = loaded;

      // Update window/tray state with current task
      const currentTask = project.markdown?.find(
        (a): a is typeof a & { type: "task" } => a.type === "task" && !a.complete,
      );
      await appWindows.setTitle(currentTask?.name ?? null);
      // Set initial window state based on project state
      if (project.workState === "planning") {
        await appWindows.expandToPlanner();
      } else {
        await appWindows.collapseToTracker();
      }
    });

    controllers = { projectManager, appWindows, store };

    // Try to load last active project
    const lastProject = await store.getLastActiveProject();
    console.info("Opening project", { lastProject });
    await projectManager.openProject(lastProject);
  } catch (e) {
    console.error("Failed to initialize app:", e);
    error = e instanceof Error ? e : new Error(String(e));
  }

  // Render React app with controllers or error
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <JotaiProvider store={getDefaultStore()}>
        <AppReady controllers={controllers} startupError={error} />
      </JotaiProvider>
    </React.StrictMode>,
  );
}

// Start the app
initializeApp();
