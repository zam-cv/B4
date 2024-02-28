use crate::{
    bank::Bank,
    socket::session::{Message, Response, Session},
    database::Database,
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
    #[allow(dead_code)]
    database: Database,
    #[allow(dead_code)]
    bank: Bank,
}

impl Server {
    pub fn new(bank: Bank, database: Database) -> Self {
        Server {
            sessions: HashMap::new(),
            bank,
            database,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

/// This handler is responsible for processing the `Connect` message and generating a response.
impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, connect: Connect, _: &mut Self::Context) -> Self::Result {
        log::info!("Connected: {}", connect.id);

        // if a connection already exists, it is rejected
        if self.sessions.contains_key(&connect.id) {
            log::info!("Connection already exists: {}", connect.id);
            connect.addr.do_send(Response::Stop);

            return;
        }

        self.sessions.insert(connect.id, connect.addr);
    }
}

/// This allows the `Server` actor to react to disconnection events and perform any necessary cleanup or logging.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, disconnect: Disconnect, _: &mut Self::Context) {
        log::info!("Disconnected: {}", disconnect.id);
        self.sessions.remove(&disconnect.id);
    }
}

/// This implementation defines how the `Server` actor should handle incoming messages of type `Message`.
impl Handler<Message> for Server {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Self::Context) {
        log::info!("Message from {}: {}", msg.0, msg.1);

        if let Some(addr) = self.sessions.get(&msg.0) {
            addr.do_send(Response::Text(msg.1));
        }
    }
}
