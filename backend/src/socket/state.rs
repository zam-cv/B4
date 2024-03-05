use crate::{
    database::Database,
    models,
    socket::session::{Response, Session},
};
use actix::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CycleData {
    pub time: i32,
}

#[derive(Deserialize)]
pub enum Request {
    Cycle(CycleData),
    CreateCropSection
}

pub struct State {
    pub id: i32,
    pub session: Addr<Session>,
    pub user: models::User,
    pub crop_sections: Vec<models::CropSection>,
}

impl State {
    // Takes the person's state from the database
    pub async fn new<'a>(
        id: i32,
        session: Addr<Session>,
        database: &'a Database,
    ) -> anyhow::Result<State> {
        if let Some(user) = database.get_user_by_id(id).await? {
            let state = State {
                id,
                user,
                session,
                crop_sections: database.get_crop_sections_by_user_id(id).await?,
            };

            state.init();
            return Ok(state);
        }

        anyhow::bail!("User not found");
    }

    pub fn init(&self) {
        self.session.do_send(Response::Str("Welcome"));
    }

    // resolve the user's cycle request
    pub async fn resolve_cycle<'a>(&self, _: CycleData, database: &'a Database) -> anyhow::Result<()> {
        self.session.do_send(Response::Str("Cycle resolved"));
        database.create_statistics(models::StatisticsSample {
            id: None,
            user_id: self.id,
            date: chrono::Utc::now().naive_utc(),
            punctuation: 5,
        }).await?;

        Ok(())
    }

    // Handles the message sent by the user
    pub async fn handle_message<'a>(&mut self, message: &String, database: &'a Database) -> anyhow::Result<()> {
        if let Ok(request) = serde_json::from_str::<Request>(message) {
            match request {
                Request::Cycle(data) => {
                    self.resolve_cycle(data, database).await?;
                }
                Request::CreateCropSection => {
                    log::info!("Create crop section");

                    // TODO: Validate
                    self.crop_sections.push(models::CropSection {
                        id: None,
                        user_id: self.id,
                        crop_type_id: None,
                        units: 0
                    });
                }
            }
        }

        Ok(())
    }

    // At the end of the session, the state is saved in the database
    pub async fn save(&self, database: &Database) -> anyhow::Result<()> {
        database.update_user(self.user.clone()).await?;
        database.upsert_crop_sections(self.crop_sections.clone()).await?;
        Ok(())
    }
}