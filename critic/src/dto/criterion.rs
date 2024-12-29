#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct Criterion {
    pub group: i32,
    pub id: i32,
    pub name: String,
    pub group_name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct UpdateCriterion {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct NewCriterion {
    pub group: i32,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct DeleteCriterion {
    pub id: i32,
}
