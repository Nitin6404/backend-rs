CREATE TABLE server_checks (
    timestamp TEXT NOT NULL,
    response_time_ms BIGINT,
    status TEXT,
    cpu_usage DOUBLE PRECISION,
    memory_usage DOUBLE PRECISION
);
