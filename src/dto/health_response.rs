use crate::enums::HealthStatus;

#[derive(serde::Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
}
