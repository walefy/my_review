use crate::dto::HealthResponse;
use crate::enums::HealthStatus;
use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: HealthStatus::Health,
    })
}
