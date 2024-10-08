use crate::{entity::User, prisma::*};
use actix_web::web::Data;
use prisma_client_rust::QueryError;
use user::SetParam;

use crate::dto::UserCreation;

pub async fn create(
    user_creation: UserCreation,
    client: &Data<PrismaClient>,
) -> Result<User, QueryError> {
    let params: Vec<SetParam> = vec![SetParam::SetPhotoUrl(user_creation.photo_url)];

    let query_result = client
        .user()
        .create(user_creation.email, user_creation.name, params)
        .include(user::include!({ reviews }))
        .exec()
        .await;

    match query_result {
        Ok(result) => Ok(User {
            id: result.id,
            email: result.email,
            name: result.name,
            photo_url: result.photo_url,
            reviews: result.reviews,
        }),
        Err(err) => Err(err),
    }
}

pub async fn find_user_by_email(
    email: String,
    client: &Data<PrismaClient>,
) -> Result<Option<User>, QueryError> {
    let query_result = client
        .user()
        .find_unique(user::UniqueWhereParam::EmailEquals(email))
        .include(user::include!({ reviews }))
        .exec()
        .await;

    match query_result {
        Ok(result) => {
            if result.is_none() {
                return Ok(None);
            }

            let result_verified = result.unwrap();
            Ok(Some(User {
                id: result_verified.id,
                email: result_verified.email.to_owned(),
                name: result_verified.name.to_owned(),
                photo_url: result_verified.photo_url.to_owned(),
                reviews: result_verified.reviews.to_owned(),
            }))
        }
        Err(err) => Err(err),
    }
}

pub async fn find_user_by_id(
    id: i32,
    client: &Data<PrismaClient>,
) -> Result<Option<User>, QueryError> {
    let query_result = client
        .user()
        .find_unique(user::UniqueWhereParam::IdEquals(id))
        .include(user::include!({ reviews }))
        .exec()
        .await;

    match query_result {
        Ok(result) => {
            if result.is_none() {
                return Ok(None);
            };

            let result_verified = result.unwrap();

            Ok(Some(User {
                id: result_verified.id,
                email: result_verified.email,
                name: result_verified.name,
                photo_url: result_verified.photo_url,
                reviews: result_verified.reviews,
            }))
        }
        Err(err) => Err(err),
    }
}

pub async fn find_all_user(client: &Data<PrismaClient>) -> Result<Vec<User>, QueryError> {
    let query_result = client
        .user()
        .find_many(vec![])
        .include(user::include!({ reviews }))
        .exec()
        .await;

    match query_result {
        Ok(result) => Ok(result
            .into_iter()
            .map(|user| User {
                id: user.id,
                email: user.email,
                name: user.name,
                photo_url: user.photo_url,
                reviews: user.reviews,
            })
            .collect()),
        Err(err) => Err(err),
    }
}

pub async fn delete_user_by_id(id: i32, client: &Data<PrismaClient>) -> Result<(), QueryError> {
    let _ = client
        .user()
        .delete(user::UniqueWhereParam::IdEquals(id))
        .exec()
        .await;

    Ok(())
}
