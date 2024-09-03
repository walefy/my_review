use crate::enums::UserErrorStatus;
use crate::errors::{UserError, UserErrorPayload};
use crate::prisma::{user, PrismaClient};
use actix_web::web::Data;

use crate::dto::UserCreation;
use crate::model::user_model;

pub async fn create(
    user_creation: UserCreation,
    client: Data<PrismaClient>,
) -> Result<user::Data, UserError> {
    let user_found = user_model::find_user_by_email(user_creation.email.clone(), &client).await;

    match user_found {
        Ok(res) => match res {
            Some(_) => {
                return Err(UserError {
                    status: UserErrorStatus::Conflict,
                    payload: UserErrorPayload {
                        message: "user already exists!".to_string(),
                    },
                });
            }
            None => {}
        },
        Err(_) => {
            return Err(UserError {
                status: UserErrorStatus::BadRequest,
                payload: UserErrorPayload {
                    message: "can't create user".to_string(),
                },
            })
        }
    }

    let result = user_model::create(user_creation, &client).await;

    match result {
        Ok(model_response) => Ok(model_response),
        Err(_) => Err(UserError {
            status: UserErrorStatus::BadRequest,
            // TODO: fazer um mapeamento melhor de qual erro ocorreu
            payload: UserErrorPayload {
                message: "can't create user".to_string(),
            },
        }),
    }
}

pub async fn find_user_by_id(client: Data<PrismaClient>, id: i32) -> Result<user::Data, UserError> {
    let result = user_model::find_user_by_id(id, &client).await;

    match result {
        Ok(user) => {
            if user.is_none() {
                return Err(UserError {
                    status: UserErrorStatus::NotFound,
                    payload: UserErrorPayload {
                        message: format!("can't find user with id {id}"),
                    },
                });
            }

            Ok(user.unwrap())
        }
        Err(_) => Err(UserError {
            status: UserErrorStatus::NotFound,
            payload: UserErrorPayload {
                message: format!("can't find user with id {id}"),
            },
        }),
    }
}
