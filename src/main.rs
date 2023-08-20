mod error;
mod middleware;
mod models;
mod schema;
mod utils;
mod controllers;
mod db;

use actix_cors::Cors;
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
            web::scope("addresses")
                .service(controllers::address::find_address)
                .service(controllers::address::list_addresses)
                .service(controllers::address::create_address)
                .service(controllers::address::edit_address)
                .service(controllers::address::delete_address)
        )
        .service(
            web::scope("stores")
                .service(controllers::store::create_store)
                .service(controllers::store::create_store_invite)
        )
        .service(
            web::scope("products")
                .service(controllers::product::create_product_category)
                .service(controllers::product::create_product_variant)
                .service(controllers::product::create_product)
                .service(controllers::product::list_variants)
                .service(controllers::product::list_categories)
        )
        .service(
            web::scope("shopping-carts")
                .service(controllers::shopping_cart::create_shopping_cart)
                .service(controllers::shopping_cart::edit_shopping_cart)
                .service(controllers::shopping_cart::delete_shopping_cart)
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

        let cors = Cors::permissive();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .configure(routes)
        })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind to server address {}", &bind_address))
        .run().await
}
