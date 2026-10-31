use crate::env::EnvRef;
use crate::printer::pr_str;
use crate::types::MalType;

pub fn add (inputs: Vec<MalType>) -> Result<MalType, String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Number(a+b)),
        _ => Err(String::from("Token isn't a number")),
    }
}

pub fn subtract (inputs: Vec<MalType>) -> Result<MalType, String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Number(a-b)),
        _ => Err(String::from("Token isn't a number")),
    }
}

pub fn multiply (inputs: Vec<MalType>) -> Result<MalType, String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Number(a*b)),
        _ => Err(String::from("Token isn't a number")),
    }
}

pub fn divide (inputs: Vec<MalType>) -> Result<MalType, String> {
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

pub fn prn (inputs: Vec<MalType>) -> Result<MalType,String> {
    let text = match inputs.as_slice() {
        [a] => pr_str(a)?,
        _ => return Err(String::from("prn expects an argument")),
    };
    println!("{}", text);
    Ok(MalType::Nil)
}

pub fn list (inputs: Vec<MalType>) -> Result<MalType,String> {
    Ok(MalType::List(inputs))
}

pub fn is_list (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(_)] => Ok(MalType::Bool(true)),
        [_] => Ok(MalType::Bool(false)),
        _ => Err(String::from("list? expects an argument")),
    }
}

pub fn is_empty (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(l)] => {
            Ok(MalType::Bool(l.is_empty()))
        }
        _=> Err(String::from("empty? expects a list"))
    }
}

pub fn count (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(l)] => Ok(MalType::Number(l.len() as i64)),
        [MalType::Nil] => Ok(MalType::Number(0)),
        _=> Err(String::from("count expects a list"))
    }
}

pub fn equal (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [a,b] => Ok(MalType::Bool(a==b)),
        _ => Err(String::from("= expects two arguments"))
    }
}

pub fn less (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Bool(a<b)),
        _ => Err(String::from("< expected two parameters"))
    }
}

pub fn less_equal (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Bool(a<=b)),
        _ => Err(String::from("<= expected two parameters"))
    }
}

pub fn more (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Bool(a>b)),
        _ => Err(String::from("> expected two parameters"))
    }
}

pub fn more_equal (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Number(a),MalType::Number(b)] => Ok(MalType::Bool(a>=b)),
        _ => Err(String::from(">= expected two parameters"))
    }
}

pub fn register_env (env: EnvRef) {
    let functions: &[(&str, fn(Vec<MalType>) -> Result<MalType, String>)] = &[
        ("+", add),
        ("-", subtract),
        ("*", multiply),
        ("/", divide),
        ("prn",prn),
        ("list",list),
        ("list?",is_list),
        ("empty?",is_empty),
        ("count",count),
        ("=",equal),
        ("<",less),
        ("<=",less_equal),
        (">",more),
        (">=",more_equal),
    ];
    for &(name, function) in functions {
        env.borrow_mut().set(String::from(name), MalType::Function(function));
    }
}
