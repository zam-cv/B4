use crate::database::Database;
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub mod server;
pub mod session;
pub mod state;

pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<server::ServerHandle>,
    database: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    // The id was obtained from the token when authenticating
    if let Some(id) = req.extensions().get::<i32>() {
        return ws::start(
            session::Session {
                id: *id,
                srv: srv.get_ref().clone(),
                database: database.get_ref().clone(),
            },
            &req,
            stream,
        );
    }

    Ok(HttpResponse::Unauthorized().finish())
}
