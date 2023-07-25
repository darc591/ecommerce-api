mod api;
mod models;
mod config;
mod schema;
mod error;
mod constants;

use std::env;

use actix_web::{ middleware::Logger, web, App, HttpResponse, HttpServer, Responder };

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let pool = config::db::init_db_pool(&db_url);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(pool.clone()))
            .configure(crate::config::app::config_services)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
