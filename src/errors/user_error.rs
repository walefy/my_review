use crate::enums::UserErrorStatus;

#[derive(serde::Serialize)]
pub struct UserErrorPayload {
    pub message: String,
}

pub struct UserError {
    pub status: UserErrorStatus,
    pub payload: UserErrorPayload,
}
