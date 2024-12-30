pub struct CategoryItem {
    pub id: i32,
    pub name: String,
    pub sub_categories: Vec<String>,
    pub elo: f32,
}

pub struct NewCategoryItem {
    pub name: String,
    pub sub_categories: Vec<String>,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct Title {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct NewTitle {
    pub name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct UpdateTitle {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct NewTitleCriteria {
    pub title: i32,
    pub criteria: i32,
}
