use actix_web::{web, Resource};

use crate::controller::health_controller::health_check;

pub fn health_router() -> Resource {
    web::resource("/health").route(web::get().to(health_check))
}
