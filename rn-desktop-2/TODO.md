---
current_state: working
current_state_changed_at: "2025-02-20T15:00:10.515Z"
current_state_ends_at: "2025-02-20T15:25:10.515Z"
pomodoro_settings:
  work_duration: 25
  break_duration: 5
---

# Audio Cues

- [ ] Play audio cue when work starts
      See ChatGPT
- [ ] Play audio cues for start, and warning before end
- [ ] Play audio cue when the user manually checks off a TODO item

# UI

- [ ] Simplify first screen & show recent TODO lists to open
- [ ] Resize text to fit the available area
- [ ] How to make a Tauri window appear above fullscreen apps in macOS like Zoom
      Use https://github.com/ahkohd/tauri-nspanel to swizzle the window in Rust
      https://chatgpt.com/share/67b74c68-a140-8009-820d-f1dfa7b3a4b3does
- [ ] Control OS music playback when starting a break
- [ ] Show a flashing state when the timer goes past the allocated time

# Perf

- [ ] Investigate if tabler icons is being included entirely in dev
- [ ] Use jotai atoms to control more of the state

# Misc

- [ ] Add "Edit" menu item for opening the TODO file
- [ ] Set the task bar to the section heading, and current task as a submenu with "complete"
- [ ] Compact mode should stay aligned to bottom coords
- [ ] Render markdown from details in the bottom
- Is it possible to enable some kind of "scroll" to increase the window size?
- [ ] Disable double click to maximize
      You can add an event listener on the relevant element which then calls `startDragging()`. The tauri-drag-region attribute does the same under the hood (but with the addition of the double-click handler).
      See https://v2.tauri.app/learn/window-customization/#manual-implementation-of-data-tauri-drag-region
- [ ] Only set the values in frontmatter if they are different from the default (e.g. for pomodoro_settings)
- [ ] Prevent making window larger and not reverting when there is a scrollbar issue
- [ ] Don't lose the newlines between tasks and headings
- [x] Synchronize "compact" state with the window state (derive some reactive varaible?)

# DONE

- [x] Make it not go smaller when it resets to next todo item
- [x] Fix nested TODO items (create tests for it?)
- [x] Fix focus jank
- [x] Enable completing a task item from the UI
