use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(ping_controller::ping))
        .service(
            web
                ::scope("auth")
                .service(web::resource("/signup").route(web::post().to(auth_controllers::signup)))
        )
        .service(
            web
                ::scope("/user")
                .service(web::resource("").route(web::get().to(auth_controllers::find_all)))
                .service(
                    web::resource("/email").route(web::get().to(auth_controllers::find_by_email))
                )
        );
}
