use super::Criterion;

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct Contestant {
    pub id: i32,
    pub name: String,
    pub elo: f32,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct Contest {
    pub criterion: Criterion,
    pub a: Contestant,
    pub b: Contestant,
}
