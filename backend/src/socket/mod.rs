use crate::{database::Database, socket::server::Command};
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::sync::{atomic::AtomicUsize, Arc};
use tokio::sync::broadcast::Sender;

pub mod server;
pub mod session;
pub mod state;
pub mod viewer;

pub async fn server_index(
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

// it is responsible for handling the connection of the spectators
pub async fn viewer_index(
    req: HttpRequest,
    stream: web::Payload,
    tx: web::Data<Sender<Command>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        viewer::Viewer {
            visitor_count: Arc::new(AtomicUsize::new(0)),
            tx: tx.into_inner(),
        },
        &req,
        stream,
    )
}
