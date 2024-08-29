use crate::dto::health_response::HealthResponse;
use crate::enums::health_status::HealthStatus;
use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: HealthStatus::Health,
    })
}
