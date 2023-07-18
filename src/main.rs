mod api;
mod db;
mod models;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).route("/ping", web::get().to(ping))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
