use crate::types::MalType;
use std::collections::HashMap;

pub struct Env {
    data: HashMap<String,MalType>,
}

fn add (inputs: Vec<MalType>) -> Result<MalType, String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Number(a+b)),
        _ => Err(String::from("Token isn't a number")),
    }
}

fn subtract (inputs: Vec<MalType>) -> Result<MalType, String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Number(a-b)),
        _ => Err(String::from("Token isn't a number")),
    }
}

fn multiply (inputs: Vec<MalType>) -> Result<MalType, String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Number(a*b)),
        _ => Err(String::from("Token isn't a number")),
    }
}

fn divide (inputs: Vec<MalType>) -> Result<MalType, String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => {

            match a.checked_div(*b) {
                Some(value) => Ok(MalType::Number(value)),
                None => Err(String::from("Divided by zero or caused signed overflow")),
            }
        }
        _ => Err(String::from("Token isn't a number")),
    }
}

impl Env {
    pub fn new() -> Env {
        let mut map: HashMap<String, MalType> = HashMap::new();
        map.insert(String::from("+"), MalType::Function(add));
        map.insert(String::from("-"), MalType::Function(subtract));
        map.insert(String::from("*"), MalType::Function(multiply));
        map.insert(String::from("/"), MalType::Function(divide));
        Env { data:map }
    }
    pub fn set(&mut self, key: String, value:MalType) {
        self.data.insert(key, value);
    }
    pub fn get(&self, key:&str) -> Option<&MalType> {
        self.data.get(key)
    }
}
