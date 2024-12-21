use rusqlite::{params, Connection};

use crate::{dto::NewCategoryItem, DbError, Record};

impl Record<Connection> for NewCategoryItem {
    fn save(&self, connection: &mut Connection) -> Result<usize, DbError> {
        let tx = connection
            .transaction()
            .expect("Save transaction could not be started");

        {
            let mut ins_stmt = tx
                .prepare(include_str!("ins_entry.sql"))
                .expect("Failed to prepare statement");

            let mut ins_sub_stmt = tx
                .prepare(include_str!("ins_entry_criterion.sql"))
                .expect("Failed to prepare statement");

            ins_stmt
                .execute(params![self.name])
                .map_err(DbError::Sqlite)?;

            ins_sub_stmt
                .execute(params![self.name, "General"])
                .map_err(DbError::Sqlite)?;

            for sc in self.sub_categories.iter().filter(|x| !x.is_empty()) {
                ins_sub_stmt
                    .execute(params![self.name, sc])
                    .map_err(DbError::Sqlite)?;
            }
        }

        tx.commit().map_err(DbError::Sqlite).map(|_| 1)
    }
}
