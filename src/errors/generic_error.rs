use crate::enums::HttpStatus;

#[derive(serde::Serialize)]
pub struct ErrorPayload {
    pub message: String,
}

pub struct GenericError {
    pub status: HttpStatus,
    pub payload: ErrorPayload,
}
