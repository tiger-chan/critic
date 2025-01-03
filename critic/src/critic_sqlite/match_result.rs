use rusqlite::{params, Connection};

use crate::{dto::MatchResult, DbError, Record};

use super::procedures;

impl Record<Connection> for MatchResult {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let tx = connection
            .transaction()
            .expect("Save transaction could not be started");

        {
            let mut ins_stmt = tx
                .prepare(procedures::ADD_CONTEST_RESULT)
                .expect("Failed to prepare statement");

            let mut update_stmt = tx
                .prepare(procedures::UPDATE_ELO)
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
                .execute(params![self.a, self.criteria_group, self.elo_change.0])
                .map_err(DbError::Sqlite)?;

            update_stmt
                .execute(params![self.b, self.criteria_group, self.elo_change.1])
                .map_err(DbError::Sqlite)?;
        }

        tx.commit().map_err(DbError::Sqlite).map(|_| 1)
    }
}
