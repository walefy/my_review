use crate::{dto::UserCreation, prisma::PrismaClient};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::service::user_service;

#[post("")]
pub async fn create(client: Data<PrismaClient>, body: Json<UserCreation>) -> impl Responder {
    let result = user_service::create(body.0, client).await;

    match result {
        Ok(service_response) => HttpResponse::Created().json(service_response),
        Err(service_err) => {
            let status = service_err.status;
            status.to_response_builder().json(service_err.payload)
        }
    }
}

#[get("/{user_id}")]
pub async fn find_user_by_id(client: Data<PrismaClient>, user_id: Path<i32>) -> impl Responder {
    let result = user_service::find_user_by_id(client, user_id.into_inner()).await;

    match result {
        Ok(service_response) => HttpResponse::Ok().json(service_response),
        Err(service_err) => {
            let status = service_err.status;
            status.to_response_builder().json(service_err.payload)
        }
    }
}
