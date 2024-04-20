use crate::models;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

lazy_static! {
    static ref FUNCTION_REGEX: Regex = Regex::new(r"^([A-Za-x].*)\((.*)\)$").unwrap();
    static ref VARIABLES_REGEX: Regex = Regex::new(r"\$\{.*?\}").unwrap();
    static ref IDENT_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();
    static ref STRING_REGEX: Regex =
        Regex::new(r#"^"([^"\\]*(\\.[^"\\]*)*)"|'([^'\\]*(\\.[^'\\]*)*)'$"#).unwrap();
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
    pub default: Vec<RawSentence>,
}

#[derive(Debug)]
pub enum Argument<'a> {
    Number(&'a str),
    String(&'a str),
    Ident(&'a str),
}

impl<'a> Argument<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Argument::Number(n) => n.to_string(),
            Argument::String(s) => s.to_string(),
            Argument::Ident(i) => i.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub args: Vec<Argument<'a>>,
}

#[derive(Debug)]
pub enum SentenceFragment<'a> {
    Variable(&'a str),
    Text(&'a str),
}

#[derive(Debug)]
pub struct Sentence {
    pub getters: Vec<models::Function>,
    pub event: models::Event,
    pub handlers: Vec<models::Function>,
}

pub fn get_function<'a>(function: &'a str) -> Function<'a> {
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
                Argument::String(&arg[1..arg.len() - 1])
            } else if IDENT_REGEX.is_match(arg) {
                Argument::Ident(arg)
            } else {
                panic!("Invalid argument: {}", arg);
            }
        })
        .collect();

    Function { name, args }
}

pub fn get_fragments<'a>(value: &'a str) -> Vec<SentenceFragment> {
    let mut fragments = Vec::new();
    let mut last = 0;

    for captures in VARIABLES_REGEX.captures_iter(value) {
        if let (Some(start), Some(end)) = (captures.get(0), captures.get(0)) {
            let start = start.start();
            let end = end.end();

            if start > last {
                fragments.push(SentenceFragment::Text(&value[last..start]));
            }

            fragments.push(SentenceFragment::Variable(&value[start + 2..end - 1]));
            last = end;
        }
    }

    if last < value.len() {
        fragments.push(SentenceFragment::Text(&value[last..]));
    }

    fragments
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
