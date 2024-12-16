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
