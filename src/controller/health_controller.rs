use crate::dto::HealthResponse;
use crate::enums::HealthStatus;
use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: HealthStatus::Health,
    })
}
