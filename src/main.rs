mod controller;
mod dto;
mod entity;
mod enums;
mod errors;
mod middlewares;
mod model;
#[allow(warnings, unused)]
mod prisma;
mod routes;
mod service;

use actix_web::{web, App, HttpServer};
use prisma::*;
use routes::{health_router, reviewable_controller, user_router};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("can't connect to database!"),
    );

    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    #[cfg(not(debug_assertions))]
    client._migrate_deploy();

    println!("[server] starting on port 3001");

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(health_router())
            .service(user_router())
            .service(reviewable_controller())
    })
    .bind(("localhost", 3001))?
    .run()
    .await
}
