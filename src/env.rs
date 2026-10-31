use crate::types::MalType;
use crate::mal_core::register_env;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type EnvRef = Rc<RefCell<Env>>;

pub struct Env {
    data: HashMap<String,MalType>,
    parent: Option<EnvRef>,
}

impl Env {
    pub fn root() -> EnvRef {
        let env = Rc::new(RefCell::new(Env {data:HashMap::new(), parent:None,}));

        register_env(env.clone());

        env
    }
    pub fn new(parent_env: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Env {data:HashMap::new(), parent: Some(parent_env),}))
    }
    pub fn set(&mut self, key: String, value: MalType) {
        self.data.insert(key, value);
    }
    pub fn new_with_binds(parent_env: EnvRef,binds: &[String],exprs: &[MalType]) -> Result<EnvRef,String> {
        if binds.len() != exprs.len() {
            Err(format!("Expected {} parameters, got {}",binds.len(),exprs.len()))
        } else {
            let env = Env::new(parent_env);
            for (index,parameter) in binds.iter().enumerate() {
                env.borrow_mut().set(parameter.to_string(),exprs[index].clone());
            }
            Ok(env)
        }
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

impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#<env>")
    }
}

impl PartialEq for Env {
    fn eq(&self, _other: &Env) -> bool {
        false
    }
}
