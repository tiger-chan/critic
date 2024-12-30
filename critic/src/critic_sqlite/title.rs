use rusqlite::{params, Connection};

use crate::{
    dto::{NewTitle, NewTitleCriteria, UpdateTitle},
    DbError, Record,
};

use super::procedures;

impl Record<Connection> for NewTitle {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let tx = connection
            .transaction()
            .expect("Save transaction could not be started");

        let id = {
            let mut stmt = tx
                .prepare(procedures::CREATE_TITLE)
                .expect("Failed to prepare statement");

            stmt.execute(params![self.name]).map_err(DbError::Sqlite)?;
            tx.last_insert_rowid() as usize
        };

        tx.commit().map_err(DbError::Sqlite).map(|_| id)
    }
}

impl Record<Connection> for UpdateTitle {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let tx = connection
            .transaction()
            .expect("Save transaction could not be started");

        {
            let mut stmt = tx
                .prepare(procedures::UPDATE_TITLE)
                .expect("Failed to prepare statement");

            stmt.execute(params![self.id, self.name])
                .map_err(DbError::Sqlite)?;
        }

        tx.commit().map_err(DbError::Sqlite).map(|_| 1)
    }
}

impl Record<Connection> for NewTitleCriteria {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let tx = connection
            .transaction()
            .expect("Save transaction could not be started");

        let id = {
            let mut stmt = tx
                .prepare(procedures::CREATE_TITLE_CRITERIA)
                .expect("Failed to prepare statement");

            stmt.execute(params![self.title, self.criteria])
                .map_err(DbError::Sqlite)?;

            tx.last_insert_rowid() as usize
        };

        tx.commit().map_err(DbError::Sqlite).map(|_| id)
    }
}
