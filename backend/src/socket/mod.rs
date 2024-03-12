use crate::database::Database;
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
    visitor_count: web::Data<Arc<AtomicUsize>>,
    viewer_tx: web::Data<Sender<()>>,
    database: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    // The id was obtained from the token when authenticating
    if let Some(id) = req.extensions().get::<i32>() {
        return ws::start(
            session::Session {
                id: *id,
                visitor_count: visitor_count.get_ref().clone(),
                srv: srv.get_ref().clone(),
                database: database.get_ref().clone(),
                viewer_tx: viewer_tx.get_ref().clone(),
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
    visitor_count: web::Data<Arc<AtomicUsize>>,
    tx: web::Data<Sender<()>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        viewer::Viewer {
            visitor_count: visitor_count.get_ref().clone(),
            tx: tx.into_inner(),
        },
        &req,
        stream,
    )
}
