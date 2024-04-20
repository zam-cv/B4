use crate::{
    bank::{
        getters, handlers,
        sentences::{
            build_function_map, get_fragments, get_function, Argument, Sentence, SentenceFragment,
        },
    },
    config, models,
    socket::{context::Context, state::CycleData},
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

type Getter = fn(&mut Context, Vec<String>) -> Result<String>;
type Handler = fn(&mut Context, Vec<String>) -> Result<()>;

const NUMBER_OF_RANDOM_EVENTS: usize = 1;

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

#[derive(Serialize)]
pub struct ResolveCycleData {
    pub events: Vec<String>,
    pub player: models::Player,
    pub tip: Option<String>,
}

pub struct Bank {
    pub getters: HashMap<&'static str, Getter>,
    pub handlers: HashMap<&'static str, Handler>,
}

impl Bank {
    pub fn new() -> Self {
        let getters = build_function_map!(Getter, getters, get_money, get_value_random);
        let handlers = build_function_map!(Handler, handlers, decrement_money, increment_money);

        Bank { getters, handlers }
    }

    fn get_variables_from_getters(
        &self,
        context: &mut Context,
        variables: &mut HashMap<String, Result<String>>,
        methods: &Vec<models::Function>,
    ) {
        variables.clear();
        for function in methods {
            let func = get_function(&function.function.as_str());

            if let Some(callback) = self.getters.get(func.name) {
                if let Some(key) = &function.key {
                    variables.insert(
                        key.clone(),
                        callback(context, func.args.iter().map(|s| s.to_string()).collect()),
                    );
                }
            }
        }
    }

    fn get_message_from_sentence(
        sentence: &Sentence,
        variables: &HashMap<String, Result<String>>,
    ) -> String {
        let mut message = String::new();

        for fragment in &get_fragments(&sentence.event.content) {
            match fragment {
                SentenceFragment::Text(text) => message.push_str(text),
                SentenceFragment::Variable(variable) => {
                    if let Some(Ok(value)) = variables.get(&variable.to_string()) {
                        message.push_str(value);
                    } else {
                        message.push_str(variable);
                    }
                }
            }
        }

        message
    }

    fn handle_event(
        &self,
        context: &mut Context,
        methods: &Vec<models::Function>,
        variables: &HashMap<String, Result<String>>,
    ) {
        for handler in methods {
            let func = get_function(&handler.function.as_str());

            if let Some(callback) = self.handlers.get(func.name) {
                let args = func
                    .args
                    .iter()
                    .map(|arg| match arg {
                        Argument::Ident(ident) => {
                            if let Some(Ok(value)) = variables.get(&ident.to_string()) {
                                value.clone()
                            } else {
                                "".to_string()
                            }
                        }
                        Argument::Number(number) => number.to_string(),
                        Argument::String(string) => string.to_string(),
                    })
                    .collect();

                let _ = callback(context, args);
            }
        }
    }

    fn handle_sentence<'a>(
        &self,
        sentence: Sentence,
        context: &mut Context,
        variables: &mut HashMap<String, Result<String>>,
    ) -> String {
        // obtains the values ​​of the variables for getters
        self.get_variables_from_getters(context, variables, &sentence.getters);

        // build the message
        let message = Bank::get_message_from_sentence(&sentence, &variables);

        // handle the event
        self.handle_event(context, &sentence.handlers, &variables);

        message
    }

    async fn get_message<'a>(
        &self,
        context: &mut Context<'a>,
        event: models::Event,
        variables: &mut HashMap<String, Result<String>>,
    ) -> Result<Option<String>> {
        if let Some(id) = event.id {
            let getters = context
                .database
                .get_getter_functions_by_event_id(id)
                .await?;

            let handlers = context
                .database
                .get_handler_functions_by_event_id(id)
                .await?;

            let message = self.handle_sentence(
                Sentence {
                    getters,
                    event,
                    handlers,
                },
                context,
                variables,
            );

            return Ok(Some(message));
        }

        anyhow::bail!("Event not found")
    }

    pub async fn handle_cycle<'a>(
        &self,
        _: &'a CycleData,
        mut context: Context<'a>,
    ) -> anyhow::Result<ResolveCycleData> {
        let mut events = Vec::with_capacity(NUMBER_OF_RANDOM_EVENTS);
        let mut variables = HashMap::new();

        let random_events = context
            .database
            .get_random_events(NUMBER_OF_RANDOM_EVENTS as i64)
            .await?;

        for event in random_events {
            if let Some(message) = self
                .get_message(&mut context, event, &mut variables)
                .await?
            {
                events.push(message);
            }
        }

        // default events
        let default_events = context.database.get_default_events().await?;

        for event in default_events {
            if let Some(message) = self
                .get_message(&mut context, event, &mut variables)
                .await?
            {
                events.push(message);
            }
        }

        // choose a random tip
        let tip = if let Some(player_id) = context.player.id {
            if let Ok(Some(tip)) = context.database.get_random_tip(player_id).await {
                if let Some(tip_id) = tip.id {
                    let _ = context.database.register_tip(tip_id, player_id).await;
                }

                Some(tip.content)
            } else {
                None
            }
        } else {
            None
        };

        let max_change = context.player.max_change;
        let change = context.player.balance_cash * config::CASH_WEIGHT
            + context.player.balance_verqor * config::VERQOR_WEIGHT
            + context.player.balance_coyote * config::COYOTE_WEIGHT;

        if change > max_change {
            context.player.max_change = change;
            context.player.current_score = 1.0;
        } else {
            context.player.current_score = change as f64 / max_change as f64;
        }

        Ok(ResolveCycleData {
            events,
            player: context.player.clone(),
            tip,
        })
    }
}
