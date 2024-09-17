use actix_web::{HttpResponse, HttpResponseBuilder};

pub enum HttpStatus {
    BadRequest,
    Conflict,
    NotFound,
    NoContent,
    InternalServerError,
}

impl HttpStatus {
    pub fn to_response_builder(&self) -> HttpResponseBuilder {
        match self {
            HttpStatus::BadRequest => HttpResponse::BadRequest(),
            HttpStatus::Conflict => HttpResponse::Conflict(),
            HttpStatus::NotFound => HttpResponse::NotFound(),
            HttpStatus::NoContent => HttpResponse::NoContent(),
            HttpStatus::InternalServerError => HttpResponse::InternalServerError(),
        }
    }
}
