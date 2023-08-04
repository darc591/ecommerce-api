use crate::{ models::product::ProductItem, error::ServiceError };
use super::Connection;
use diesel::sql_types::{ Integer, Bool };
use diesel::{ prelude::*, sql_query };

pub struct ProductItemService;

impl ProductItemService {
    pub fn find(
        id: &i32,
        deleted: bool,
        conn: &mut Connection
    ) -> Result<ProductItem, ServiceError> {
        let product_item_result = sql_query(
            "SELECT * FROM public.product_item WHERE id = $1 AND deleted = $2"
        )
            .bind::<Integer, _>(id)
            .bind::<Bool, _>(deleted)
            .get_result::<ProductItem>(conn);

        match product_item_result {
            Ok(product_item) => Ok(product_item),
            Err(_) =>
                Err(ServiceError::NotFound {
                    error_message: "Product item not found!".to_string(),
                }),
        }
    }
}
