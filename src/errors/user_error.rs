use crate::enums::HttpStatus;

#[derive(serde::Serialize)]
pub struct UserErrorPayload {
    pub message: String,
}

pub struct UserError {
    pub status: HttpStatus,
    pub payload: UserErrorPayload,
}
