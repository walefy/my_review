use actix_web::web::Data;

use crate::dto::ReviewableCreation;
use crate::enums::HttpStatus;
use crate::errors::{ErrorPayload, GenericError};
use crate::model::reviewable_model;
use crate::prisma::{reviewable, PrismaClient};

pub async fn create(
    reviewable_creation: ReviewableCreation,
    client: Data<PrismaClient>,
) -> Result<reviewable::Data, GenericError> {
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
