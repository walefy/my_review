use prisma_client_rust::chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{review, user};

#[derive(Deserialize, Serialize, Clone)]
pub struct Reviewable {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub rating: i32,
    pub creator: user::Data,
    pub reviews: Vec<review::Data>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "referenceLink")]
    pub reference_link: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,
}
