CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    duration INTEGER NOT NULL,
    points_per_second INTEGER NOT NULL CHECK (points_per_second BETWEEN -10 AND 10),
    activity_id INTEGER NOT NULL,
    FOREIGN KEY (activity_id) REFERENCES activities (id)
);