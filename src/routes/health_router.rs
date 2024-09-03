use actix_web::{web, Scope};

use crate::controller::health_controller::health_check;

pub fn health_router() -> Scope {
    web::scope("/health").service(health_check)
}
