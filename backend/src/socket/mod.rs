use actix::prelude::*;
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub mod server;
pub mod session;

pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::Server>>
) -> Result<HttpResponse, Error> {
    // The id was obtained from the token when authenticating
    if let Some(id) = req.extensions().get::<i32>() {
        return ws::start(
            session::Session {
                id: *id,
                addr: srv.get_ref().clone(),
            },
            &req,
            stream,
        );
    }

    Ok(HttpResponse::Unauthorized().finish())
}
