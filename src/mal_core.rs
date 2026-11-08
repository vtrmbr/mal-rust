use crate::env::EnvRef;
use crate::printer::pr_str;
use crate::types::MalType;
use crate::reader::read_str;
use std::cell::RefCell;
use std::rc::Rc;

fn join_args (inputs: &[MalType], readably: bool, separator: &str) -> Result<String,String> {
    Ok(inputs.iter().map(|x| pr_str(x,readably)).collect::<Result<Vec<String>, String>>()?.join(separator))
}

pub fn print_str (inputs: Vec<MalType>) -> Result<MalType,String> {
    Ok(MalType::Str(join_args(&inputs,true," ")?))
}

pub fn str_fn (inputs: Vec<MalType>) -> Result<MalType,String> {
    Ok(MalType::Str(join_args(&inputs,false,"")?))
}

pub fn prn (inputs: Vec<MalType>) -> Result<MalType,String> {
    println!("{}",join_args(&inputs,true," ")?);
    Ok(MalType::Nil)
}

pub fn println_fn (inputs: Vec<MalType>) -> Result<MalType,String> {
    println!("{}",join_args(&inputs,false," ")?);
    Ok(MalType::Nil)
}

pub fn read_string (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Str(s)] => read_str(s),
        _ => Err(String::from("read_str expects one parameter"))
    }
}

pub fn slurp (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Str(s)] => Ok(MalType::Str(std::fs::read_to_string(s).map_err(|e| format!("{}: {}", s, e))?)),
        _ => Err(String::from("slurp expects one parameter")),
    }
}

pub fn atom (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice(){
        [m] => Ok(MalType::Atom(Rc::new(RefCell::new(m.clone())))),
        _ => Err(String::from("atom expects one parameter")),
    }
}

pub fn is_atom (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice(){
        [MalType::Atom(_)] => Ok(MalType::Bool(true)),
        [_] => Ok(MalType::Bool(false)),
        _ => Err(String::from("atom? expects one parameter")),
    }
}

pub fn deref (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice(){
        [MalType::Atom(a)] => Ok(a.borrow().clone()),
        _ => Err(String::from("deref expects one atom parameter")),
    }
}

pub fn reset (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice(){
        [MalType::Atom(a),m] => {
            *a.borrow_mut()=m.clone();
            Ok(m.clone())
        }
        _ => Err(String::from("reset! expects an atom and another parameter in this order")),
    }
}


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
        [MalType::List(l)]|[MalType::Vector(l)] => {
            Ok(MalType::Bool(l.is_empty()))
        }
        _=> Err(String::from("empty? expects a list"))
    }
}

pub fn count (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(l)]|[MalType::Vector(l)] => Ok(MalType::Number(l.len() as i64)),
        [MalType::Nil] => Ok(MalType::Number(0)),
        _=> Err(String::from("count expects a list"))
    }
}

pub fn equal (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(l),MalType::Vector(v)]|[MalType::Vector(v),MalType::List(l)] =>Ok(MalType::Bool(v==l)),
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
        ("=",equal),
        ("<",less),
        ("<=",less_equal),
        (">",more),
        (">=",more_equal),
        ("atom",atom),
        ("atom?",is_atom),
        ("deref",deref),
        ("reset!",reset),
        ("pr-str",print_str),
        ("str",str_fn),
        ("prn",prn),
        ("println",println_fn),
        ("read-string",read_string),
        ("slurp",slurp),
        ("list",list),
        ("list?",is_list),
        ("empty?",is_empty),
        ("count",count),
    ];
    for &(name, function) in functions {
        env.borrow_mut().set(String::from(name), MalType::Function(function));
    }
}
