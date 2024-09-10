use actix_web::{web, Scope};

use crate::controller::reviewable_controller;

pub fn reviewable_controller() -> Scope {
    web::scope("/reviewable").service(reviewable_controller::create)
}
