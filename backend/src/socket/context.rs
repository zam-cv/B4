use crate::{
    database::Database,
    models,
    socket::{
        session::{self, Session},
        state::{Response, CycleData},
    },
};
use actix::prelude::*;

pub struct Context<'a, 'b> {
    pub id_user: &'a i32,
    pub database: &'a Database,
    pub player: &'a mut models::Player,
    pub plots: &'a mut Vec<models::Plot>,
    pub session: &'a Addr<Session>,
    pub cycle_data: &'b CycleData,
}

impl<'a, 'b> Context<'a, 'b> {
    #[allow(dead_code)]
    pub fn send(&self, response: Response) -> anyhow::Result<()> {
        let response = serde_json::to_string(&response)?;
        self.session.do_send(session::Response::String(response));
        Ok(())
    }
}
