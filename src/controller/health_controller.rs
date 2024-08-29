use crate::{dto::health_response::HealthResponse, enums::health_status::HealthStatus};
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: HealthStatus::Health,
    })
}
