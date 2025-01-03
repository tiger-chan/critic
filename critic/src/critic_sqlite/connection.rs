use crate::{DbConnection, DbError, Record};

use rusqlite::params;
pub use rusqlite::Connection;

use super::procedures;

impl DbConnection for Connection {
    fn open_category<T: AsRef<std::path::Path>>(path: T) -> Result<Self, DbError> {
        let conn = Connection::open(path.as_ref()).map_err(DbError::Sqlite)?;
        conn.execute("PRAGMA foreign_keys=ON;", params![])
            .map_err(DbError::Sqlite)?;

        let conn = conn
            .execute_batch(procedures::CREATE)
            .map_err(DbError::Sqlite)
            .map(|_| conn)?;

        Ok(conn)
    }

    fn save<T: Record<Self>>(&mut self, record: &T) -> Result<usize, DbError> {
        record.save(self)
    }
}
