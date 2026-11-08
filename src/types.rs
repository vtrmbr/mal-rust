use crate::env::EnvRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashKey { Str(String), Keyword(String) }


#[derive(Debug, Clone, PartialEq)]
pub enum MalType{
    Number(i64),
    Symbol(String),
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    HMap(HashMap<HashKey,MalType>),
    Keyword(String),
    Bool(bool),
    Nil,
    Str(String),
    Atom(Rc<RefCell<MalType>>),
    Function(fn(Vec<MalType>)-> Result<MalType, String>),
    UserFunction {
        params: Vec<String>,
        body: Box<MalType>,
        env: EnvRef,
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
