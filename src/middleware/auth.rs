use actix_web::FromRequest;
use futures_util::future::{err, ok, Ready};

use crate::{error::ServiceError, utils::auth::TokenClaims};

pub struct AuthMiddleware {
    pub user: TokenClaims,
}

impl FromRequest for AuthMiddleware {
    type Error = ServiceError;
    type Future = Ready<Result<AuthMiddleware, ServiceError>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth = req.headers().get("authorization");

        match auth {
            Some(_) => {
                let token = auth.unwrap().to_str().unwrap().split(' ').last().unwrap();
                match TokenClaims::decode_token(token) {
                    Ok(token_claims) => ok(AuthMiddleware { user: token_claims }),
                    Err(_) => err(ServiceError::Unauthorized {
                        error_message: "Invalid token!".to_string(),
                    }),
                }
            }
            None => err(ServiceError::Unauthorized {
                error_message: "Authorization header is required".to_string(),
            }),
        }
    }
}
