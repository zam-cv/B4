use crate::{config::CONFIG, utils};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use actix_web_lab::middleware::Next;

const AUTH_HEADER: &str = "token";
const AUTH_COOKIE: &str = "token";

macro_rules! auth {
    ($name:ident, $secret_key:expr) => {
        pub async fn $name(
            req: ServiceRequest,
            next: Next<impl MessageBody + 'static>,
        ) -> Result<ServiceResponse<impl MessageBody>, Error> {
            let token = match req.headers().get(AUTH_HEADER) {
                Some(token) => token.to_str().ok().map(|s| s.to_string()),
                None => req
                    .cookie(AUTH_COOKIE)
                    .map(|cookie| cookie.value().to_string()),
            };

            if let Some(token) = token {
                if let Ok(claims) = utils::decode_token(&$secret_key, &token) {
                    if claims.exp < chrono::Utc::now().timestamp() as usize {
                        let cookie = utils::get_cookie_with_expired_token();
                        let response = HttpResponse::Unauthorized().cookie(cookie).finish();
                        return Ok(req.into_response(response).map_into_right_body());
                    }

                    req.extensions_mut().insert(claims.id);
                    return Ok(next.call(req).await?.map_into_left_body());
                }
            }

            let response = HttpResponse::Unauthorized().finish();
            Ok(req.into_response(response).map_into_right_body())
        }
    };
}

auth!(user_auth, CONFIG.user_secret_key);
auth!(admin_auth, CONFIG.admin_secret_key);
