-- Your SQL goes here
DROP INDEX IF EXISTS product_variant_name_value_key;

CREATE UNIQUE INDEX product_variant_name_value_store_key ON product_variant (name, value, store_id);

