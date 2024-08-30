use actix_web::{web, Resource};

use crate::controller::user_controller;

pub fn user_controller() -> Resource {
    web::resource("/user").route(web::post().to(user_controller::create))
}
