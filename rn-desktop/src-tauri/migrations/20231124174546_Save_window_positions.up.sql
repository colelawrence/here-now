-- Add up migration script here
CREATE TABLE IF NOT EXISTS window_positions
(
    window_key          TEXT NOT NULL,
    desktop_arrangement TEXT NOT NULL,
    window_position_x   INT  NOT NULL,
    window_position_y   INT  NOT NULL,
    window_size_w       INT  NOT NULL,
    window_size_h       INT  NOT NULL,
    PRIMARY KEY (window_key, desktop_arrangement)
);
