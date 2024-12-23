#[cfg(feature = "rusqlite")]
pub mod critic_sqlite;
pub mod dto;
pub mod elo;
mod seed;

#[derive(Debug)]
pub enum DbError {
    #[cfg(feature = "rusqlite")]
    Sqlite(rusqlite::Error),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "rusqlite")]
            DbError::Sqlite(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for DbError {}

pub trait Record<T> {
    fn save(&self, connection: &mut T) -> Result<usize, DbError>;
}

pub trait DbConnection: Sized {
    fn open_category<T: AsRef<std::path::Path>>(path: T) -> Result<Self, DbError>;
    fn save<T: Record<Self>>(&mut self, record: &T) -> Result<usize, DbError>;
}

pub trait CriticData {
    fn next_contest(&self) -> Result<dto::Contest, DbError>;
    fn top(&self, criteria_group: &str, count: usize, page: usize) -> Result<Vec<dto::TopRow>, DbError>;
}

pub mod prelude {
    #[cfg(feature = "rusqlite")]
    pub use crate::critic_sqlite::Connection;
    pub use crate::{
        dto::{CategoryItem, MatchResult, NewCategoryItem},
        CriticData, DbConnection, DbError, Record,
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
