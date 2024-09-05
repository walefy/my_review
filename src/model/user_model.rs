use crate::prisma::*;
use actix_web::web::Data;
use prisma_client_rust::{raw, QueryError};
use user::SetParam;

use crate::dto::UserCreation;

pub async fn create(
    user_creation: UserCreation,
    client: &Data<PrismaClient>,
) -> Result<user::Data, QueryError> {
    let params: Vec<SetParam> = vec![SetParam::SetPhotoUrl(user_creation.photo_url)];

    client
        .user()
        .create(user_creation.email, user_creation.name, params)
        .exec()
        .await
}

pub async fn find_user_by_email(
    email: String,
    client: &Data<PrismaClient>,
) -> Result<Option<user::Data>, QueryError> {
    client
        .user()
        .find_unique(user::UniqueWhereParam::EmailEquals(email))
        .exec()
        .await
}

pub async fn find_user_by_id(
    id: i32,
    client: &Data<PrismaClient>,
) -> Result<Option<user::Data>, QueryError> {
    client
        .user()
        .find_unique(user::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
}

pub async fn find_all_user(client: &Data<PrismaClient>) -> Result<Vec<user::Data>, QueryError> {
    client._query_raw(raw!("SELECT * FROM User")).exec().await
}

pub async fn delete_user_by_id(
    id: i32,
    client: &Data<PrismaClient>,
) -> Result<user::Data, QueryError> {
    client
        .user()
        .delete(user::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
}
