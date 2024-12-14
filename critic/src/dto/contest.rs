use super::Category;

pub struct Contestant {
    pub id: i32,
    pub name: String,
    pub elo: i32,
}

pub struct Contest {
    pub category: Category,
    pub a: Contestant,
    pub b: Contestant,
}
