use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse, Responder};

use crate::dto::ReviewableCreation;
use crate::service::reviewable_service;
use crate::PrismaClient;

#[post("")]
async fn create(client: Data<PrismaClient>, body: Json<ReviewableCreation>) -> impl Responder {
    let result = reviewable_service::create(body.0, client).await;

    match result {
        Ok(service_response) => HttpResponse::Ok().json(service_response),
        Err(service_err) => {
            let status = service_err.status;

            status.to_response_builder().json(service_err.payload)
        }
    }
}
