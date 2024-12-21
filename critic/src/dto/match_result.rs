pub struct MatchResult {
    pub criteria_group: i32,
    pub criterion: i32,
    pub a: i32,
    pub b: i32,
    pub score: f32,
    pub elo_change: (f32, f32),
}
