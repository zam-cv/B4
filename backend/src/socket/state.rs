use crate::{
    bank::{core::ResolveCycleData, Bank},
    config,
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
pub enum Duration {
    #[serde(rename = "1M")]
    OneMonth,
    #[serde(rename = "6M")]
    SixMonths,
    #[serde(rename = "1Y")]
    OneYear,
}

impl Duration {
    pub fn to_months(&self) -> i32 {
        match self {
            Duration::OneMonth => 1,
            Duration::SixMonths => 6,
            Duration::OneYear => 12,
        }
    }
}

#[derive(Deserialize)]
pub struct CycleData {
    pub duration: Duration,
}

#[derive(Deserialize)]
pub struct CropData {
    pub name: String,
    pub quantity: i32,
    pub money_type: models::MoneyType,
}

#[derive(Serialize)]
pub struct ModifiedPlayer<T: Serialize> {
    pub player: models::Player,
    pub payload: T,
}

#[derive(Serialize)]
pub struct InitialData {
    pub plots: Vec<models::Plot>,
    pub top_players: Vec<String>,
    pub crops_types: Vec<models::CropType>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Cycle(CycleData),
    BuyCrop(CropData),
    ResetPlayer, // TODO: Add more
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Response {
    Init(ModifiedPlayer<InitialData>),
    CycleResolved(ModifiedPlayer<ResolveCycleData>),
    CropBought(ModifiedPlayer<Vec<models::Plot>>),
    PlayerReseted(ModifiedPlayer<Vec<models::Plot>>),
    // TODO: Add more
}

pub struct State {
    pub id: i32,
    pub connected_at: std::time::Instant,
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
        database.upsert_session(models::Session::new(id)).await?;

        if let Some(player) = database.get_player_by_user_id(id).await? {
            let state = State {
                id,
                connected_at: std::time::Instant::now(),
                player,
                session,
                plots: database.get_plots_by_player_id(id).await?,
            };

            state.init(database).await?;
            return Ok(state);
        }

        anyhow::bail!("User not found");
    }

    // Send user data at startup
    pub async fn init(&self, database: &Database) -> anyhow::Result<()> {
        self.send(Response::Init(ModifiedPlayer {
            player: self.player.clone(),
            payload: InitialData {
                plots: self.plots.clone(),
                top_players: database.get_top_players(config::TOP_PLAYERS).await?,
                crops_types: database.get_crop_types().await?,
            },
        }))
    }

    // Send a message to the user
    pub fn send(&self, response: Response) -> anyhow::Result<()> {
        let response = serde_json::to_string(&response)?;
        self.session.do_send(session::Response::String(response));
        Ok(())
    }

    pub async fn harvest<'a>(
        &mut self,
        cycle_data: &'a CycleData,
        database: &'a Database,
    ) -> anyhow::Result<()> {
        for plot in self.plots.iter_mut() {
            // search for crops that are ready to harvest
            if let Some(crop_type_id) = &plot.crop_type_id {
                if let Some(crop) = database.get_crop_type_by_name(crop_type_id.clone()).await? {
                    // to the growth of the plant is added the time of the simulation
                    plot.growth += cycle_data.duration.to_months();

                    // if the plant has grown enough it is harvested and sold
                    if plot.growth >= crop.duration {
                        let sum = crop.price * plot.quantity;
                        self.player.balance_cash +=
                            sum + (sum as f64 * config::REVENUE_PERCENTAGE) as i32;

                        plot.crop_type_id = None;
                        plot.quantity = 0;
                        plot.growth = 0;
                    }
                }
            }
        }

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

        let data = bank.handle_cycle(&cycle_data, context).await?;
        self.player.current_cycle += 1;
        self.send(Response::CycleResolved(ModifiedPlayer {
            player: self.player.clone(),
            payload: data.0,
        }))?;

        self.harvest(&cycle_data, database).await?;

        let previous_time = self.player.time;
        self.player.time += cycle_data.duration.to_months();

        // verqor and coyote collect what they lent at the end of each year
        if (previous_time / 12) < (self.player.time / 12) {
            let half = self.player.balance_cash / 2;

            if self.player.balance_verqor < 0 {
                if self.player.balance_verqor + half <= 0 {
                    self.player.balance_verqor += half;
                    self.player.balance_cash -= half;
                } else {
                    self.player.balance_cash -= self.player.balance_verqor;
                    self.player.balance_verqor = 0;
                }
            }

            if self.player.balance_coyote < 0 {
                if self.player.balance_coyote + half <= 0 {
                    self.player.balance_coyote += half;
                    self.player.balance_cash -= half;
                } else {
                    self.player.balance_cash -= self.player.balance_coyote;
                    self.player.balance_coyote = 0;
                }
            }

            // the player receives interest on the balance
            self.player.balance_verqor +=
                (self.player.balance_verqor as f64 * config::INTEREST_PERCENTAGE_VERQOR) as i32;

            self.player.balance_coyote +=
                (self.player.balance_coyote as f64 * config::INTEREST_PERCENTAGE_COYOTE) as i32;
        }

        if let Some(player_id) = self.player.id {
            let id = database
                .create_statistics(models::Statistic {
                    id: None,
                    cycle: self.player.current_cycle,
                    score: self.player.current_score,
                    player_id,
                })
                .await?;

            // save the values generated by the functions in the database
            for functions in data.1 {
                for function in functions.0 {
                    if let (Some(function_id), Some(key)) = (function.id, function.key.as_ref()) {
                        if let Some(Ok(value)) = functions.1.get(key) {
                            database
                                .add_value(models::Value {
                                    statistic_id: id,
                                    function_id,
                                    content: value.clone(),
                                })
                                .await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get_empty_plot(&mut self) -> Option<&mut models::Plot> {
        self.plots.iter_mut().find(|p| p.crop_type_id.is_none())
    }

    pub fn buy_crop_with_credit(
        &mut self,
        crop_data: CropData,
        crop: &models::CropType,
        price: i32,
    ) -> anyhow::Result<()> {
        if let Some(plot) = self.get_empty_plot() {
            plot.crop_type_id = Some(crop.name.clone());
            plot.quantity = crop_data.quantity;

            match crop_data.money_type {
                models::MoneyType::Verqor => {
                    self.player.balance_verqor -= price;
                }
                models::MoneyType::Coyote => {
                    self.player.balance_coyote -= price;
                }
                _ => {}
            };

            self.send(Response::CropBought(ModifiedPlayer {
                player: self.player.clone(),
                payload: self.plots.clone(),
            }))?;
        }

        Ok(())
    }

    pub async fn buy_crop(
        &mut self,
        crop_data: CropData,
        database: &Database,
    ) -> anyhow::Result<()> {
        if crop_data.quantity <= 0 {
            return Ok(());
        }

        if let Some(crop) = database
            .get_crop_type_by_name(crop_data.name.clone())
            .await?
        {
            let price = crop.price * crop_data.quantity;

            if let models::MoneyType::Cash = crop_data.money_type {
                if self.player.balance_cash >= price {
                    if let Some(plot) = self.get_empty_plot() {
                        plot.crop_type_id = Some(crop.name.clone());
                        plot.quantity = crop_data.quantity;
                        self.player.balance_cash -= price;

                        self.send(Response::CropBought(ModifiedPlayer {
                            player: self.player.clone(),
                            payload: self.plots.clone(),
                        }))?;
                    }
                }

                return Ok(());
            }

            match crop_data.money_type {
                models::MoneyType::Verqor => {
                    if self.player.balance_verqor - price >= config::CREDIT_LIMIT {
                        self.buy_crop_with_credit(crop_data, &crop, price)?;
                    }
                }
                models::MoneyType::Coyote => {
                    if self.player.balance_coyote - price >= config::CREDIT_LIMIT {
                        self.buy_crop_with_credit(crop_data, &crop, price)?;
                    }
                }
                _ => {}
            };
        }

        Ok(())
    }

    pub async fn reset_player(&mut self, database: &Database) -> anyhow::Result<()> {
        // reset the player
        self.player.reset();

        // reset the plots
        for plot in self.plots.iter_mut() {
            plot.crop_type_id = None;
            plot.quantity = 0;
            plot.growth = 0;
        }

        // delete the values and statistics of the player
        if let Some(player_id) = self.player.id {
            database.delete_values_by_player_id(player_id).await?;
            database.delete_statistics_by_player_id(player_id).await?;
        }

        self.send(Response::PlayerReseted(ModifiedPlayer {
            player: self.player.clone(),
            payload: self.plots.clone(),
        }))?;

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
                Request::BuyCrop(plot) => {
                    self.buy_crop(plot, database).await?;
                }
                Request::ResetPlayer => {
                    self.reset_player(database).await?;
                }
            }
        }

        Ok(())
    }

    // At the end of the session, the state is saved in the database
    pub async fn save(&mut self, database: &Database) -> anyhow::Result<()> {
        self.player.time_in_game += (self.connected_at.elapsed().as_secs() as f64 / 60.0) as f64;
        database.update_player(self.player.clone()).await?;
        database.upsert_plots(self.plots.clone()).await?;
        Ok(())
    }
}
