use crate::{
    database::Database,
    socket::server::{Command, ServerHandle},
};
use tokio::sync::broadcast::Sender;
use actix::prelude::*;
use actix_web_actors::ws;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub i32, pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub enum Response {
    #[allow(dead_code)]
    Str(&'static str),
    String(String),
    Stop,
}

pub struct Session {
    pub id: i32,
    pub srv: ServerHandle,
    pub database: Database,
    pub visitor_count: Arc<AtomicUsize>,
    pub viewer_tx: Sender<()>,
}

impl Handler<Response> for Session {
    type Result = ();

    fn handle(&mut self, msg: Response, ctx: &mut Self::Context) {
        match msg {
            Response::Str(text) => ctx.text(text),
            Response::String(text) => ctx.text(text),
            Response::Stop => ctx.stop(),
        }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    /// The function `started` in Rust sends a message to connect and sets the id based on the response
    /// or stops the context if there is an error.
    ///
    /// Arguments:
    ///
    /// * `ctx`: The `ctx` parameter in the `started` function is a mutable reference to the context of
    /// the actor. It is typically used to interact with the actor system, send messages, access the
    /// actor's address, and manage the actor's lifecycle.
    fn started(&mut self, ctx: &mut Self::Context) {
        if self
            .srv
            .tx
            .send(Command::Connect(self.id, ctx.address()))
            .is_err()
        {
            ctx.stop();
        }

        self.visitor_count.fetch_add(1, Ordering::SeqCst);
        let _ = self.viewer_tx.send(());
    }

    /// The function `stopping` sends a `Disconnect` message to an address and returns `Running::Stop`.
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        let _ = self.srv.tx.send(Command::Disconnect(self.id));
        self.visitor_count.fetch_sub(1, Ordering::SeqCst);
        let _ = self.viewer_tx.send(());

        Running::Stop
    }
}

/// This implementation is defining how the `Session` actor handles incoming WebSocket messages.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        let message = match msg {
            ws::Message::Text(text) => Some(text.to_string()),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
                None
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
                None
            }
            _ => None,
        };

        if let Some(message) = message {
            let _ = self.srv.tx.send(Command::Message(self.id, message));
        }
    }
}
