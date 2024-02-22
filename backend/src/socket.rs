use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

struct Server;

struct Session {
    pub id: String,
    pub addr: Addr<Server>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = String;

    fn handle(&mut self, _: Connect, _: &mut Self::Context) -> Self::Result {
        println!("Connected");
        Uuid::new_v4().to_string()
    }
}

impl Handler<Message> for Server {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Self::Context) {
        println!("Message: {}", msg.0);
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

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
}

impl Handler<Message> for Session {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

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
            self.addr.do_send(Message(message));
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
