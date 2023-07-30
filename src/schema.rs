// @generated automatically by Diesel CLI.

diesel::table! {
    order (id) {
        id -> Int4,
        status -> Int2,
        total_price -> Numeric,
        total_discount -> Nullable<Numeric>,
        created_at -> Timestamp,
        customer_id -> Int4,
        store_id -> Int4,
        payment_method_id -> Int4,
        shipping_information_id -> Int4,
    }
}

diesel::table! {
    order_item (id) {
        id -> Int4,
        unit_price -> Numeric,
        quantity -> Int4,
        product_item_id -> Int4,
        shopping_cart_id -> Nullable<Int4>,
        order_id -> Nullable<Int4>,
    }
}

diesel::table! {
    payment_method (id) {
        id -> Int4,
        name -> Text,
        inactive -> Bool,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        store_id -> Int4,
    }
}

diesel::table! {
    product (id) {
        id -> Int4,
        name -> Text,
        deleted -> Bool,
        store_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    product_category (id) {
        id -> Int4,
        name -> Text,
        store_id -> Int4,
    }
}

diesel::table! {
    product_discount (id) {
        id -> Int4,
        percentual -> Numeric,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        store_id -> Int4,
    }
}

diesel::table! {
    product_item (id) {
        id -> Int4,
        sku -> Nullable<Text>,
        image_url -> Nullable<Text>,
        description -> Nullable<Text>,
        price -> Numeric,
        stock -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
        variant_id -> Nullable<Int4>,
        product_id -> Int4,
        discount_id -> Nullable<Int4>,
        store_id -> Int4,
    }
}

diesel::table! {
    product_variant (id) {
        id -> Int4,
        name -> Text,
        value -> Text,
        store_id -> Int4,
    }
}

diesel::table! {
    shipping_information (id) {
        id -> Int4,
        status -> Int4,
        tracking_number -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        shipping_price -> Numeric,
        address_id -> Int4,
        shipping_method_id -> Int4,
    }
}

diesel::table! {
    shipping_method (id) {
        id -> Int4,
        name -> Text,
        inactive -> Bool,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        store_id -> Int4,
    }
}

diesel::table! {
    shopping_cart (id) {
        id -> Int4,
        customer_id -> Int4,
        store_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    store (id) {
        id -> Int4,
        name -> Text,
        logo_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    store_invite (id) {
        id -> Text,
        valid -> Bool,
        created_at -> Timestamp,
        store_id -> Int4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        salt -> Text,
        first_name -> Text,
        last_name -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login -> Timestamp,
        managed_store_id -> Nullable<Int4>,
    }
}

diesel::table! {
    user_address (id) {
        id -> Int4,
        address_line1 -> Text,
        address_line2 -> Nullable<Text>,
        number -> Text,
        city -> Text,
        country -> Text,
        postal_code -> Text,
        phone_country_code -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::joinable!(order -> payment_method (payment_method_id));
diesel::joinable!(order -> shipping_information (shipping_information_id));
diesel::joinable!(order -> store (store_id));
diesel::joinable!(order -> user (customer_id));
diesel::joinable!(order_item -> order (order_id));
diesel::joinable!(order_item -> product_item (product_item_id));
diesel::joinable!(order_item -> shopping_cart (shopping_cart_id));
diesel::joinable!(payment_method -> store (store_id));
diesel::joinable!(product -> product_category (category_id));
diesel::joinable!(product -> store (store_id));
diesel::joinable!(product_category -> store (store_id));
diesel::joinable!(product_discount -> store (store_id));
diesel::joinable!(product_item -> product (product_id));
diesel::joinable!(product_item -> product_discount (discount_id));
diesel::joinable!(product_item -> product_variant (variant_id));
diesel::joinable!(product_item -> store (store_id));
diesel::joinable!(product_variant -> store (store_id));
diesel::joinable!(shipping_information -> shipping_method (shipping_method_id));
diesel::joinable!(shipping_information -> user_address (address_id));
diesel::joinable!(shipping_method -> store (store_id));
diesel::joinable!(shopping_cart -> store (store_id));
diesel::joinable!(shopping_cart -> user (customer_id));
diesel::joinable!(store_invite -> store (store_id));
diesel::joinable!(user -> store (managed_store_id));
diesel::joinable!(user_address -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    order,
    order_item,
    payment_method,
    product,
    product_category,
    product_discount,
    product_item,
    product_variant,
    shipping_information,
    shipping_method,
    shopping_cart,
    store,
    store_invite,
    user,
    user_address,
);
