use super::Category;

pub struct Contestant {
    pub id: i32,
    pub name: String,
    pub elo: f32,
}

pub struct Contest {
    pub category: Category,
    pub a: Contestant,
    pub b: Contestant,
}
