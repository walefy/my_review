use crate::{dto::UserCreation, prisma::PrismaClient};
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::service::user_service;

pub async fn create(client: Data<PrismaClient>, body: Json<UserCreation>) -> impl Responder {
    let result = user_service::create(body.0, client).await;

    match result {
        Ok(service_response) => HttpResponse::Ok().json(service_response),
        Err(service_err) => {
            let status = service_err.status;
            let response = status.to_response_builder().json(service_err.payload);
            response
        }
    }
}
