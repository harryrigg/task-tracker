CREATE TABLE tasks(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,

    started_at TIMESTAMP NOT NULL,
    finished_at TIMESTAMP,

    description VARCHAR NOT NULL,

    project_id INTEGER NOT NULL,

    FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
)