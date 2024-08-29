#[derive(serde::Serialize)]
pub enum HealthStatus {
    #[serde(rename = "healthy")]
    Health,
}
