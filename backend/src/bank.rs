use rand::Rng;
use serde::Deserialize;
use regex::Regex;
use std::{collections::HashMap, fs::File, str::FromStr};
use lazy_static::lazy_static;

lazy_static! {
    static ref GETTER_REGEX: Regex = Regex::new(r"^([A-Za-x].)\((.)\)$").unwrap();
    static ref GET_PARAMETER_REGEX: Regex = Regex::new(r"^.\$\{(.)\}.*$").unwrap();
}

const NUMBER_OF_RANDOM_EVENTS: usize = 4;
const PATH: &str = "./assets/sentences.json";

pub struct Context;

enum Getter {
    Basic(fn(Vec<String>) -> String),
    Advanced(fn(&mut Context, Vec<String>) -> String),
}

pub struct Bank {
    pub getters: HashMap<String, Getter>,
    pub sentences: Sentences,
}

pub struct SentenceBuilder;

#[derive(Deserialize)]
pub struct Sentence {
    pub getters: HashMap<String, String>,
    pub value: String,
    pub handlers: Vec<String>,
}

#[derive(Deserialize)]
pub struct Sentences {
    pub positive: Vec<Sentence>,
    pub negative: Vec<Sentence>,
}

impl Bank {
    pub fn new() -> Self {
        let file = File::open(PATH).unwrap();
        let sentences: Sentences = serde_json::from_reader(file).unwrap();
        let mut getters = HashMap::new();

        getters.insert(
            "getValueRandom".to_string(),
            Getter::Basic(|values| {
                let first = values[0].parse::<i32>().unwrap();
                let second = values[1].parse::<i32>().unwrap();

                rand::thread_rng().gen_range(first..second).to_string()
            }),
        );

        Bank { getters, sentences }
    }

    pub fn create_sentence(&self, sentence_builder: &SentenceBuilder) -> String {
        sentence_builder.build()
    }

    pub fn create_sentence2(&self) -> String {
        let mut sentences = Vec::with_capacity(NUMBER_OF_RANDOM_EVENTS);
        let events = [&self.sentences.positive, &self.sentences.negative];

        for _ in 0..NUMBER_OF_RANDOM_EVENTS {
            let number = rand::thread_rng().gen_range(0..=1);
            let event: &Vec<Sentence> = events[number];
            let sentence = &event[rand::thread_rng().gen_range(0..event.len())];
            let mut values = HashMap::new();

            for (key, value) in &sentence.getters {
                let captures = GETTER_REGEX.captures(value).unwrap();
                let name = captures.get(1).unwrap().as_str();
                let args: Vec<String> = captures.get(2).unwrap().as_str().split(',').map(|s| s.to_string()).collect();

                if let Some(handler) = self.getters.get(name) {
                    let result = match handler {
                        Getter::Basic(handler) => {
                            handler(args)
                        }
                        Getter::Advanced(handler) => {
                            let mut context = Context;
                            handler(&mut context, args)
                        }
                    };

                    values.insert(key, result);
                }
            }

            // replace

            // apply handler

            sentences.push(sentence.value.clone());
        }

        String::from("This is a sentence")
    }
}

impl SentenceBuilder {
    pub fn new() -> Self {
        SentenceBuilder
    }

    pub fn build(&self) -> String {
        "This is a sentence".to_string()
    }
}
