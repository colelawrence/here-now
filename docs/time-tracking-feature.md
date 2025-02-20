# Time Tracking and State Management

## Overview
Right Now (RN) is a Pomodoro-style task management system that maintains focus through clear state management, minimal UI, and audio feedback. The system tracks work sessions, breaks, and task progress through markdown-based storage with real-time updates.

## Core Features

### State Management
- Three primary states: Planning, Working, Break
  - State transitions stored in frontmatter: `last_transition: ISO8601`
  - Per-project configurable durations in frontmatter:
    ```yaml
    work_duration: 25  # minutes
    break_duration: 5  # minutes
    ```

### Time Tracking
- Backend tracking
  - 10-second increment activity logging
  - Aggregation to minute-level updates
  - Rolling window to handle rapid task switching
- Storage levels
  - Task-specific: `{14m}` syntax in markdown body
  - Session totals: Stored in frontmatter
  - Project totals: Accumulated in frontmatter
    ```yaml
    total_work_minutes: 240
    total_break_minutes: 45
    session_work_minutes: 14
    session_break_minutes: 3
    ```

### UI Modes
- Tracker (Minimal)
  - Single active task display
  - Current timer status
  - State transition controls
- Planner (Expanded)
  - Full task list
  - Drag-and-drop reordering
  - Keyboard shortcuts for task management

## Technical Implementation

### Data Model
1. Markdown Storage
    ```yaml
    ---
    current_state: "working"  # planning | working | break
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
    - [ ] Current active task {20m}
    - [ ] Next task in queue
    ```

See [Markdown-Based Project Files](markdown-based-project-files.md) for complete data model specification and rationale.

2. Runtime State
    - Rolling activity window (10s increments)
    - Active timers and intervals
    - Audio player state

### Security Considerations
- Local file system access only
- No network requirements
- File watching for markdown updates

## User Experience

### State Transitions
1. Planning → Working
   - Trigger: Play button
   - Actions: Start work timer, update frontmatter
   - Audio: Work start sound

2. Working → Break
   - Trigger: Stop button or timer completion
   - Actions: Start break timer, accumulate work minutes
   - Audio: Break notification

3. Break → Planning
   - Trigger: Break completion
   - Actions: Reset session metrics
   - Audio: Break end notification

### Audio Feedback System
- Default sound bundle:
  - `work_start.mp3`
  - `break_approaching.mp3`
  - `break_start.mp3`
  - `break_end.mp3`
- Custom sound bundle support:
  - Named MP3 files in predefined directory
  - Future: Music crossfading between states

## Implementation Phases

### Phase 1: Core Time Tracking
- [ ] Markdown file structure and parsing
- [ ] Basic state management
- [ ] 10-second increment tracking
- [ ] Minute-level aggregation

### Phase 2: UI Implementation
- [ ] Tracker mode
- [ ] Planner mode
- [ ] State transition controls
- [ ] Keyboard shortcuts

### Phase 3: Audio System
- [ ] Default sound integration
- [ ] Custom sound bundle support
- [ ] State transition audio cues

## Future Enhancements
1. Project grouping via markdown headings
2. Music crossfading between states
3. AI-assisted task creation
4. Analytics dashboard
5. Custom notification sounds recording
6. Multiple markdown file switching 