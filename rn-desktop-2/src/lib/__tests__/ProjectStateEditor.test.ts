import { describe, it, expect } from "bun:test";
import { ProjectStateEditor } from "../ProjectStateEditor";
// Example parse and update from prior code snippet
// We also assume you have the same `ProjectMarkdown` type, etc.

describe("ProjectStateEditor", () => {
  const minimalFrontmatter = `---
current_state: "planning"
current_state_changed_at: "2025-01-01T10:00:00Z"
pomodoro_settings:
  work_duration: 25
  break_duration: 5
---
# Unrelated heading
- [ ] Some Task
- [ ] Another Task
`;

  const noFrontmatter = `# Just a heading
- [ ] Some Task
Task related text here
`;

  const complexBody = `---
current_state: working
current_state_changed_at: '2025-02-15T12:34:56.000Z'
pomodoro_settings:
  work_duration: 50
  break_duration: 10
---
# Main Heading

- [ ] First Task
  This line belongs to first task
  This line also belongs to first task

Some unrecognized text in between

- [ ] Second Task
  Some details under second task

## Sub Heading
Arbitrary text that is unrecognized

- [ ] Third Task
`;

  it("should parse minimal frontmatter and body correctly", () => {
    const parsed = ProjectStateEditor.parse(minimalFrontmatter);

    expect(parsed.workState).toBe("planning");
    expect(parsed.stateTransitions.startedAt).toBe(new Date("2025-01-01T10:00:00Z").getTime());
    expect(parsed.pomodoroSettings.workDuration).toBe(25);
    expect(parsed.pomodoroSettings.breakDuration).toBe(5);

    // Body checks
    expect(parsed.markdown.length).toBeGreaterThanOrEqual(2);
    // Should have a heading block for "# Unrelated heading"
    // and 2 task blocks, or else unrecognized lines
    const headingBlock = parsed.markdown.find((b) => b.type === "heading");
    expect(headingBlock).toBeTruthy();
    expect((headingBlock as any).text).toBe("Unrelated heading");

    const taskBlocks = parsed.markdown.filter((b) => b.type === "task");
    expect(taskBlocks.length).toBe(2);
    expect((taskBlocks[0] as any).name).toBe("Some Task");
    expect((taskBlocks[1] as any).name).toBe("Another Task");
  });

  it("should handle files with no frontmatter", () => {
    const parsed = ProjectStateEditor.parse(noFrontmatter);

    // Confirm defaults for missing frontmatter
    expect(parsed.workState).toBe("planning");
    // we set startedAt to Date.now() if absent, but let's just confirm it's a number
    expect(typeof parsed.stateTransitions.startedAt).toBe("number");
    expect(parsed.pomodoroSettings.workDuration).toBe(25);
    expect(parsed.pomodoroSettings.breakDuration).toBe(5);
    // Body should parse one heading, one task, and an unrecognized block
    expect(parsed.markdown).toMatchInlineSnapshot(`
      [
        {
          "level": 1,
          "text": "Just a heading",
          "type": "heading",
        },
        {
          "details": 
      "Task related text here
      "
      ,
          "name": "Some Task",
          "type": "task",
        },
      ]
    `)
  });

  it("should parse and round-trip a complex body with minimal changes", () => {
    const parsed = ProjectStateEditor.parse(complexBody);
    expect(parsed.workState).toBe("working");
    expect(parsed.stateTransitions.startedAt).toBe(new Date("2025-02-15T12:34:56Z").getTime());
    expect(parsed.pomodoroSettings.workDuration).toBe(50);
    expect(parsed.pomodoroSettings.breakDuration).toBe(10);

    // Expect multiple headings, tasks, and unrecognized blocks
    const headings = parsed.markdown.filter((b) => b.type === "heading");
    expect(headings.length).toBe(2);

    const tasks = parsed.markdown.filter((b) => b.type === "task");
    expect(tasks.length).toBe(3);
    // Check details for first task
    const firstTask = tasks[0] as any;
    expect(firstTask.name).toBe("First Task");
    expect(firstTask.details).toContain("This line belongs to first task");

    // Round-trip without changes
    const roundTripped = ProjectStateEditor.update(complexBody, parsed);
    // Because we didn't modify anything, it should remain identical
    // If there's a minor detail like a sorted order in the frontmatter keys,
    // you might relax this to .toContain(...) checks.
    expect(roundTripped).toBe(complexBody);
  });

  it("should update frontmatter fields and preserve body formatting", () => {
    const parsed = ProjectStateEditor.parse(complexBody);
    parsed.workState = "break";
    parsed.stateTransitions.startedAt = new Date("2026-01-01T00:00:00Z").getTime();
    parsed.pomodoroSettings.workDuration = 45;

    const updated = ProjectStateEditor.update(complexBody, parsed);

    // Confirm frontmatter is updated
    expect(updated).toContain("current_state: break");
    expect(updated).toContain("current_state_changed_at: '2026-01-01T00:00:00.000Z'");
    // The break_duration from the original was 10, not changed
    expect(updated).toContain("break_duration: 10");
    // The new work duration is 45
    expect(updated).toContain("work_duration: 45");

    // Confirm the body remains structurally the same
    expect(updated).toContain("# Main Heading");
    expect(updated).toContain("## Sub Heading");
    // The tasks should remain the same
    expect(updated).toContain("- [ ] First Task");
    expect(updated).toContain("- [ ] Second Task");
    expect(updated).toContain("- [ ] Third Task");
  });

  it("should preserve unrecognized text and spacing in body updates", () => {
    // Slight variation with odd spacing
    const spacedFile = `---
current_state: "planning"
current_state_changed_at: "2025-02-15T12:34:56Z"
pomodoro_settings:
  work_duration: 25
  break_duration: 5
---

# Heading 1


- [ ] Task A

Some random text


- [ ] Task B
  Detail line 1
  Detail line 2

## Heading 2

More text
`;
    const parsed = ProjectStateEditor.parse(spacedFile);

    // Let's do a tiny update: change from "planning" to "working"
    parsed.workState = "working";
    const updated = ProjectStateEditor.update(spacedFile, parsed);

    // Confirm that the frontmatter is updated
    expect(updated).toContain("current_state: working");

    // Check that new lines and spacing remain in the final doc
    // For instance, we had 2 newlines after "# Heading 1"
    const lines = updated.split("\n");
    // We can verify a sequence or count how many blank lines appear
    // after "# Heading 1". We'll do a minimal check here:
    const headingIndex = lines.indexOf("# Heading 1");
    expect(lines[headingIndex + 1].trim()).toBe("");
    expect(lines[headingIndex + 2].trim()).toBe("");

    // Confirm the tasks remain intact
    expect(updated).toContain("- [ ] Task A");
    expect(updated).toContain("- [ ] Task B");
    expect(updated).toContain("Detail line 1");
    expect(updated).toContain("Detail line 2");

    // Confirm the second heading and the extra text remain
    expect(updated).toContain("## Heading 2");
    expect(updated).toContain("More text");
  });

  it.skip("should handle user changes to tasks between parse and update", () => {
    // Let's say the user externally changes a detail line
    const original = `---
current_state: "planning"
current_state_changed_at: "2025-03-01T08:00:00Z"
pomodoro_settings:
  work_duration: 25
  break_duration: 5
---
- [ ] Old Task
  Original detail
`;
    // parse the original
    const parsed = ProjectStateEditor.parse(original);

    // Meanwhile, user edits the file externally, changing "Original detail" to "User changed detail"
    const externallyModified = `---
current_state: "planning"
current_state_changed_at: "2025-03-01T08:00:00Z"
pomodoro_settings:
  work_duration: 25
  break_duration: 5
---
- [ ] Old Task
  User changed detail
`;

    // Now we attempt to update with new frontmatter changes, e.g. user clicks "Start working"
    parsed.workState = "working";
    parsed.stateTransitions.startedAt = new Date("2025-03-01T09:00:00Z").getTime();

    // We call update, but pass in the externallyModified content
    const updated = ProjectStateEditor.update(externallyModified, parsed);

    // Confirm we see updated frontmatter
    expect(updated).toContain("current_state: working");
    expect(updated).toContain("current_state_changed_at: '2025-03-01T09:00:00.000Z'");

    // Confirm the user's external change to the detail line is still there
    expect(updated).toContain("User changed detail");
  });

  it("should parse tasks that have no details (empty next line) correctly", () => {
    const contentWithEmptyDetails = `---
current_state: "planning"
current_state_changed_at: "2025-03-02T09:00:00Z"
pomodoro_settings:
  work_duration: 25
  break_duration: 5
---
- [ ] Task 1

- [ ] Task 2
- [ ] Task 3
  Has detail
`;
    const parsed = ProjectStateEditor.parse(contentWithEmptyDetails);

    // We expect 3 tasks
    const tasks = parsed.markdown.filter((m) => m.type === "task");
    expect(tasks).toMatchInlineSnapshot(`
      [
        {
          "details": "",
          "name": "Task 1",
          "type": "task",
        },
        {
          "details": null,
          "name": "Task 2",
          "type": "task",
        },
        {
          "details": 
      "  Has detail
      "
      ,
          "name": "Task 3",
          "type": "task",
        },
      ]
    `)
  });

  it("should not break if frontmatter is invalid or partially corrupted", () => {
    const corrupted = `---
current_state: "working
pomodoro_settings
  work_duration: 25
---
- [ ] Task
`;

    // We expect parse to fallback to defaults gracefully
    const parsed = ProjectStateEditor.parse(corrupted);
    expect(parsed.workState).toBe("planning"); // fallback
    expect(parsed.pomodoroSettings.workDuration).toBe(25); // from partial parse or fallback
    // Body should still parse
    expect(parsed.markdown[0].type).toBe("task");
  });

  // More tests to consider: concurrency, partial merges, etc.
});
