use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{ Deserialize, Serialize };

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct StoreInvite {
    pub id: String,
    pub valid: bool,
    pub created_at: NaiveDateTime,
    pub store_id: i32,
}
