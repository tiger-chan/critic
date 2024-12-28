use rusqlite::{params, Connection};

use crate::{
    dto::{NewCriteriaGroup, UpdateCriteriaGroup},
    DbError, Record,
};

use super::procedures;

impl Record<Connection> for UpdateCriteriaGroup {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let mut stmt = connection
            .prepare(procedures::UPDATE_GROUP_CRITERIA)
            .expect("Failed to prepare statement");

        stmt.execute(params![self.id, self.name])
            .map_err(DbError::Sqlite)
    }
}

impl Record<Connection> for NewCriteriaGroup {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let mut stmt = connection
            .prepare(procedures::CREATE_GROUP_CRITERIA)
            .expect("Failed to prepare statement");

        stmt.execute(params![self.name]).map_err(DbError::Sqlite)?;

        Ok(connection.last_insert_rowid() as usize)
    }
}
