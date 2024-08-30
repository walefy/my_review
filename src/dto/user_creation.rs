#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct UserCreation {
    pub email: String,
    pub name: String,
    pub photo_url: Option<String>,
}
