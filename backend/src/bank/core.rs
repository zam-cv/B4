use crate::{
    bank::{
        getters, handlers,
        sentences::{
            build_function_map, raw_sentences_to_sentences, Argument, Function, RawSentences,
            Sentence, SentenceFragment, Sentences,
        },
    },
    socket::{context::Context, state::CycleData},
};
use anyhow::Result;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;

type Getter = fn(&mut Context, Vec<String>) -> Result<String>;
type Handler = fn(&mut Context, Vec<String>) -> Result<()>;

const NUMBER_OF_RANDOM_EVENTS: usize = 1;
const SENTENCES: &str = include_str!("../../assets/sentences.json");

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

pub struct Bank {
    pub getters: HashMap<&'static str, Getter>,
    pub handlers: HashMap<&'static str, Handler>,
    pub sentences: Sentences,
}

impl Bank {
    pub fn new() -> Self {
        let sentences: RawSentences = serde_json::from_str(SENTENCES).unwrap();
        let sentences = raw_sentences_to_sentences(sentences);
        let getters =
            build_function_map!(Getter, getters, get_money, get_value_random, get_message);
        let handlers = build_function_map!(Handler, handlers, decrement_money, increment_money);

        Bank {
            sentences,
            getters,
            handlers,
        }
    }

    fn get_variables_from_getters(
        &self,
        context: &mut Context,
        map: &mut HashMap<&'static str, Result<String>>,
        methods: &HashMap<&'static str, Function>,
    ) {
        map.clear();
        for (variable_name, function) in methods {
            if let Some(callback) = self.getters.get(function.name) {
                map.insert(
                    variable_name,
                    callback(
                        context,
                        function.args.iter().map(|s| s.to_string()).collect(),
                    ),
                );
            }
        }
    }

    fn get_message_from_sentence(
        sentence: &Sentence,
        variables: &HashMap<&str, Result<String>>,
    ) -> String {
        let mut message = String::new();

        for fragment in &sentence.value {
            match fragment {
                SentenceFragment::Text(text) => message.push_str(text),
                SentenceFragment::Variable(variable) => {
                    if let Some(Ok(value)) = variables.get(variable) {
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
        sentence: &Sentence,
        variables: &HashMap<&str, Result<String>>,
    ) {
        for handler in &sentence.handlers {
            if let Some(callback) = self.handlers.get(handler.name) {
                let args: Vec<String> = handler
                    .args
                    .iter()
                    .map(|arg| {
                        match arg {
                            Argument::Ident(ident) => {
                                if let Some(Ok(value)) = variables.get(ident) {
                                    value.clone()
                                } else {
                                    "".to_string()
                                }
                            }
                            Argument::Number(number) => number.to_string(),
                            Argument::String(string) => string.to_string(),
                        }
                    })
                    .collect();

                let _ = callback(context, args);
            }
        }
    }

    fn handle_sentence<'a>(
        &self,
        sentence: &Sentence,
        context: &mut Context<'a>,
        variables: &mut HashMap<&'static str, Result<String>>,
    ) -> String {
        // obtains the values ​​of the variables for getters
        self.get_variables_from_getters(context, variables, &sentence.getters);

        // build the message
        let message = Bank::get_message_from_sentence(&sentence, &variables);

        // handle the event
        self.handle_event(context, &sentence, &variables);

        message
    }

    pub fn handle_cycle<'a>(&self, _: &'a CycleData, mut context: Context<'a>) -> String {
        let mut sentences = Vec::with_capacity(NUMBER_OF_RANDOM_EVENTS);
        let events = [&self.sentences.positive, &self.sentences.negative];
        let mut variables = HashMap::new();

        for _ in 0..NUMBER_OF_RANDOM_EVENTS {
            // choose a positive or negative event
            let index = rand::thread_rng().gen_range(0..=1);
            let event: &Vec<Sentence> = events[index];

            // choose a random sentence
            let sentence = &event[rand::thread_rng().gen_range(0..event.len())];

            let message = self.handle_sentence(&sentence, &mut context, &mut variables);
            sentences.push(message);
        }

        // default event
        for sentence in &self.sentences.default {
            let message = self.handle_sentence(&sentence, &mut context, &mut variables);
            sentences.push(message);
        }

        // choose a random tip
        let tip = rand::thread_rng().gen_range(0..self.sentences.tips.len());
        sentences.push(self.sentences.tips[tip].to_string());

        sentences.join("\n")
    }
}
