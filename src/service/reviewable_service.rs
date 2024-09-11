use actix_web::web::Data;

use crate::dto::ReviewableCreation;
use crate::entity::Reviewable;
use crate::enums::HttpStatus;
use crate::errors::{ErrorPayload, GenericError};
use crate::model::reviewable_model;
use crate::prisma::PrismaClient;

use super::user_service;

pub async fn create(
    reviewable_creation: ReviewableCreation,
    client: Data<PrismaClient>,
) -> Result<Reviewable, GenericError> {
    if reviewable_creation.rating > 5 {
        return Err(GenericError {
            status: HttpStatus::BadRequest,
            payload: ErrorPayload {
                message: "the rating cannot be higher than 5.".to_string(),
            },
        });
    }

    let user_result = user_service::find_user_by_id(&client, reviewable_creation.creator_id).await;
    user_result?;

    let result = reviewable_model::create(reviewable_creation, &client).await;
    match result {
        Ok(model_response) => Ok(model_response),
        Err(_) => Err(GenericError {
            status: HttpStatus::BadRequest,
            payload: ErrorPayload {
                message: "reviewable can't be created".to_string(),
            },
        }),
    }
}
