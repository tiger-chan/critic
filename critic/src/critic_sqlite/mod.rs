use rusqlite::params;

use crate::{
    dto::{self, MatchResult, NewCategoryItem},
    CriticData, DbConnection, DbError, Record,
};

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
                time DATETIME DEFAULT CURRENT_TIMESTAMP,
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

impl CriticData for Connection {
    fn next_contest(&self) -> Result<dto::Contest, DbError> {
        let mut stmt = self
            .prepare(
                "
WITH 
random_category AS (
    SELECT id AS category_id 
    FROM categories
    ORDER BY RANDOM()
    LIMIT 1
),
unevaluated_pairs AS (
    SELECT 
        ec1.entry_id AS entry1_id,
        ec2.entry_id AS entry2_id,
        ABS(e1.elo - e2.elo) AS elo_distance,
        rc.category_id
    FROM entry_categories ec1
    JOIN entry_categories ec2 
        ON ec1.category_id = ec2.category_id AND ec1.entry_id < ec2.entry_id
    JOIN entries e1 ON ec1.entry_id = e1.id
    JOIN entries e2 ON ec2.entry_id = e2.id
    CROSS JOIN random_category rc
    LEFT JOIN match_history mh ON 
        ((mh.winner_id = ec1.entry_id AND mh.loser_id = ec2.entry_id) OR 
        (mh.winner_id = ec2.entry_id AND mh.loser_id = ec1.entry_id)) 
        AND mh.category_id = rc.category_id
    WHERE ec1.category_id = rc.category_id AND mh.id IS NULL
),
next_comparison AS (
    SELECT 
        entry1_id, 
        entry2_id, 
        category_id
    FROM unevaluated_pairs
    ORDER BY elo_distance ASC
    LIMIT 1
)
SELECT 
    e1.id, e1.name, e1.elo,
    e2.id, e2.name, e2.elo,
    c.id AS category_id,
    c.value AS category_name
FROM next_comparison nc
JOIN entries e1 ON nc.entry1_id = e1.id
JOIN entries e2 ON nc.entry2_id = e2.id
JOIN categories c ON nc.category_id = c.id;
    ",
            )
            .expect("Failed to prepare statement");

        stmt.query_row(params![], |r| {
            let a_id: i32 = r.get(0)?;
            let a_name: String = r.get(1)?;
            let a_elo: i32 = r.get(2)?;

            let b_id: i32 = r.get(3)?;
            let b_name: String = r.get(4)?;
            let b_elo: i32 = r.get(5)?;

            let cat_id: i32 = r.get(6)?;
            let cat_name: String = r.get(7)?;
            Ok(dto::Contest {
                a: dto::Contestant {
                    id: a_id,
                    name: a_name,
                    elo: a_elo,
                },
                b: dto::Contestant {
                    id: b_id,
                    name: b_name,
                    elo: b_elo,
                },
                category: dto::Category {
                    id: cat_id,
                    name: cat_name,
                },
            })
        })
        .map_err(DbError::Sqlite)
    }
}

impl Record<Connection> for NewCategoryItem {
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

impl Record<Connection> for MatchResult {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let tx = connection
            .transaction()
            .expect("Save transaction could not be started");

        {
            let mut ins_stmt = tx
                .prepare(
                    "
                    INSERT INTO match_history (category_id, winner_id, loser_id, elo_change)
                    VALUES (?1, ?2, ?3, ?4)",
                )
                .expect("Failed to prepare statement");

            ins_stmt
                .execute(params![
                    self.category,
                    self.winner,
                    self.loser,
                    self.elo_change
                ])
                .map_err(|x| DbError::Sqlite(x))?;
        }

        tx.commit().map_err(DbError::Sqlite).map(|_| 1)
    }
}
