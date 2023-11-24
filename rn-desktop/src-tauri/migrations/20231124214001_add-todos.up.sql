-- Initial Migration for Todo Management App
CREATE TABLE IF NOT EXISTS Todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    time_estimate INTEGER NOT NULL,
    completed_at DATETIME
);
CREATE TABLE IF NOT EXISTS WorkSession (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    started_at DATETIME NOT NULL,
    ended_at DATETIME,
    active_todo_id INTEGER,
    active_todo_started_at DATETIME,
    FOREIGN KEY (active_todo_id) REFERENCES Todo(id)
);
CREATE TABLE IF NOT EXISTS TodoTimeEntry (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    for_todo_id INTEGER NOT NULL,
    during_work_session_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    FOREIGN KEY (for_todo_id) REFERENCES Todo(id),
    FOREIGN KEY (during_work_session_id) REFERENCES WorkSession(id)
);