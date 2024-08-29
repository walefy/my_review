mod controller;
mod dto;
mod enums;
mod model;
mod service;

use actix_web::{App, HttpServer};
use controller::health_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_controller::health_check))
        .bind(("localhost", 3001))?
        .run()
        .await
}
