use crate::{
    bank::{Bank, SentenceBuilder},
    socket::session::{Message, Response, Session},
};
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: i32,
    pub addr: Addr<Session>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}

pub struct Server {
    sessions: HashMap<i32, Addr<Session>>,
    bank: Bank,
}

impl Server {
    pub fn new(bank: Bank) -> Self {
        Server {
            sessions: HashMap::new(),
            bank,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

/// This handler is responsible for processing the `Connect` message and generating a response.
impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, conn: Connect, _: &mut Self::Context) -> Self::Result {
        println!("Connected: {}", conn.id);
        self.sessions.insert(conn.id, conn.addr);
    }
}

/// This allows the `Server` actor to react to disconnection events and perform any necessary cleanup or logging.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, disc: Disconnect, _: &mut Self::Context) {
        println!("Disconnected: {}", disc.id);
        self.sessions.remove(&disc.id);
    }
}

/// This implementation defines how the `Server` actor should handle incoming messages of type `Message`.
impl Handler<Message> for Server {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Self::Context) {
        println!("Message from {}: {}", msg.0, msg.1);

        if let Some(addr) = self.sessions.get(&msg.0) {
            let sentence_builder = SentenceBuilder::new();
            let sentence = self.bank.create_sentence(&sentence_builder);
            addr.do_send(Response(sentence));
        }
    }
}
