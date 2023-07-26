use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(ping_controller::ping))
        .service(
            web::scope("auth")
                .service(auth_controllers::signup)
                .service(auth_controllers::login),
        );
}
