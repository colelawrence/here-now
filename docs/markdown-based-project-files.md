# Markdown-Based Project Files

## Overview

Implement a markdown-based storage system for project state and tasks that allows for human-readable persistence while maintaining the real-time capabilities of the app. This system will serve as both the storage mechanism and the interface for manual project configuration.

## Core Features

- Project State Management in Frontmatter

  - Work state (planning/working/break)
  - State transition timestamps for timer accuracy
  - Project-specific Pomodoro settings
  - Session and cumulative time tracking
  - Technical note: Update frontmatter every minute, backed by 10-second precision tracking

- Task Management in Markdown Body

  - Active task always first in list
  - Time tracking syntax: `{14m}` for task-specific time
  - Support for markdown headings as project groups (future)
  - Technical note: Parse tasks as ordered list items with optional time annotations

- Time Tracking System
  - Backend tracking at 10-second intervals
  - Rolling window aggregation to minute-level updates
  - Three-tier time tracking:
    - Per task: Stored inline with task
    - Per session: Frontmatter
    - Project total: Frontmatter
  - Technical note: Maintain in-memory buffer for sub-minute precision

## Technical Implementation

### Data Model

YAML front-matter for a markdown file storing the curent state and tasks.

#### `TODO.md`
```yaml
---
current_state: "working"
current_state_changed_at: "2024-03-14T15:23:45Z"
pomodoro_settings:
  work_duration: 25
  break_duration: 5
session:
  started_at: "2024-03-14T15:00:00Z"
  minutes_worked: 23
  break_minutes: 5
project_total:
  total_minutes: 180
  total_breaks: 15
---
- [ ] Complete markdown integration {14m}
- [ ] Implement time tracking
- [ ] Add sound notifications
```

## User Experience

### User Flow

1. Open app -> Select/create project file
2. View tasks in planner/tracker mode
3. Start work session -> Timer begins
4. Complete task -> Updates persist to markdown
5. Take break -> State changes reflect in file

### Key UI Components

- Project selector (recent files)
- Planner/tracker toggle
- Timer display
- Task list with drag-drop reordering

## Implementation Phases

### Phase 1: Basic Markdown Integration

- Implement markdown parsing/writing
- Basic frontmatter state management
- File selection interface

### Phase 2: Time Tracking

- 10-second precision backend timer
- Time aggregation system
- Markdown updates for time tracking

### Phase 3: Project Management

- Recent projects list
- Project creation workflow
- File watching for external changes

## Future Enhancements

- Project templates
- Markdown heading-based categorization
- External editor sync
- Time tracking analytics
- Music transitions between states
- AI-powered task creation from dictation/text input

## Technical Notes

- Use atomic writes for file updates to prevent corruption
- Implement file watchers for external markdown changes
- Buffer time tracking updates to reduce disk I/O
- Consider using yaml frontmatter parser for robust metadata handling
