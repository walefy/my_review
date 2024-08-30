use crate::prisma::*;
use actix_web::web::Data;
use prisma_client_rust::QueryError;
use user::SetParam;

use crate::dto::UserCreation;

pub async fn create(
    user_creation: UserCreation,
    client: Data<PrismaClient>,
) -> Result<user::Data, QueryError> {
    let params: Vec<SetParam> = vec![SetParam::SetPhotoUrl(user_creation.photo_url)];

    Ok(client
        .user()
        .create(user_creation.email, user_creation.name, params)
        .exec()
        .await?)
}
