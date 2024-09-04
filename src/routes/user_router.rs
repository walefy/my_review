use actix_web::{web, Scope};

use crate::controller::user_controller;

pub fn user_router() -> Scope {
    web::scope("/user")
        .service(user_controller::create)
        .service(user_controller::find_user_by_id)
        .service(user_controller::delete_user_by_id)
}
