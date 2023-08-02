use crate::{ models::product::ProductItem, error::ServiceError };
use super::Connection;
use diesel::sql_types::Integer;
use diesel::{ prelude::*, sql_query };

pub fn find(id: &i32, conn: &mut Connection) -> Result<ProductItem, ServiceError> {
    let product_item_result = sql_query(
        "SELECT * FROM public.product_item WHERE id = $1 AND deleted = false"
    )
        .bind::<Integer, _>(id)
        .get_result::<ProductItem>(conn);

    match product_item_result {
        Ok(product_item) => Ok(product_item),
        Err(e) =>
            Err(ServiceError::NotFound { error_message: "Product item not found!".to_string() }),
    }
}
