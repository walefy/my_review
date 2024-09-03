use actix_web::{HttpResponse, HttpResponseBuilder};

pub enum UserErrorStatus {
    BadRequest,
    Conflict,
    NotFound,
}

impl UserErrorStatus {
    pub fn to_response_builder(&self) -> HttpResponseBuilder {
        match self {
            UserErrorStatus::BadRequest => HttpResponse::BadRequest(),
            UserErrorStatus::Conflict => HttpResponse::Conflict(),
            UserErrorStatus::NotFound => HttpResponse::NotFound(),
        }
    }
}
