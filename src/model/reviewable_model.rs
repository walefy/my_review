use actix_web::web::Data;
use prisma_client_rust::QueryError;
use reviewable::SetParam;

use crate::dto::ReviewableCreation;
use crate::prisma::*;

pub async fn create(
    reviewable_creation: ReviewableCreation,
    client: &Data<PrismaClient>,
) -> Result<reviewable::Data, QueryError> {
    let params: Vec<SetParam> = vec![
        SetParam::SetRating(reviewable_creation.rating),
        SetParam::SetImageUrl(reviewable_creation.image_url),
        SetParam::SetReferenceLink(reviewable_creation.reference_link),
    ];

    client
        .reviewable()
        .create(
            reviewable_creation.title,
            reviewable_creation.description,
            user::UniqueWhereParam::IdEquals(reviewable_creation.creator_id),
            params,
        )
        .exec()
        .await
}
