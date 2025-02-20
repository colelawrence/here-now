import { useEffect, useState } from "react";
import type { ProjectManager, AppWindows, ProjectStore } from "./lib";
import type { LoadedProjectState, ProjectState } from "./lib/project";
import { Timer } from "./components/Timer";
import { StateControls } from "./components/StateControls";
import { openPath } from "@tauri-apps/plugin-opener";
import type { ProjectMarkdown } from "./lib/ProjectStateEditor";
import { IconEdit } from "@tabler/icons-react";

interface AppControllers {
  projectManager: ProjectManager;
  appWindows: AppWindows;
  store: ProjectStore;
}

interface AppProps {
  controllers?: AppControllers;
  startupError?: Error;
}

function AppOuter({ controllers, startupError }: AppProps) {
  // If we have a startup error, show error UI
  if (startupError) {
    return (
      <main className="h-screen flex items-center justify-center bg-red-100">
        <div className="px-6 py-4">
          <h1 className="text-lg font-semibold text-red-700 mb-2">Startup Error</h1>
          <div className="bg-red-50 px-3 py-2 rounded">
            <p className="text-red-700 font-mono text-sm whitespace-pre-wrap">{startupError.message}</p>
          </div>
          <button
            onClick={() => window.location.reload()}
            className="mt-3 px-3 py-1.5 bg-red-600 text-white text-sm rounded hover:bg-red-700 transition-colors"
          >
            Retry
          </button>
        </div>
      </main>
    );
  }

  // If we don't have controllers (but no error), show loading state
  if (!controllers) {
    return (
      <main className="h-screen flex items-center justify-center bg-gray-50">
        <div className="text-sm text-gray-600">Initializing...</div>
      </main>
    );
  }

  return <AppReady controllers={controllers} />;
}

function useLoadedProject(projectManager: ProjectManager) {
  const [loaded, setLoaded] = useState<LoadedProjectState>();
  useEffect(() => {
    return projectManager.subscribe((newProject) => {
      setLoaded(newProject);
    });
  }, [projectManager]);
  return loaded;
}

function AppReady({ controllers }: { controllers: AppControllers }) {
  const { projectManager, appWindows, store } = controllers;
  const loaded = useLoadedProject(projectManager);
  const project = loaded?.project;
  const [isCompact, setIsCompact] = useState(project?.workState !== "planning");

  const handleStateChange = async (newState: ProjectState["workState"]) => {
    let shouldCollapse = false;
    let shouldExpand = false;
    await projectManager.updateProject((draft) => {
      if (draft.workState === newState) return false;
      if (draft.workState === "planning") {
        shouldCollapse = true;
        setIsCompact(true);
      } else if (newState === "planning") {
        shouldExpand = true;
        setIsCompact(false);
      }
      draft.workState = newState;
      const startedAt = Date.now();
      draft.stateTransitions = {
        startedAt,
        endsAt:
          newState === "working"
            ? startedAt + draft.pomodoroSettings.workDuration * 60 * 1000
            : newState === "break"
              ? startedAt + draft.pomodoroSettings.breakDuration * 60 * 1000
              : undefined,
      };
    });

    // Update window format based on state
    if (shouldCollapse) {
      await appWindows.collapseToTracker();
    } else if (shouldExpand) {
      await appWindows.expandToPlanner();
    }
  };

  const handleTimeAdjust = async (ms: number) => {
    await projectManager.updateProject((draft) => {
      draft.stateTransitions.endsAt = Date.now() + ms;
      console.info("Adjusted time to", ms, { applyTo: new Date(draft.stateTransitions.endsAt) });
    });
  };

  // If no project is loaded, show the choose project UI
  if (!project) {
    return <AppNoProject onOpenProject={() => projectManager.openProject()} />;
  }

  const endTime =
    project.stateTransitions.endsAt ??
    (project.workState === "working"
      ? project.stateTransitions.startedAt + project.pomodoroSettings.workDuration * 60 * 1000
      : project.stateTransitions.startedAt + project.pomodoroSettings.breakDuration * 60 * 1000);

  const commonProps = {
    project,
    loaded,
    endTime,
    onStateChange: handleStateChange,
    onTimeAdjust: handleTimeAdjust,
    onOpenProject: () => projectManager.openProject(),
  };

  return isCompact ? <AppCompact {...commonProps} /> : <AppPlanner {...commonProps} />;
}

