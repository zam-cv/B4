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
pub struct Captacion {
    pub visitor_count: usize,
}

pub struct Viewer {
    pub visitor_count: Arc<AtomicUsize>,
    pub tx: Arc<Sender<()>>,
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

        let captacion = Captacion {
            visitor_count: self.visitor_count.load(Ordering::SeqCst)
        };

        // Send the initial visitor count
        if let Ok(text) = serde_json::to_string(&captacion) {
            addr.do_send(Message(text));
        }

        let visitor_count = self.visitor_count.clone();

        // Listen for visitor count updates
        task::spawn(async move {
            while let Ok(_) = rx.recv().await {
                let captacion = Captacion {
                    visitor_count: visitor_count.load(Ordering::SeqCst)
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
