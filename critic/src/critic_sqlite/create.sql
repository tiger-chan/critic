BEGIN;
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY NOT NULL,
    name STRING NOT NULL,
    elo INTEGER DEFAULT 1000 NOT NULL,
    UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS criteria (
    id INTEGER PRIMARY KEY NOT NULL,
    value STRING NOT NULL,
    UNIQUE(value)
);

CREATE TABLE IF NOT EXISTS entry_criteria (
    id INTEGER PRIMARY KEY NOT NULL,
    entry_id INTEGER NOT NULL,
    criterion_id INTEGER NOT NULL,
    FOREIGN KEY (entry_id) REFERENCES entries(id)
    FOREIGN KEY (criterion_id) REFERENCES criteria(id)
    UNIQUE(entry_id, criterion_id)
);

CREATE TABLE IF NOT EXISTS match_history (
    id INTEGER PRIMARY KEY NOT NULL,
    criterion_id INTEGER NOT NULL,
    winner_id INTEGER NOT NULL,
    loser_id INTEGER NOT NULL,
    elo_adj INTEGER NOT NULL,
    time DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (criterion_id) REFERENCES criteria(id),
    FOREIGN KEY (winner_id) REFERENCES entries(id),
    FOREIGN KEY (loser_id) REFERENCES entries(id)
);
COMMIT;
