use crate::{
    database::Database,
    models,
    socket::session::{self, Session},
};
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
pub struct CycleData {
    pub duration: i32,
}

#[derive(Deserialize)]
pub enum Request {
    Cycle(CycleData),
    CreateCropSection,
}

#[derive(Serialize)]
pub enum Response {
    Init(models::User),
    CycleResolved,
}

pub struct State {
    pub id: i32,
    pub session: Addr<Session>,
    pub user: models::User,
    pub crop_sections: Vec<models::CropSection>,
}

impl State {
    // Takes the person's state from the database
    pub async fn new(
        id: i32,
        session: Addr<Session>,
        database: &Database,
    ) -> anyhow::Result<State> {
        if let Some(user) = database.get_user_by_id(id).await? {
            let state = State {
                id,
                user,
                session,
                crop_sections: database.get_crop_sections_by_user_id(id).await?,
            };

            state.init()?;
            return Ok(state);
        }

        anyhow::bail!("User not found");
    }

    // Send a message to the user
    pub fn send(&self, response: Response) -> anyhow::Result<()> {
        self.session
            .do_send(session::Response::String(serde_json::to_string(&response)?));
        Ok(())
    }

    // Send user data at startup
    pub fn init(&self) -> anyhow::Result<()> {
        self.send(Response::Init(self.user.clone()))
    }

    // resolve the user's cycle request
    pub async fn resolve_cycle<'a>(
        &self,
        _: CycleData,
        database: &'a Database,
    ) -> anyhow::Result<()> {
        self.send(Response::CycleResolved)?;

        database
            .create_statistics(models::StatisticsSample {
                id: None,
                user_id: self.id,
                date: chrono::Utc::now().naive_utc(),
                punctuation: 5,
            })
            .await?;

        Ok(())
    }

    // Handles the message sent by the user
    pub async fn handle_message<'a>(
        &mut self,
        message: &str,
        database: &'a Database,
    ) -> anyhow::Result<()> {
        if let Ok(request) = serde_json::from_str::<Request>(message) {
            match request {
                Request::Cycle(data) => {
                    self.resolve_cycle(data, database).await?;
                }
                Request::CreateCropSection => {
                    log::debug!("Create crop section");

                    // TODO: Validate
                    self.crop_sections.push(models::CropSection {
                        id: None,
                        user_id: self.id,
                        crop_type_id: None,
                        units: 0,
                    });
                }
            }
        }

        Ok(())
    }

    // At the end of the session, the state is saved in the database
    pub async fn save(&self, database: &Database) -> anyhow::Result<()> {
        database.update_user(self.user.clone()).await?;
        database
            .upsert_crop_sections(self.crop_sections.clone())
            .await?;
        Ok(())
    }
}
