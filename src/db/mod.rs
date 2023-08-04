pub mod user;
pub mod address;
pub mod store;
pub mod product;
pub mod shopping_cart;
pub mod order_item;
pub mod product_item;

use diesel::{ pg::PgConnection, r2d2::{ self, ConnectionManager } };

pub type Connection = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn new_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<Connection>::new(db_url);

    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    pool
}
