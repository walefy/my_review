mod controller;
mod dto;
mod enums;
mod model;
#[allow(warnings, unused)]
mod prisma;
mod routes;
mod service;

use actix_web::{web, App, HttpServer};
use prisma::*;
use routes::health_router::health_router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("can't connect to database!"),
    );

    HttpServer::new(move || App::new().app_data(client.clone()).service(health_router()))
        .bind(("localhost", 3001))?
        .run()
        .await
}
