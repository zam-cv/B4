use crate::{
    routes::{Response, Status},
    utils::decode_token,
    CONFIG,
};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use actix_web_lab::middleware::Next;

const AUTHORIZATION_HEADER: &str = "Authorization";

macro_rules! auth {
    ($name:ident, $secret_key:expr) => {
        pub async fn $name(
            req: ServiceRequest,
            next: Next<impl MessageBody + 'static>,
        ) -> Result<ServiceResponse<impl MessageBody>, Error> {
            if let Some(token) = req.headers().get(AUTHORIZATION_HEADER) {
                if let Ok(token) = token.to_str() {
                    if let Ok(claims) = decode_token(&$secret_key, token) {
                        if claims.exp < chrono::Utc::now().timestamp() as usize {
                            let response = HttpResponse::Unauthorized().json(Response {
                                message: Status::Incorrect("Token has expired"),
                                payload: None::<()>,
                            });

                            return Ok(req.into_response(response).map_into_right_body());
                        }

                        req.extensions_mut().insert(claims.id);
                        return Ok(next.call(req).await?.map_into_left_body());
                    }
                }
            }

            let response = HttpResponse::Unauthorized().json(Response {
                message: Status::Incorrect("Unauthorized"),
                payload: None::<()>,
            });

            Ok(req.into_response(response).map_into_right_body())
        }
    };
}

auth!(user_auth, CONFIG.user_secret_key);
auth!(admin_auth, CONFIG.admin_secret_key);