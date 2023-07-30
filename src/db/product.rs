use diesel::prelude::*;
use crate::{
    controllers::product::CreateCategoryBody,
    models::{ product::InsertableCategory, response::IDResponse },
    error::ServiceError,
    utils::validation::validate,
    db::store as db_store,
};

use super::Connection;

pub fn create_category(
    payload: CreateCategoryBody,
    store_id: i32,
    user_id: i32,
    conn: &mut Connection
) -> Result<IDResponse<i32>, ServiceError> {
    use crate::schema::product_category;

    validate(&payload)?;

    db_store::check_store_admin(store_id, user_id, conn)?;

    let new_category = InsertableCategory {
        name: payload.name,
        store_id,
    };

    match
        diesel
            ::insert_into(product_category::dsl::product_category)
            .values(new_category)
            .returning(product_category::dsl::id)
            .get_result::<i32>(conn)
    {
        Ok(result_id) => Ok(IDResponse { id: result_id }),
        Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
    }
}
