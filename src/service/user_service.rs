use crate::enums::HttpStatus;
use crate::errors::{ErrorPayload, GenericError};
use crate::prisma::{user, PrismaClient};
use actix_web::web::Data;

use crate::dto::UserCreation;
use crate::model::user_model;

pub async fn create(
    user_creation: UserCreation,
    client: Data<PrismaClient>,
) -> Result<user::Data, GenericError> {
    let user_found = user_model::find_user_by_email(user_creation.email.clone(), &client).await;

    match user_found {
        Ok(res) => {
            if res.is_some() {
                return Err(GenericError {
                    status: HttpStatus::Conflict,
                    payload: ErrorPayload {
                        message: "user already exists!".to_string(),
                    },
                });
            }
        }
        Err(_) => {
            return Err(GenericError {
                status: HttpStatus::BadRequest,
                payload: ErrorPayload {
                    message: "can't create user".to_string(),
                },
            })
        }
    }

    let result = user_model::create(user_creation, &client).await;

    match result {
        Ok(model_response) => Ok(model_response),
        Err(_) => Err(GenericError {
            status: HttpStatus::BadRequest,
            // TODO: fazer um mapeamento melhor de qual erro ocorreu
            payload: ErrorPayload {
                message: "can't create user".to_string(),
            },
        }),
    }
}

pub async fn find_all_user(client: Data<PrismaClient>) -> Result<Vec<user::Data>, GenericError> {
    let result = user_model::find_all_user(&client).await;

    match result {
        Ok(users) => Ok(users),
        Err(_) => Err(GenericError {
            status: HttpStatus::BadRequest,
            payload: ErrorPayload {
                message: "can't find all users".to_string(),
            },
        }),
    }
}

pub async fn find_user_by_id(
    client: &Data<PrismaClient>,
    id: i32,
) -> Result<user::Data, GenericError> {
    let result = user_model::find_user_by_id(id, client).await;

    match result {
        Ok(user) => {
            if user.is_none() {
                return Err(GenericError {
                    status: HttpStatus::NotFound,
                    payload: ErrorPayload {
                        message: format!("can't find user with id {id}"),
                    },
                });
            }

            Ok(user.unwrap())
        }
        Err(_) => Err(GenericError {
            status: HttpStatus::NotFound,
            payload: ErrorPayload {
                message: format!("can't find user with id {id}"),
            },
        }),
    }
}

pub async fn delete_by_id(client: Data<PrismaClient>, id: i32) -> Result<HttpStatus, GenericError> {
    let user_found = user_model::find_user_by_id(id, &client).await;

    match user_found {
        Ok(user) => {
            if user.is_none() {
                return Err(GenericError {
                    status: HttpStatus::NotFound,
                    payload: ErrorPayload {
                        message: format!("can't find user with id {id}"),
                    },
                });
            }
        }
        Err(_) => {
            return Err(GenericError {
                status: HttpStatus::NotFound,
                payload: ErrorPayload {
                    message: format!("can't find user with id {id}"),
                },
            });
        }
    }

    let result = user_model::delete_user_by_id(id, &client).await;
    match result {
        Ok(_) => Ok(HttpStatus::NoContent),
        Err(_) => Err(GenericError {
            status: HttpStatus::BadRequest,
            payload: ErrorPayload {
                message: "can't delete user!".to_string(),
            },
        }),
    }
}
