import type { ProjectState } from "../lib/project";
import { IconPlayerPlay, IconSquare, IconCoffee } from "@tabler/icons-react";

interface StateControlsProps {
  project: ProjectState;
  onStateChange: (newState: ProjectState["workState"]) => void;
  /** If true, shows only icons without text */
  compact?: boolean;
}

export function StateControls({ project, onStateChange, compact = false }: StateControlsProps) {
  const isWorking = project.workState === "working";
  const isBreak = project.workState === "break";

  const buttonClass = compact
    ? "p-1.5 rounded text-white hover:opacity-90 transition-opacity"
    : "px-3 py-1.5 rounded text-sm text-white hover:opacity-90 transition-opacity flex items-center gap-2";

  if (project.workState === "planning") {
    return (
      <button onClick={() => onStateChange("working")} className={`${buttonClass} bg-green-600`} title="Start Working">
        <IconPlayerPlay size={16} />
        {!compact && "Start Working"}
      </button>
    );
  }

  return (
    <div className="flex gap-2 items-center">
      <button onClick={() => onStateChange("planning")} className={`${buttonClass} bg-gray-600`} title="End Session">
        <IconSquare size={16} />
        {!compact && "End Session"}
      </button>

      {isWorking && (
        <button onClick={() => onStateChange("break")} className={`${buttonClass} bg-blue-600`} title="Take Break">
          <IconCoffee size={16} />
          {!compact && "Take Break"}
        </button>
      )}

      {isBreak && (
        <button
          onClick={() => onStateChange("working")}
          className={`${buttonClass} bg-green-600`}
          title="Start Working"
        >
          <IconPlayerPlay size={16} />
          {!compact && "Start Working"}
        </button>
      )}
    </div>
  );
}
