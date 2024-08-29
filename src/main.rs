mod controller;
mod dto;
mod enums;
mod model;
mod routes;
mod service;

use actix_web::{App, HttpServer};
use routes::health_router::health_router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_router()))
        .bind(("localhost", 3001))?
        .run()
        .await
}
