use crate::{ config::db::Connection, schema::user::dsl::* };

use chrono::NaiveDateTime;
use diesel::{ ExpressionMethods, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl };
use serde::{ Deserialize, Serialize };
use validator::{ Validate, ValidationError };

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub type_: i32,
    pub created_at: NaiveDateTime,
    pub first_name: String,
    pub last_login: NaiveDateTime,
    pub last_name: String,
    pub managed_store_id: Option<i32>,
    pub updated_at: NaiveDateTime,
    pub salt: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserPayload {
    #[validate(length(max = 60))]
    first_name: String,
    #[validate(length(max = 60))]
    last_name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
    invite_code: Option<String>,
    type_: i32,
}

impl User {
    // pub fn signup() {
    //     if Self::find_by_email(user_email, conn)
    // }

    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<User>> {
        user.order(id.asc()).load::<User>(conn)
    }

    pub fn find_by_email(user_email: &str, conn: &mut Connection) -> QueryResult<User> {
        user.filter(email.eq(user_email)).get_result::<User>(conn)
    }
}
