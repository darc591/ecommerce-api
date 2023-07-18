use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    post,
    web::{Data, Json, JsonBody, Path},
    App, HttpResponse, HttpServer, Responder,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum UserType {
    CUSTOMER,
    ADMIN,
}
#[derive(Serialize, Deserialize)]
pub struct SignupBody {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    invite_code: String,
    user_type: UserType,
}
#[derive(Serialize, Deserialize)]
pub struct IdResponse {
    id: String,
}

#[post("/auth/signup")]
pub async fn signup(body: Json<SignupBody>) -> Json<IdResponse> {
    let name = &body.first_name;

    return Json(IdResponse {
        id: "12345".to_string(),
    });
}
