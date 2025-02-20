import * as Ta from "@tabler/icons-react";
import type { ProjectMarkdown } from "../lib/ProjectStateEditor";

interface TaskListProps {
  tasks: ProjectMarkdown[];
  onCompleteTask: (task: ProjectMarkdown & { type: "task" }) => void;
}

export function TaskList({ tasks, onCompleteTask }: TaskListProps) {
  // Group tasks under their most recent heading
  const sections: { heading?: ProjectMarkdown & { type: "heading" }; items: ProjectMarkdown[] }[] = [];
  let currentSection: typeof sections[0] = { items: [] };

  for (const item of tasks) {
    if (item.type === "heading") {
      if (currentSection.items.length > 0) {
        sections.push(currentSection);
      }
      currentSection = { heading: item, items: [] };
    } else {
      currentSection.items.push(item);
    }
  }
  // Add the last section
  if (currentSection.items.length > 0) {
    sections.push(currentSection);
  }

  return (
    <div className="space-y-6">
      {sections.map((section, i) => (
        <div key={i} className="space-y-2">
          {section.heading && (
            <h2 
              className={`font-medium ${
                section.heading.level === 1 ? "text-xl" : "text-lg"
              } pb-2 border-b border-gray-200`}
            >
              {section.heading.text}
            </h2>
          )}
          <div className="space-y-2 pl-1">
            {section.items.map((item, j) => {
              if (item.type === "task") {
                return (
                  <div key={j} className="flex items-start gap-2 group">
                    <button
                      onClick={() => onCompleteTask(item)}
                      className={`p-1 rounded hover:bg-gray-100 ${
                        item.complete ? "text-green-600" : "text-gray-400"
                      }`}
                      title={item.complete ? "Mark incomplete" : "Mark complete"}
                    >
                      {item.complete ? (
                        <Ta.IconCheckbox size={16} />
                      ) : (
                        <Ta.IconSquare size={16} />
                      )}
                    </button>
                    <div className="flex-1">
                      <div className={item.complete ? "line-through text-gray-500" : ""}>
                        {item.name}
                      </div>
                      {item.details && (
                        <div className="text-sm text-gray-600 mt-1">{item.details}</div>
                      )}
                    </div>
                  </div>
                );
              }
              return null;
            })}
          </div>
        </div>
      ))}
    </div>
  );
} 