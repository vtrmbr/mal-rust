use crate::types::MalType;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type EnvRef = Rc<RefCell<Env>>;

pub struct Env {
    data: HashMap<String,MalType>,
    parent: Option<EnvRef>,
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
    pub fn root() -> EnvRef {
        let mut map: HashMap<String, MalType> = HashMap::new();
        map.insert(String::from("+"), MalType::Function(add));
        map.insert(String::from("-"), MalType::Function(subtract));
        map.insert(String::from("*"), MalType::Function(multiply));
        map.insert(String::from("/"), MalType::Function(divide));

        Rc::new(RefCell::new(Env {data:map , parent:None,}))

    }
    pub fn new(parent_env: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Env {data:HashMap::new(), parent: Some(parent_env),}))
    }
    pub fn set(&mut self, key: String, value: MalType) {
        self.data.insert(key, value);
    }
    pub fn get(&self, key:&str) -> Option<MalType> {
        if let Some(val)= self.data.get(key).cloned() {
            Some(val)
        }else if let Some(par) = &self.parent {
            par.borrow().get(key)
        }else {
            None
        }
    }
}
