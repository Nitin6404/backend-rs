CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
    id TEXT PRIMARY KEY,
    user_id INTEGER,
    filename TEXT,
    path TEXT,
    uploaded_at TEXT,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
