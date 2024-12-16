use rusqlite::params;

use crate::{
    dto::{self, MatchResult, NewCategoryItem},
    CriticData, DbConnection, DbError, Record,
};

pub use rusqlite::Connection;

use super::seed::{DEFAULT_CRITERIA, ENTRIES};

impl DbConnection for Connection {
    fn open_category<T: AsRef<std::path::Path>>(path: T) -> Result<Self, DbError> {
        let conn = Connection::open(path.as_ref()).map_err(DbError::Sqlite)?;

        let mut conn = conn
            .execute_batch(include_str!("create.sql"))
            .map_err(DbError::Sqlite)
            .map(|_| conn)?;

        let count = conn
            .query_row("SELECT COUNT(1) FROM entries", params![], |x| {
                let y: i32 = x.get(0)?;
                Ok(y)
            })
            .map_err(DbError::Sqlite);

        if let Ok(count) = count {
            if count == 0 {
                let tx = conn
                    .transaction()
                    .expect("Save transaction could not be started");

                {
                    let mut ins_stmt = tx
                        .prepare(include_str!("ins_entry.sql"))
                        .expect("Failed to prepare statement");

                    let mut ins_criteria_stmt = tx
                        .prepare(include_str!("ins_criterion.sql"))
                        .expect("Failed to prepare statement");

                    let mut ins_default_stmt = tx
                        .prepare(include_str!("ins_entry_criterion_default.sql"))
                        .expect("Failed to prepare statement");

                    let mut ins_sub_stmt = tx
                        .prepare(include_str!("ins_entry_criterion.sql"))
                        .expect("Failed to prepare statement");

                    for criterion in DEFAULT_CRITERIA {
                        ins_criteria_stmt
                            .execute(params![criterion])
                            .map_err(DbError::Sqlite)?;
                    }

                    for entry in ENTRIES {
                        ins_stmt
                            .execute(params![entry.name])
                            .map_err(DbError::Sqlite)?;

                        ins_default_stmt
                            .execute(params![entry.name])
                            .map_err(DbError::Sqlite)?;

                        for sc in entry.criteria {
                            ins_criteria_stmt
                                .execute(params![sc])
                                .map_err(DbError::Sqlite)?;

                            ins_sub_stmt
                                .execute(params![entry.name, sc])
                                .map_err(DbError::Sqlite)?;
                        }
                    }
                }
                tx.commit().map_err(DbError::Sqlite)?;
            }
        }

        Ok(conn)
    }

    fn save<T: Record<Self>>(&mut self, record: &T) -> Result<usize, DbError> {
        record.save(self)
    }
}

impl CriticData for Connection {
    fn next_contest(&self) -> Result<dto::Contest, DbError> {
        let mut stmt = self
            .prepare(include_str!("next_contest.sql"))
            .expect("Failed to prepare statement");

        stmt.query_row(params![], |r| {
            let a_id: i32 = r.get(0)?;
            let a_name: String = r.get(1)?;
            let a_elo: f32 = r.get(2)?;

            let b_id: i32 = r.get(3)?;
            let b_name: String = r.get(4)?;
            let b_elo: f32 = r.get(5)?;

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
                .prepare(include_str!("ins_entry.sql"))
                .expect("Failed to prepare statement");

            let mut ins_sc_stmt = tx
                .prepare(include_str!("ins_criterion.sql"))
                .expect("Failed to prepare statement");

            let mut ins_default_stmt = tx
                .prepare(include_str!("ins_entry_criterion_default.sql"))
                .expect("Failed to prepare statement");

            let mut ins_sub_stmt = tx
                .prepare(include_str!("ins_entry_criterion.sql"))
                .expect("Failed to prepare statement");

            ins_stmt
                .execute(params![self.name])
                .map_err(DbError::Sqlite)?;

            ins_default_stmt
                .execute(params![self.name])
                .map_err(DbError::Sqlite)?;

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
                .prepare(include_str!("add_contest_result.sql"))
                .expect("Failed to prepare statement");

            let mut update_stmt = tx
                .prepare(include_str!("update_elo.sql"))
                .expect("Failed to prepare statement");

            ins_stmt
                .execute(params![
                    self.criterion,
                    self.a,
                    self.b,
                    self.score,
                    self.elo_change.0,
                    self.elo_change.1,
                ])
                .map_err(DbError::Sqlite)?;

            update_stmt
                .execute(params![self.a, self.elo_change.0])
                .map_err(DbError::Sqlite)?;

            update_stmt
                .execute(params![self.b, self.elo_change.1])
                .map_err(DbError::Sqlite)?;
        }

        tx.commit().map_err(DbError::Sqlite).map(|_| 1)
    }
}
