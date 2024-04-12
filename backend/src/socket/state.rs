use crate::{
    bank::Bank,
    database::Database,
    models,
    socket::{
        context::Context,
        session::{self, Session},
    },
};
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
pub struct CycleData {
    pub duration: i32,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Cycle(CycleData),
    CreateCropSection,
    // TODO: Add more
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Response {
    Init(models::Player),
    CycleResolved { message: String },
    // TODO: Add more
}

pub struct State {
    pub id: i32,
    pub session: Addr<Session>,
    pub player: models::Player,
    pub plots: Vec<models::Plot>,
}

impl State {
    // Takes the person's state from the database
    pub async fn new(
        id: i32,
        session: Addr<Session>,
        database: &Database,
    ) -> anyhow::Result<State> {
        if let Some(player) = database.get_player_by_user_id(id).await? {
            let state = State {
                id,
                player,
                session,
                plots: database.get_plots_by_player_id(id).await?,
            };

            state.init()?;
            return Ok(state);
        }

        anyhow::bail!("User not found");
    }

    // Send user data at startup
    pub fn init(&self) -> anyhow::Result<()> {
        self.send(Response::Init(self.player.clone()))
    }

    // Send a message to the user
    pub fn send(&self, response: Response) -> anyhow::Result<()> {
        let response = serde_json::to_string(&response)?;
        self.session.do_send(session::Response::String(response));
        Ok(())
    }

    // resolve the user's cycle request
    pub async fn resolve_cycle<'a>(
        &mut self,
        cycle_data: CycleData,
        database: &'a Database,
        bank: &'a Bank,
    ) -> anyhow::Result<()> {
        let context = Context {
            id_user: &self.id,
            database,
            player: &mut self.player,
            plots: &mut self.plots,
            session: &self.session,
        };

        let message = bank.handle_cycle(&cycle_data, context);
        self.player.current_cycle += 1;
        self.send(Response::CycleResolved { message })?;

        database
            .create_statistics(models::StatisticsSample {
                id: None,
                cycle: self.player.current_cycle,
                score: 5,
                player_id: self.player.id.unwrap_or_default(),
            })
            .await?;

        Ok(())
    }

    // Handles the message sent by the user
    pub async fn handle_message<'a>(
        &mut self,
        message: &str,
        database: &'a Database,
        bank: &'a Bank,
    ) -> anyhow::Result<()> {
        if let Ok(request) = serde_json::from_str::<Request>(message) {
            match request {
                Request::Cycle(data) => {
                    self.resolve_cycle(data, database, bank).await?;
                }
                Request::CreateCropSection => {
                    log::debug!("Create crop section");

                    // TODO: Validate
                    self.plots.push(models::Plot {
                        id: None,
                        crop_type_id: None,
                        player_id: self.player.id.unwrap_or_default(),
                    });
                }
            }
        }

        Ok(())
    }

    // At the end of the session, the state is saved in the database
    pub async fn save(&self, database: &Database) -> anyhow::Result<()> {
        database.update_player(self.player.clone()).await?;
        database.upsert_plots(self.plots.clone()).await?;
        Ok(())
    }
}
