-- Add up migration script here
CREATE TABLE IF NOT EXISTS window_positions (
  id ROWID AUTOINCREMENT,
  window_name TEXT NOT NULL,
  window_position TEXT NOT NULL
);
