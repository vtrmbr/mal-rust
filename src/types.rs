use crate::env::EnvRef;

#[derive(Debug, Clone, PartialEq)]
pub enum MalType{
    Number(i64),
    Symbol(String),
    List(Vec<MalType>),
    Function(fn(Vec<MalType>)-> Result<MalType, String>),
    Bool(bool),
    Nil,
    UserFunction {
        params: Vec<String>,
        body: Box<MalType>,
        env: EnvRef,
    }
}
