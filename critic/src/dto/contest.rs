use super::Criterion;

pub struct Contestant {
    pub id: i32,
    pub name: String,
    pub elo: f32,
}

pub struct Contest {
    pub criterion: Criterion,
    pub a: Contestant,
    pub b: Contestant,
}
