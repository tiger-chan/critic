use rusqlite::params;

use crate::{CategoryItem, DbConnection, DbError, Record};

pub use rusqlite::Connection;

impl DbConnection for Connection {
    fn open_category<T: AsRef<std::path::Path>>(path: T) -> Result<Self, DbError> {
        let conn = Connection::open(path.as_ref()).map_err(DbError::Sqlite)?;

        let init = conn
            .execute_batch(
                "
            BEGIN;
            CREATE TABLE IF NOT EXISTS entries (
                id INTEGER PRIMARY KEY NOT NULL,
                name STRING NOT NULL,
                elo INTEGER DEFAULT 1000 NOT NULL,
                UNIQUE(name)
            );

            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY NOT NULL,
                value STRING NOT NULL,
                UNIQUE(value)
            );

            CREATE TABLE IF NOT EXISTS entry_categories (
                id INTEGER PRIMARY KEY NOT NULL,
                entry_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                FOREIGN KEY (entry_id) REFERENCES entries(id)
                FOREIGN KEY (category_id) REFERENCES categories(id)
                UNIQUE(entry_id, category_id)
            );

            CREATE TABLE IF NOT EXISTS match_history (
                id INTEGER PRIMARY KEY NOT NULL,
                category_id INTEGER NOT NULL,
                winner_id INTEGER NOT NULL,
                loser_id INTEGER NOT NULL,
                elo_adj INTEGER NOT NULL,
                FOREIGN KEY (category_id) REFERENCES categories(id),
                FOREIGN KEY (winner_id) REFERENCES entries(id),
                FOREIGN KEY (loser_id) REFERENCES entries(id)
            );
            COMMIT;",
            )
            .map_err(DbError::Sqlite);

        init.map(|_| conn)
    }

    fn save<T: Record<Self>>(&mut self, record: &T) -> Result<usize, DbError> {
        record.save(self)
    }
}

impl Record<Connection> for CategoryItem {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let tx = connection
            .transaction()
            .expect("Save transaction could not be started");

        {
            let mut ins_stmt = tx
                .prepare(
                    "
                    INSERT INTO entries (name)
                    VALUES (?1) ON CONFLICT DO NOTHING",
                )
                .expect("Failed to prepare statement");

            let mut ins_sc_stmt = tx
                .prepare(
                    "
                    INSERT INTO categories (value)
                    VALUES(?1) ON CONFLICT DO NOTHING;",
                )
                .expect("Failed to prepare statement");
            let mut ins_sub_stmt = tx
                .prepare(
                    "
                    INSERT INTO entry_categories (entry_id, category_id)
                    VALUES (
                        (SELECT id FROM entries WHERE name = ?1),
                        (SELECT id FROM categories WHERE value = ?2)
                    ) ON CONFLICT DO NOTHING",
                )
                .expect("Failed to prepare statement");

            ins_stmt
                .execute(params![self.name])
                .map_err(|x| DbError::Sqlite(x))?;

            for sc in self.sub_categories.iter().filter(|x| !x.is_empty()) {
                ins_sc_stmt.execute(params![sc]).map_err(DbError::Sqlite)?;

                ins_sub_stmt
                    .execute(params![self.name, sc])
                    .map_err(DbError::Sqlite)?;
            }
        }

        tx.commit().map_err(DbError::Sqlite).map(|_| 1)
    }
}
