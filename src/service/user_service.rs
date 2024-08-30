use crate::enums::CreateUserErrorStatus;
use crate::errors::{CreateUserError, CreateUserErrorPayload};
use crate::prisma::{user, PrismaClient};
use actix_web::web::Data;

use crate::dto::UserCreation;
use crate::model::user_model;

pub async fn create(
    user_creation: UserCreation,
    client: Data<PrismaClient>,
) -> Result<user::Data, CreateUserError> {
    let user_found = user_model::find_user_by_email(user_creation.email.clone(), &client).await;

    match user_found {
        Ok(res) => match res {
            Some(_) => {
                return Err(CreateUserError {
                    status: CreateUserErrorStatus::Conflict,
                    payload: CreateUserErrorPayload {
                        message: "user already exists!",
                    },
                });
            }
            None => {}
        },
        Err(_) => {
            return Err(CreateUserError {
                status: CreateUserErrorStatus::BadRequest,
                payload: CreateUserErrorPayload {
                    message: "can't create user",
                },
            })
        }
    }

    let result = user_model::create(user_creation, &client).await;

    match result {
        Ok(model_response) => Ok(model_response),
        Err(_) => Err(CreateUserError {
            status: CreateUserErrorStatus::BadRequest,
            // TODO: fazer um mapeamento melhor de qual erro ocorreu
            payload: CreateUserErrorPayload {
                message: "can't create user",
            },
        }),
    }
}
