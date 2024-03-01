use crate::{database::Database, models, socket::session::Session};
use actix::prelude::*;
use actix_web::error;

pub struct State {
    pub id: i32,
    pub session: Addr<Session>,
    pub user: models::User,
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

    // At the end of the session, the state is saved in the database
    pub async fn save(&self, database: &Database) -> Result<(), error::Error> {
        database.update_user(self.user.clone()).await?;
        Ok(())
    }
}
