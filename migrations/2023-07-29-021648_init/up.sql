-- Your SQL goes here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    salt TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    type INTEGER NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    managed_store_id INTEGER
);

CREATE TABLE user_address (
    id SERIAL PRIMARY KEY,
    address_line1 TEXT NOT NULL,
    address_line2 TEXT,
    number TEXT NOT NULL,
    city TEXT NOT NULL,
    country TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    phone_country_code TEXT,
    phone_number TEXT,
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE store (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    logo_url TEXT,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE store_invite (
    id TEXT PRIMARY KEY,
    valid BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    store_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE payment_method (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    inactive BOOLEAN NOT NULL DEFAULT false,
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    store_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE shipping_method (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    inactive BOOLEAN NOT NULL DEFAULT false,
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    store_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT false,
    store_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE product_item (
    id SERIAL PRIMARY KEY,
    sku TEXT,
    image_url TEXT,
    description TEXT,
    price DECIMAL(15,2) NOT NULL,
    stock INTEGER NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted BOOLEAN NOT NULL DEFAULT false,
    variant_id INTEGER,
    product_id INTEGER NOT NULL,
    discount_id INTEGER,
    store_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE product_category (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    store_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE product_variant (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    store_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE product_discount (
    id SERIAL PRIMARY KEY,
    percentual DECIMAL(15,2) NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP(3) NOT NULL,
    store_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE order_item (
    id SERIAL PRIMARY KEY,
    unit_price DECIMAL(15,2) NOT NULL,
    quantity INTEGER NOT NULL,
    product_item_id INTEGER NOT NULL,
    shopping_cart_id INTEGER,
    order_id INTEGER
);

-- CreateTable
CREATE TABLE "order" (
    id SERIAL PRIMARY KEY,
    status SMALLINT NOT NULL,
    total_price DECIMAL(15,2) NOT NULL,
    total_discount DECIMAL(15,2),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    customer_id INTEGER NOT NULL,
    store_id INTEGER NOT NULL,
    payment_method_id INTEGER NOT NULL,
    shipping_information_id INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE shopping_cart (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL,
    store_id INTEGER NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE shipping_information (
    id SERIAL PRIMARY KEY,
    status INTEGER NOT NULL,
    tracking_number TEXT,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    shipping_price DECIMAL(15,2) NOT NULL,
    address_id INTEGER NOT NULL,
    shipping_method_id INTEGER NOT NULL
);

CREATE UNIQUE INDEX user_email_key ON "user" (email);

CREATE UNIQUE INDEX product_variant_name_value_key ON product_variant (name, value);

CREATE UNIQUE INDEX shopping_cart_customer_id_store_id ON shopping_cart (customer_id, store_id);

ALTER TABLE "user" ADD CONSTRAINT user_managed_store_id_fkey FOREIGN KEY (managed_store_id) REFERENCES store (id) ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE user_address ADD CONSTRAINT user_address_user_id_fkey FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE store_invite ADD CONSTRAINT store_invite_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE payment_method ADD CONSTRAINT payment_method_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE shipping_method ADD CONSTRAINT shipping_method_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE product ADD CONSTRAINT product_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE product ADD CONSTRAINT product_category_id_fk FOREIGN KEY (category_id) REFERENCES product_category (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE product_item add CONSTRAINT product_item_variant_id_fk FOREIGN KEY (variant_id) REFERENCES product_variant (id) ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE product_item add CONSTRAINT product_item_product_id_fk FOREIGN KEY (product_id) REFERENCES product (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE product_item ADD CONSTRAINT product_item_discount_id_fkey FOREIGN KEY (discount_id) REFERENCES product_discount (id) ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE product_item ADD CONSTRAINT product_item_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE product_category ADD CONSTRAINT product_category_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE product_variant ADD CONSTRAINT product_variant_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE product_discount ADD CONSTRAINT product_discount_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE order_item ADD CONSTRAINT order_item_product_item_id_fkey FOREIGN KEY (product_item_id) REFERENCES product_item (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE order_item ADD CONSTRAINT order_item_shopping_cart_id_fkey FOREIGN KEY (shopping_cart_id) REFERENCES shopping_cart(id) ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE order_item ADD CONSTRAINT order_item_order_id_fkey FOREIGN KEY (order_id) REFERENCES "order" (id) ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE "order" ADD CONSTRAINT order_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES "user" (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "order" ADD CONSTRAINT order_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "order" ADD CONSTRAINT order_payment_method_id_fkey FOREIGN KEY (payment_method_id) REFERENCES payment_method (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "order" ADD CONSTRAINT order_shipping_information_id_fkey FOREIGN KEY (shipping_information_id) REFERENCES shipping_information (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE shopping_cart ADD CONSTRAINT shopping_cart_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES "user" (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE shopping_cart ADD CONSTRAINT shopping_cart_store_id_fkey FOREIGN KEY (store_id) REFERENCES store (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE shipping_information ADD CONSTRAINT shipping_information_address_id_fkey FOREIGN KEY (address_id) REFERENCES user_address (id) ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE shipping_information ADD CONSTRAINT shipping_information_shipping_method_id_fkey FOREIGN KEY (shipping_method_id) REFERENCES shipping_method (id) ON DELETE RESTRICT ON UPDATE CASCADE;
