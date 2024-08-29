use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[derive(serde::Serialize)]
enum HealthStatus {
    #[serde(rename = "healthy")]
    Health,
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: HealthStatus,
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: HealthStatus::Health,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check))
        .bind(("localhost", 3001))?
        .run()
        .await
}
