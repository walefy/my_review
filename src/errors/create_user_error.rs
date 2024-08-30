use crate::enums::CreateUserErrorStatus;

#[derive(serde::Serialize)]
pub struct CreateUserErrorPayload {
    pub message: &'static str,
}

pub struct CreateUserError {
    pub status: CreateUserErrorStatus,
    pub payload: CreateUserErrorPayload,
}
