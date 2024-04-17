use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

lazy_static! {
    static ref FUNCTION_REGEX: Regex = Regex::new(r"^([A-Za-x].*)\((.*)\)$").unwrap();
    static ref VARIABLES_REGEX: Regex = Regex::new(r"\$\{.*?\}").unwrap();
    static ref IDENT_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();
    static ref STRING_REGEX: Regex = Regex::new(r#"^"([^"\\]*(\\.[^"\\]*)*)"|'([^'\\]*(\\.[^'\\]*)*)'$"#).unwrap();
}

#[derive(Deserialize)]
pub struct RawSentence {
    pub getters: HashMap<&'static str, &'static str>,
    pub value: &'static str,
    pub handlers: Vec<&'static str>,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct RawSentences {
    pub positive: Vec<RawSentence>,
    pub negative: Vec<RawSentence>,
    pub default: Vec<RawSentence>
}

#[derive(Debug)]
pub enum Argument {
    Number(&'static str),
    String(&'static str),
    Ident(&'static str),
}

impl Argument {
    pub fn to_string(&self) -> String {
        match self {
            Argument::Number(n) => n.to_string(),
            Argument::String(s) => s.to_string(),
            Argument::Ident(i) => i.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: &'static str,
    pub args: Vec<Argument>,
}

#[derive(Debug)]
pub enum SentenceFragment {
    Variable(&'static str),
    Text(&'static str),
}

#[derive(Debug)]
pub struct Sentence {
    pub getters: HashMap<&'static str, Function>,
    pub value: Vec<SentenceFragment>,
    pub handlers: Vec<Function>,
}

#[derive(Debug)]
pub struct Sentences {
    pub positive: Vec<Sentence>,
    pub negative: Vec<Sentence>,
    pub default: Vec<Sentence>
}

fn get_function(function: &'static str) -> Function {
    let captures = FUNCTION_REGEX.captures(function).unwrap();
    let name = captures.get(1).map_or("", |m| m.as_str());

    let args: Vec<Argument> = captures
        .get(2)
        .map_or("", |m| m.as_str())
        .split(',')
        .map(str::trim)
        .filter(|arg| !arg.is_empty())
        .map(|arg| {
            if NUMBER_REGEX.is_match(arg) {
                Argument::Number(arg)
            } else if STRING_REGEX.is_match(arg) {
                Argument::String(&arg[1..arg.len()-1])
            } else if IDENT_REGEX.is_match(arg) {
                Argument::Ident(arg)
            } else {
                panic!("Invalid argument: {}", arg);
            }
        })
        .collect();

    Function { name, args }
}

fn raw_sentence_to_sentence(raw: RawSentence) -> Sentence {
    let getters = raw
        .getters
        .into_iter()
        .map(|(key, value)| (key, get_function(value)))
        .collect();

    let handlers = raw.handlers.into_iter().map(get_function).collect();
    let mut value = Vec::new();

    // finds the positions of the variables and cuts the text
    let mut last = 0;
    for captures in VARIABLES_REGEX.captures_iter(raw.value) {
        let start = captures.get(0).unwrap().start();
        let end = captures.get(0).unwrap().end();

        if start > last {
            value.push(SentenceFragment::Text(&raw.value[last..start]));
        }

        value.push(SentenceFragment::Variable(&raw.value[start + 2..end - 1]));
        last = end;
    }

    if last < raw.value.len() {
        value.push(SentenceFragment::Text(&raw.value[last..]));
    }

    Sentence {
        getters,
        value,
        handlers,
    }
}

pub fn raw_sentences_to_sentences(raw: RawSentences) -> Sentences {
    let positive: Vec<Sentence> = raw
        .positive
        .into_iter()
        .map(raw_sentence_to_sentence)
        .collect();

    let negative: Vec<Sentence> = raw
        .negative
        .into_iter()
        .map(raw_sentence_to_sentence)
        .collect();

    let default: Vec<Sentence> = raw
        .default
        .into_iter()
        .map(raw_sentence_to_sentence)
        .collect();

    Sentences {
        positive,
        negative,
        default
    }
}

macro_rules! build_function_map {
    ($ty:ty ,$module:ident, $($name:ident),*) => {
        {
            let mut map: HashMap<&'static str, $ty> = std::collections::HashMap::new();
            $(
                map.insert(stringify!($name), $module::$name);
            )*
            map
        }
    };
}

pub(crate) use build_function_map;