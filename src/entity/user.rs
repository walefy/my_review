use crate::review;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub reviews: Vec<review::Data>,
}
