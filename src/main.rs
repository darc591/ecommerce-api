mod constants;
mod error;
mod middleware;
mod models;
mod schema;
mod utils;
mod controllers;
mod db;

use actix_web::{ middleware::Logger, web, App, HttpServer };
use log::info;
use std::env;

fn routes(app: &mut web::ServiceConfig) {
    app.service(
            web::scope("auth")
                .service(controllers::auth::signup)
                .service(controllers::auth::login)
        )
        .service(
            web::scope("address")
                .service(controllers::address::find_address)
                .service(controllers::address::list_addresses)
                .service(controllers::address::create_address)
                .service(controllers::address::edit_address)
                .service(controllers::address::delete_address)
        )
        .service(
            web::scope("store")
                .service(controllers::store::create_store)
        )
        .service(
            web::scope("product")
                .service(controllers::product::create_product_category)
                .service(controllers::product::create_product_variant)
                .service(controllers::product::create_product)
        );
        
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "info");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let pool = db::new_pool(&db_url);

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("logando!");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .configure(routes)
        })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind to server address {}", &bind_address))
        .run().await
}
