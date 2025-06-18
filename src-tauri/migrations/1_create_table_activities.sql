CREATE TABLE activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    points_per_second INTEGER NOT NULL CHECK (points_per_second BETWEEN -10 AND 10),
    deleted BOOLEAN NOT NULL DEFAULT 0
);