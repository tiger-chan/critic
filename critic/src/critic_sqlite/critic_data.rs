use rusqlite::{params, Connection};

use crate::{dto, CriticData, DbError};

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

            let cat_group_id: i32 = r.get(6)?;
            let cat_id: i32 = r.get(7)?;
            let cat_name: String = r.get(8)?;
            let cat_group_name: String = r.get(9)?;
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
                criterion: dto::Criterion {
                    group: cat_group_id,
                    id: cat_id,
                    name: cat_name,
                    group_name: cat_group_name,
                },
            })
        })
        .map_err(DbError::Sqlite)
    }

    fn top(
        &self,
        criteria_group: &str,
        count: usize,
        page: usize,
    ) -> Result<Vec<dto::TopRow>, DbError> {
        let mut stmt = self
            .prepare(include_str!("top_criteria.sql"))
            .expect("Failed to prepare statement");

        let first = page * count;
        let last = (page + 1) * count;

        let params = if criteria_group.is_empty() {
            params![rusqlite::types::Null, first, last]
        } else {
            params![criteria_group, first, last]
        };

        let row_iter = stmt
            .query_map(params, |r| {
                let elo: f32 = r.get(2)?;
                let elo = elo as i32;
                Ok(dto::TopRow {
                    group: r.get(0)?,
                    entry: r.get(1)?,
                    elo,
                })
            })
            .map_err(DbError::Sqlite)?;

        let mut results = Vec::new();
        for row in row_iter {
            results.push(row.unwrap());
        }

        Ok(results)
    }
}
