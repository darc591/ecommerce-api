use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<T>,
}

impl<T> ResponseBody<T> {
    pub fn new(data: T) -> ResponseBody<T> {
        ResponseBody {
            data: Some(data),
            error: None,
        }
    }
    pub fn new_err(error: T) -> ResponseBody<T> {
        ResponseBody {
            error: Some(error),
            data: None,
        }
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct IDResponse<T> {
    pub id: T,
}
