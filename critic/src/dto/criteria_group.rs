#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct CriteriaGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct CriteriaGroupItem {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct UpdateCriteriaGroup {
    pub id: i32,
    pub name: String,
}
