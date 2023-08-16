use bigdecimal::{ BigDecimal, FromPrimitive };
use diesel::{ prelude::*, sql_query, sql_types::Integer };
use crate::{
    controllers::product::{ CreateCategoryBody, CreateVariantBody, CreateProductBody },
    models::{
        product::{
            InsertableCategory,
            InsertableVariant,
            InsertableProduct,
            InsertableProductItem,
            ProductVariant,
        },
        response::IDResponse,
    },
    error::ServiceError,
    utils::validation::validate,
    db::store::StoreService,
};

use super::Connection;

pub struct ProductService;

impl ProductService {
    pub fn create_category(
        payload: CreateCategoryBody,
        store_id: &i32,
        user_id: &i32,
        conn: &mut Connection
    ) -> Result<IDResponse<i32>, ServiceError> {
        use crate::schema::product_category;

        validate(&payload)?;

        StoreService::check_store_admin(store_id, user_id, conn)?;

        let new_category = InsertableCategory {
            name: payload.name,
            store_id: store_id.to_owned(),
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

    pub fn create_variant(
        payload: CreateVariantBody,
        user_id: &i32,
        store_id: &i32,
        conn: &mut Connection
    ) -> Result<IDResponse<i32>, ServiceError> {
        use crate::schema::product_variant;

        validate(&payload)?;

        StoreService::check_store_admin(store_id, user_id, conn)?;

        let new_variant = InsertableVariant {
            name: payload.name,
            value: payload.value,
            store_id: store_id.to_owned(),
        };

        match
            diesel
                ::insert_into(product_variant::dsl::product_variant)
                .values(new_variant)
                .returning(product_variant::dsl::id)
                .get_result(conn)
        {
            Ok(id) => Ok(IDResponse { id }),
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }

    pub fn create(
        payload: CreateProductBody,
        user_id: &i32,
        store_id: &i32,
        conn: &mut Connection
    ) -> Result<IDResponse<i32>, ServiceError> {
        use crate::schema::{ product, product_item };
        validate(&payload)?;

        if payload.data.len() == 0 {
            return Err(ServiceError::BadRequest {
                error_message: "Missing product data".to_string(),
            });
        }

        StoreService::check_store_admin(store_id, user_id, conn)?;

        let new_product = InsertableProduct {
            name: payload.name,
            category_id: payload.category_id,
            store_id: store_id.to_owned(),
        };

        match
            diesel
                ::insert_into(product::dsl::product)
                .values(new_product)
                .returning(product::dsl::id)
                .get_result::<i32>(conn)
        {
            Ok(product_id) => {
                let product_items: Vec<InsertableProductItem> = payload.data
                    .into_iter()
                    .map(|p_data| InsertableProductItem {
                        description: Some(p_data.description),
                        image_url: Some(p_data.image),
                        sku: Some(p_data.sku),
                        price: BigDecimal::from_f32(p_data.price).unwrap(),
                        stock: p_data.stock,
                        variant_id: p_data.variant_id,
                        store_id: store_id.to_owned(),
                        product_id: product_id,
                    })
                    .collect();

                let product_items_result = diesel
                    ::insert_into(product_item::dsl::product_item)
                    .values(product_items)
                    .execute(conn);

                if product_items_result.is_err() {
                    return Err(ServiceError::InternalServerError {
                        error_message: product_items_result.unwrap_err().to_string(),
                    });
                }

                Ok(IDResponse { id: product_id })
            }
            Err(e) =>
                Err(ServiceError::InternalServerError {
                    error_message: e.to_string(),
                }),
        }
    }

    pub fn list_variants(
        store_id: &i32,
        conn: &mut Connection
    ) -> Result<Vec<ProductVariant>, ServiceError> {
        let variants_result = sql_query("SELECT * FROM public.product_variant WHERE store_id = $1")
            .bind::<Integer, _>(store_id)
            .get_results::<ProductVariant>(conn);

        match variants_result {
            Ok(values) => Ok(values),
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }
}
