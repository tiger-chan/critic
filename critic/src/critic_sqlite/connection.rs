use crate::{DbConnection, DbError, Record};

use rusqlite::params;
pub use rusqlite::Connection;

use super::{
    super::seed::{DEFAULT_CRITERIA, ENTRIES},
    procedures,
};

impl DbConnection for Connection {
    fn open_category<T: AsRef<std::path::Path>>(path: T) -> Result<Self, DbError> {
        let conn = Connection::open(path.as_ref()).map_err(DbError::Sqlite)?;
        conn.execute("PRAGMA foreign_keys=ON;", params![])
            .map_err(DbError::Sqlite)?;

        let mut conn = conn
            .execute_batch(procedures::CREATE)
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
                        .prepare(procedures::CREATE_TITLE)
                        .expect("Failed to prepare statement");

                    let mut ins_group_criteria_stmt = tx
                        .prepare(procedures::CREATE_GROUP_CRITERIA)
                        .expect("Failed to prepare statement");

                    let mut ins_criteria_stmt = tx
                        .prepare(procedures::CREATE_CRITERION)
                        .expect("Failed to prepare statement");

                    let mut ins_sub_stmt = tx
                        .prepare(procedures::CREATE_TITLE_CRITERIA)
                        .expect("Failed to prepare statement");

                    for group in DEFAULT_CRITERIA {
                        ins_group_criteria_stmt
                            .execute(params![group.name])
                            .map_err(DbError::Sqlite)?;

                        let id = tx.last_insert_rowid();

                        for criterion in group.sub_criteria {
                            ins_criteria_stmt
                                .execute(params![id, criterion])
                                .map_err(DbError::Sqlite)?;
                        }
                    }

                    for entry in ENTRIES {
                        ins_stmt
                            .execute(params![entry.name])
                            .map_err(DbError::Sqlite)?;

                        ins_sub_stmt
                            .execute(params![entry.name, "General"])
                            .map_err(DbError::Sqlite)?;

                        for group in entry.groups {
                            ins_sub_stmt
                                .execute(params![entry.name, group])
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
