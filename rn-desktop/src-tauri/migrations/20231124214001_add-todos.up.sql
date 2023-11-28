-- Initial Migration for Todo Management App
CREATE TABLE IF NOT EXISTS Todo (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    time_estimate INTEGER NOT NULL,
    completed_at INTEGER,
    meta_json TEXT NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS WorkSession (
    id TEXT PRIMARY KEY NOT NULL,
    started_at INTEGER NOT NULL,
    ended_at INTEGER,
    active_todo_id TEXT,
    active_todo_started_at INTEGER,
    FOREIGN KEY (active_todo_id) REFERENCES Todo(id)
) STRICT;
CREATE TABLE IF NOT EXISTS TodoTimeEntry (
    id TEXT PRIMARY KEY NOT NULL,
    for_todo_id TEXT NOT NULL,
    during_work_session_id TEXT NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    FOREIGN KEY (for_todo_id) REFERENCES Todo(id),
    FOREIGN KEY (during_work_session_id) REFERENCES WorkSession(id)
) STRICT;