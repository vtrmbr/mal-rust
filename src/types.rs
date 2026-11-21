use crate::env::EnvRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashKey { Str(String), Keyword(String) }

#[derive(Debug)]
pub enum MalError{
    Text(String),
    Exception(MalType),
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq)]
pub enum MalType{
    Number(i64),
    Symbol(String),
    List(Vec<MalType>,Box<MalType>),
    Vector(Vec<MalType>,Box<MalType>),
    HMap(HashMap<HashKey,MalType>,Box<MalType>),
    Keyword(String),
    Bool(bool),
    Nil,
    Str(String),
    Atom(Rc<RefCell<MalType>>),
    Function(fn(Vec<MalType>)-> Result<MalType, String>,Box<MalType>),
    UserFunction {
        params: Vec<String>,
        body: Box<MalType>,
        env: EnvRef,
        is_macro: bool,
        meta: Box<MalType>,
    }
}

impl From<String> for MalType {
    fn from(s: String) -> Self {
        MalType::Str(s)
    }
}

impl From<&str> for MalType {
    fn from(s: &str) -> Self {
        MalType::Str(String::from(s))
    }
}

impl From<String> for MalError {
    fn from(s: String) -> Self {
        MalError::Text(s)
    }
}

impl From<&str> for MalError {
    fn from(s: &str) -> Self {
        MalError::Text(String::from(s))
    }
}

pub fn mal_to_hash (mal: &MalType) -> Result<HashKey,String> {
    match mal {
        MalType::Str(s) => Ok(HashKey::Str(s.to_string())),
        MalType::Keyword(k) => Ok(HashKey::Keyword(k.to_string())),
        _ => Err(String::from("Only string and keyword Maltypes can be a HashKeys")),
    }
}

pub fn hash_to_mal (hash: &HashKey) -> MalType {
    match hash {
        HashKey::Str(s) => MalType::Str(s.to_string()),
        HashKey::Keyword(k) => MalType::Keyword(k.to_string()),
    }
}
