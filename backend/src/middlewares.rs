use crate::routes::{Response, Status};
use crate::utils::decode_token;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use actix_web_lab::middleware::Next;

const AUTHORIZATION_HEADER: &str = "Authorization";

pub async fn auth(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if let Some(token) = req.headers().get(AUTHORIZATION_HEADER) {
        if let Ok(token) = token.to_str() {
            if let Ok(_) = decode_token(token) {
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
