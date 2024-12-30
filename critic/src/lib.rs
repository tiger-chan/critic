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
    fn top(
        &self,
        criteria_group: &str,
        count: usize,
        page: usize,
    ) -> Result<Vec<dto::TopRow>, DbError>;
    fn all_groups(&self) -> Result<Vec<dto::CriteriaGroup>, DbError>;
    fn criteria(&self, id: i32) -> Result<Vec<dto::CriteriaGroupItem>, DbError>;
    fn all_titles(&self) -> Result<Vec<dto::Title>, DbError>;
    fn groups_by_title(&self, title_id: i32) -> Result<Vec<dto::CriteriaGroup>, DbError>;
}

pub mod prelude {
    #[cfg(feature = "rusqlite")]
    pub use crate::critic_sqlite::Connection;
    pub use crate::{
        dto::{
            CategoryItem, CriteriaGroup, CriteriaGroupItem, Criterion, DeleteCriteriaGroup,
            DeleteCriterion, MatchResult, NewCategoryItem, NewCriteriaGroup, NewCriterion,
            NewTitle, Title, UpdateCriteriaGroup, UpdateCriterion, UpdateTitle,
        },
        CriticData, DbConnection, DbError, Record,
    };
}
