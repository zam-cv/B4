use crate::{
    bank::{
        getters, handlers,
        sentences::{
            build_function_map, get_fragments, get_function, Argument, Sentence, SentenceFragment,
        },
    },
    config, models,
    socket::{
        context::Context,
        state::{CycleData, Duration},
    },
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

pub type Functions = Vec<(Vec<models::Function>, HashMap<String, Result<String>>)>;

type Getter = fn(&mut Context, Vec<String>) -> Result<String>;
type Handler = fn(&mut Context, Vec<String>) -> Result<()>;

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

#[derive(Serialize)]
pub struct ResolveCycleData {
    pub events: Vec<String>,
    pub tip: Option<String>,
}

pub struct Bank {
    pub getters: HashMap<&'static str, Getter>,
    pub handlers: HashMap<&'static str, Handler>,
}

impl Bank {
    pub fn new() -> Self {
        // build the function maps
        let getters = build_function_map!(
            Getter,
            getters,
            get_money,
            get_value_random,
            robar,
            get_time,
            get_personal_expenses
        );

        let handlers = build_function_map!(
            Handler,
            handlers,
            decrement_money,
            increment_money,
            drop_money,
            duplicate_money
        );

        Bank { getters, handlers }
    }

    fn get_variables_from_getters(
        &self,
        context: &mut Context,
        variables: &mut HashMap<String, Result<String>>,
        methods: &Vec<models::Function>,
    ) {
        for function in methods {
            let func = get_function(&function.function.as_str());

            if let Some(callback) = self.getters.get(func.name) {
                if let Some(key) = &function.key {
                    variables.insert(
                        key.clone(),
                        // execute the getter function
                        callback(context, func.args.iter().map(|s| s.to_string()).collect()),
                    );
                }
            }
        }
    }

    pub fn get_message_from_event(
        event: &models::Event,
        variables: &HashMap<String, Result<String>>,
    ) -> String {
        let mut message = String::new();

        // divides the content into fragments and replaces them according to their type
        for fragment in &get_fragments(&event.content) {
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

    pub fn handle_event(
        &self,
        context: &mut Context,
        methods: &Vec<models::Function>,
        variables: &HashMap<String, Result<String>>,
    ) {
        for handler in methods {
            let func = get_function(&handler.function.as_str());

            if let Some(callback) = self.handlers.get(func.name) {
                // extract the arguments from the function
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
        sentence: &Sentence,
        context: &mut Context,
        variables: &mut HashMap<String, Result<String>>,
    ) -> String {
        // obtains the values ​​of the variables for getters
        self.get_variables_from_getters(context, variables, &sentence.getters);

        // build the message
        let message = Bank::get_message_from_event(&sentence.event, &variables);

        // handle the event
        self.handle_event(context, &sentence.handlers, &variables);

        message
    }

    async fn get_message<'a, 'b>(
        &self,
        context: &mut Context<'a, 'b>,
        event: models::Event,
    ) -> Result<(
        String,
        Vec<models::Function>,
        HashMap<String, Result<String>>,
    )> {
        if let Some(id) = event.id {
            let mut variables = HashMap::new();

            let getters = context
                .database
                .get_getter_functions_by_event_id(id)
                .await?;

            let handlers = context
                .database
                .get_handler_functions_by_event_id(id)
                .await?;

            let sentence = Sentence {
                getters,
                event,
                handlers,
            };

            let message = self.handle_sentence(&sentence, context, &mut variables);
            return Ok((message, sentence.getters, variables));
        }

        anyhow::bail!("Event not found")
    }

    pub async fn handle_cycle<'a, 'b>(
        &self,
        cycle_data: &'a CycleData,
        mut context: Context<'a, 'b>,
    ) -> anyhow::Result<(ResolveCycleData, Functions)> {
        let number_of_random_events = match cycle_data.duration {
            Duration::OneMonth => 2,
            Duration::SixMonths => 4,
            Duration::OneYear => 6,
        };

        let mut events = Vec::with_capacity(number_of_random_events as usize);
        let mut functions = Vec::new();

        let random_events = context
            .database
            .get_random_events(number_of_random_events as i64)
            .await?;

        for event in random_events {
            let message = self.get_message(&mut context, event).await?;
            events.push(message.0);
            functions.push((message.1, message.2));
        }

        // default events
        let default_events = context.database.get_default_events().await?;

        for event in default_events {
            let message = self.get_message(&mut context, event).await?;
            events.push(message.0);
            functions.push((message.1, message.2))
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

        // calculate the score
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

        Ok((ResolveCycleData { events, tip }, functions))
    }
}
