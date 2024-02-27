use crate::socket::server::{Connect, Disconnect, Server};
use actix::prelude::*;
use actix_web_actors::ws;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub i32, pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub enum Response {
    Text(String),
    Stop
}

pub struct Session {
    pub id: i32,
    pub addr: Addr<Server>,
}

impl Handler<Response> for Session {
    type Result = ();

    fn handle(&mut self, msg: Response, ctx: &mut Self::Context) {
        match msg {
            Response::Text(text) => ctx.text(text),
            Response::Stop => ctx.stop()
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
        self.addr
            .send(Connect {
                id: self.id,
                // Share the address of the session with the server
                addr: ctx.address(),
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    /// The function `stopping` sends a `Disconnect` message to an address and returns `Running::Stop`.
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id });
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
            // Send the message to the server
            self.addr.do_send(Message(self.id, message));
        }
    }
}
