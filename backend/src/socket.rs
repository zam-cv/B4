use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String, pub String);

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

struct Server;

struct Session {
    pub id: String,
    pub addr: Addr<Server>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

/// This handler is responsible for processing the `Connect` message and generating a response.
impl Handler<Connect> for Server {
    type Result = String;

    fn handle(&mut self, _: Connect, _: &mut Self::Context) -> Self::Result {
        let id = Uuid::new_v4().to_string();
        println!("Connected: {}", id);

        id
    }
}

/// This allows the `Server` actor to react to disconnection events and perform any necessary cleanup or logging.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        println!("Disconnected: {}", msg.id);
    }
}

/// This implementation defines how the `Server` actor should handle incoming messages of type `Message`.
impl Handler<Message> for Server {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Self::Context) {
        println!("Message from {}: {}", msg.0, msg.1);
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
        let addr = ctx.address();

        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    /// The function `stopping` sends a `Disconnect` message to an address and returns `Running::Stop`.
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect {
            id: self.id.clone(),
        });
        Running::Stop
    }
}

impl Handler<Message> for Session {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
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
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
                None
            }
            ws::Message::Text(text) => {
                let response = text.to_string();
                ctx.text(text);
                Some(response)
            }
            ws::Message::Binary(bin) => {
                ctx.binary(bin);
                None
            }
            ws::Message::Pong(_) => None,
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
                None
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
                None
            }
            ws::Message::Nop => None,
        };

        if let Some(message) = message {
            self.addr.do_send(Message(self.id.clone(), message));
        }
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(
        Session {
            id: "".to_string(),
            addr: Server.start(),
        },
        &req,
        stream,
    )
}
