#[cfg(feature = "rusqlite")]
pub mod critic_sqlite;

#[derive(Debug)]
pub enum DbError {
    #[cfg(feature = "rusqlite")]
    Sqlite(rusqlite::Error),
}

pub struct CategoryItem {
    pub name: String,
    pub sub_categories: Vec<String>,
}

pub struct EloRating {}

pub struct CondorcetRating {}

pub trait Record<T> {
    fn save(&self, connection: &mut T) -> Result<usize, DbError>;
}

pub trait DbConnection: Sized {
    fn open_category<T: AsRef<std::path::Path>>(path: T) -> Result<Self, DbError>;
    fn save<T: Record<Self>>(&mut self, record: &T) -> Result<usize, DbError>;
}

pub mod prelude {
    #[cfg(feature = "rusqlite")]
    pub use crate::critic_sqlite::Connection;
    pub use crate::{CategoryItem, CondorcetRating, DbConnection, DbError, EloRating, Record};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
