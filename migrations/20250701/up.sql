CREATE TABLE IF NOT EXISTS server_checks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    response_time_ms INTEGER,
    status TEXT,
    cpu_usage REAL,
    memory_usage REAL
);
