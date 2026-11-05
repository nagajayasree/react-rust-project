CREATE TABLE IF NOT EXISTS applications (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    company     TEXT NOT NULL,
    role        TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'Applied',
    job_url     TEXT,
    notes       TEXT,
    applied_on  TEXT NOT NULL DEFAULT (date('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);