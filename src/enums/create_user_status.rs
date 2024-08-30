use actix_web::{HttpResponse, HttpResponseBuilder};

pub enum CreateUserErrorStatus {
    BadRequest, // Conflict,
}

impl CreateUserErrorStatus {
    pub fn to_response_builder(&self) -> HttpResponseBuilder {
        match self {
            CreateUserErrorStatus::BadRequest => HttpResponse::BadRequest(),
            // CreateUserErrorStatus::Conflict => HttpResponse::Conflict(),
        }
    }
}
