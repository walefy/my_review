use crate::enums::health_status::HealthStatus;

#[derive(serde::Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
}
