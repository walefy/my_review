use actix_web::web::Data;
use prisma_client_rust::QueryError;
use reviewable::SetParam;

use crate::dto::ReviewableCreation;
use crate::entity::Reviewable;
use crate::prisma::*;

pub async fn create(
    reviewable_creation: ReviewableCreation,
    client: &Data<PrismaClient>,
) -> Result<Reviewable, QueryError> {
    let params: Vec<SetParam> = vec![
        SetParam::SetRating(reviewable_creation.rating),
        SetParam::SetImageUrl(reviewable_creation.image_url),
        SetParam::SetReferenceLink(reviewable_creation.reference_link),
    ];

    let result_with_include = client
        .reviewable()
        .create(
            reviewable_creation.title,
            reviewable_creation.description,
            user::UniqueWhereParam::IdEquals(reviewable_creation.creator_id),
            params,
        )
        .include(reviewable::include!({ creator reviews }))
        .exec()
        .await;

    match result_with_include {
        Ok(result) => Ok(Reviewable {
            id: result.id,
            title: result.title,
            description: result.description,
            rating: result.rating,
            creator: result.creator,
            reviews: result.reviews,
            image_url: result.image_url,
            reference_link: result.reference_link,
            created_at: result.created_at,
            updated_at: result.updated_at,
        }),
        Err(err) => Err(err),
    }
}

pub async fn find_all(client: &Data<PrismaClient>) -> Result<Vec<Reviewable>, QueryError> {
    let result_with_include = client
        .reviewable()
        .find_many(vec![])
        .include(reviewable::include!({ creator reviews }))
        .exec()
        .await;

    match result_with_include {
        Ok(result) => Ok(result
            .into_iter()
            .map(|r| Reviewable {
                id: r.id,
                title: r.title,
                description: r.description,
                rating: r.rating,
                creator: r.creator,
                reviews: r.reviews,
                image_url: r.image_url,
                reference_link: r.reference_link,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect()),
        Err(err) => Err(err),
    }
}
