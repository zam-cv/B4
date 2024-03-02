use crate::socket::server::Command;
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::{sync::broadcast::Sender, task};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Serialize)]
pub struct Captacion<'a> {
    pub visitor_count: &'a AtomicUsize,
}

pub struct Viewer {
    pub visitor_count: Arc<AtomicUsize>,
    pub tx: Arc<Sender<Command>>,
}

impl Handler<Message> for Viewer {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Actor for Viewer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut rx = self.tx.subscribe();
        let addr = ctx.address();
        let visitor_count = self.visitor_count.clone();

        let captacion = Captacion {
            visitor_count: &visitor_count,
        };

        // Send the initial visitor count
        if let Ok(text) = serde_json::to_string(&captacion) {
            addr.do_send(Message(text));
        }

        task::spawn(async move {
            while let Ok(cmd) = rx.recv().await {
                match cmd {
                    Command::Connect(_, _) => {
                        visitor_count.fetch_add(1, Ordering::SeqCst);
                    }
                    Command::Disconnect(_) => {
                        visitor_count.fetch_sub(1, Ordering::SeqCst);
                    }
                    _ => {
                        continue;
                    }
                }

                let captacion = Captacion {
                    visitor_count: &visitor_count,
                };

                if let Ok(text) = serde_json::to_string(&captacion) {
                    addr.do_send(Message(text));
                }
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Viewer {
    fn handle(&mut self, _: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {}
}