interface AppViewProps {
  project: ProjectState;
  loaded: LoadedProjectState | undefined;
  endTime: number;
  onStateChange: (newState: ProjectState["workState"]) => void;
  onTimeAdjust: (ms: number) => void;
  onOpenProject: () => void;
}

function useCurrentTask(project: ProjectState) {
  const [currentTask, setCurrentTask] = useState<ProjectMarkdown & { type: "task" }>();
  useEffect(() => {
    const task = project.markdown.find(
      (m): m is ProjectMarkdown & { type: "task" } => m.type === "task" && !m.complete,
    );
    setCurrentTask(task);
  }, [project]);
  return currentTask;
}

function AppCompact({ project, loaded, endTime, onStateChange, onTimeAdjust, onOpenProject }: AppViewProps) {
  const currentTask = useCurrentTask(project);
  const workingOnTask = project.workState === "working" && currentTask != null
  const colors = workingOnTask ? "bg-amber-100 border-amber-300 text-blue-900" : "bg-slate-50 border-blue-400 text-slate-900"
  return (
    <main data-tauri-drag-region className={`h-screen flex items-center px-2 border-4 ${colors}`}>
      <div data-tauri-drag-region className="flex-1 flex items-center gap-4">
        <div data-tauri-drag-region className="ml-auto flex items-center gap-2">
          {loaded?.fullPath && (
            <button
              onClick={() => loaded.fullPath && openPath(loaded.fullPath)}
              className="text-xs p-1.5 text-gray-600 hover:bg-gray-200 rounded flex-1"
              title="Open project file in default application"
            >
              <IconEdit size={16} />
            </button>
          )}
        </div>
        <div className="text-lg tracking-wide font-medium truncate flex-grow" data-tauri-drag-region>
          {currentTask?.name || "No task selected"}
        </div>
        {currentTask?.details?.trim() && <div className="absolute top-full left-0 right-0">{currentTask?.details}</div>}
        <Timer
          startTime={project.stateTransitions.startedAt}
          endTime={endTime}
          className="text-sm font-mono shrink-0"
          onAdjustTime={onTimeAdjust}
        />
        <StateControls project={project} onStateChange={onStateChange} compact />
      </div>
    </main>
  );
}

function AppPlanner({ project, loaded, endTime, onStateChange, onTimeAdjust, onOpenProject }: AppViewProps) {
  return (
    <main className="h-screen flex flex-col bg-white">
      <header data-tauri-drag-region className="flex items-center justify-between border-b px-4 py-2 bg-gray-50">
        <div data-tauri-drag-region className="flex items-center gap-2 select-none">
          <span className="text-sm font-medium pointer-events-none">
            {project.workState === "planning" && "Planning"}
            {project.workState === "working" && "Working"}
            {project.workState === "break" && "Break"}
          </span>
          {project.workState !== "planning" && (
            <Timer
              startTime={project.stateTransitions.startedAt}
              endTime={endTime}
              className="text-sm font-mono"
              onAdjustTime={onTimeAdjust}
            />
          )}
        </div>
        <div className="flex items-center gap-2">
          {loaded && (
            <button
              onClick={() => loaded.fullPath && openPath(loaded.fullPath)}
              className="text-xs px-2 py-1 text-gray-600 hover:bg-gray-200 rounded"
              title="Open project file in default application"
              children="Edit"
            />
          )}
          <button
            onClick={onOpenProject}
            className="text-xs px-2 py-1 text-gray-600 hover:bg-gray-200 rounded"
            children={loaded?.fullPath.split("/").slice(-1) || "Open project..."}
          />
        </div>
      </header>

      <div className="flex-1 overflow-auto p-4">
        <pre className="text-xs font-mono bg-gray-50 p-2 rounded">{JSON.stringify(project, null, 2)}</pre>
      </div>

      <footer className="border-t px-4 py-2 bg-gray-50">
        <div className="flex justify-center">
          <StateControls project={project} onStateChange={onStateChange} />
        </div>
      </footer>
    </main>
  );
}

function AppNoProject({ onOpenProject }: { onOpenProject: () => void }) {
  return (
    <main className="h-screen flex flex-col items-center justify-center bg-gray-50">
      <h1 className="text-lg font-semibold text-gray-800 mb-3">Welcome to Right Now</h1>
      <p className="text-sm text-gray-600 mb-4">Choose a project file to begin</p>
      <button
        onClick={onOpenProject}
        className="px-4 py-2 bg-blue-600 text-white text-sm rounded hover:bg-blue-700 transition-colors"
      >
        Open Project
      </button>
    </main>
  );
}

export default AppOuter;
