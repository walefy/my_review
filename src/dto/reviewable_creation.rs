pub struct ReviewableCreation {
    pub title: String,
    pub description: String,
    pub rating: i32,
    pub image_url: Option<String>,
    pub reference_link: Option<String>,
    pub creator_id: i32,
}
