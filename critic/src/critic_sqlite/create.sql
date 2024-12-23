BEGIN;
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY NOT NULL,
    name STRING NOT NULL,
    UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS criteria_group (
    id INTEGER PRIMARY KEY NOT NULL,
    value STRING NOT NULL,
    UNIQUE(value)
);

CREATE TABLE IF NOT EXISTS criteria (
    id INTEGER PRIMARY KEY NOT NULL,
    value STRING NOT NULL,
    group_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES criteria_group(id),
    UNIQUE(value, group_id)
);

CREATE TABLE IF NOT EXISTS entry_criteria (
    id INTEGER PRIMARY KEY NOT NULL,
    elo REAL DEFAULT 1000.0 NOT NULL,
    entry_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    FOREIGN KEY (entry_id) REFERENCES entries(id)
    FOREIGN KEY (group_id) REFERENCES group_criteria(id)
    UNIQUE(entry_id, group_id)
);

CREATE TABLE IF NOT EXISTS match_history (
    id INTEGER PRIMARY KEY NOT NULL,
    criterion_id INTEGER NOT NULL,
    a_id INTEGER NOT NULL,
    b_id INTEGER NOT NULL,
    score REAL NOT NULL,
    elo_adj_a REAL NOT NULL,
    elo_adj_b REAL NOT NULL,
    time DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (criterion_id) REFERENCES criteria(id),
    FOREIGN KEY (a_id) REFERENCES entries(id),
    FOREIGN KEY (b_id) REFERENCES entries(id)
);
COMMIT;
