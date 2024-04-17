use crate::{
    database::Database,
    models,
    socket::{
        session::{self, Session},
        state::Response,
    },
};
use actix::prelude::*;

pub struct Context<'a> {
    pub id_user: &'a i32,
    pub database: &'a Database,
    pub player: &'a mut models::Player,
    pub plots: &'a mut Vec<models::Plot>,
    pub session: &'a Addr<Session>,
}

impl<'a> Context<'a> {
    #[allow(dead_code)]
    pub fn send(&self, response: Response) -> anyhow::Result<()> {
        let response = serde_json::to_string(&response)?;
        self.session.do_send(session::Response::String(response));
        Ok(())
    }
}
