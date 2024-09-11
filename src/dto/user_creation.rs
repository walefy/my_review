#[derive(serde::Deserialize)]
pub struct UserCreation {
    pub email: String,
    pub name: String,
    pub photo_url: Option<String>,
}
