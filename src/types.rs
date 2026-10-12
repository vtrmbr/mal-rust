#[derive(Debug, Clone, PartialEq)]
pub enum MalType{
    Number(i64),
    Symbol(String),
    List(Vec<MalType>),
    Function(fn(Vec<MalType>)-> Result<MalType, String>)
}
