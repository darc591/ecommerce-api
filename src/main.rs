mod api;
mod config;
mod constants;
mod error;
mod middleware;
mod models;
mod schema;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use std::env;

// test commit
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let pool = config::db::init_db_pool(&db_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .configure(crate::config::routes::config_services)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
