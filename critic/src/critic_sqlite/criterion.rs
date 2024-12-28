use rusqlite::{params, Connection};

use crate::{
    dto::{NewCriterion, UpdateCriterion},
    DbError, Record,
};

use super::procedures;

impl Record<Connection> for UpdateCriterion {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let mut stmt = connection
            .prepare(procedures::UPDATE_CRITERION)
            .expect("Failed to prepare statement");

        stmt.execute(params![self.id, self.name])
            .map_err(DbError::Sqlite)
    }
}

impl Record<Connection> for NewCriterion {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let mut stmt = connection
            .prepare(procedures::CREATE_CRITERION)
            .expect("Failed to prepare statement");

        stmt.execute(params![self.group, self.name])
            .map_err(DbError::Sqlite)
    }
}
