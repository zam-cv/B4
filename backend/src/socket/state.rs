use crate::{
    database::Database,
    models,
    socket::session::{Response, Session},
};
use actix::prelude::*;
use actix_web::error;
use serde::Deserialize;

pub struct State {
    pub id: i32,
    pub session: Addr<Session>,
    pub user: models::User,
}

#[derive(Deserialize)]
pub struct CycleData {
    pub time: i32,
}

#[derive(Deserialize)]
pub enum Request {
    Cycle(CycleData),
}

impl State {
    // Takes the person's state from the database
    pub async fn new<'a>(
        id: i32,
        session: Addr<Session>,
        database: &'a Database,
    ) -> anyhow::Result<Self> {
        if let Ok(Some(user)) = database.get_user_by_id(id).await {
            Ok(State { id, user, session })
        } else {
            Err(anyhow::anyhow!("Failed to get user"))
        }
    }

    // resolve the user's cycle request
    pub fn resolve_cycle(&self, _: CycleData) {
        self.session.do_send(Response::Str("Cycle resolved"));
    }

    // Handles the message sent by the user
    pub fn handle_message(&self, message: &String) {
        if let Ok(request) = serde_json::from_str::<Request>(message) {
            match request {
                Request::Cycle(data) => {
                    self.resolve_cycle(data);
                }
            }
        }
    }

    // At the end of the session, the state is saved in the database
    pub async fn save(&self, database: &Database) -> Result<(), error::Error> {
        database.update_user(self.user.clone()).await?;
        Ok(())
    }
}
