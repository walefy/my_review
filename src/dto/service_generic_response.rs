use crate::enums::HttpStatus;

#[derive(serde::Serialize)]
pub struct ServiceResponsePayload {
    pub message: String,
}

pub struct ServiceResponse {
    pub status: HttpStatus,
    pub payload: ServiceResponsePayload,
}
