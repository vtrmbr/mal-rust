use crate::env::EnvRef;
use crate::printer::pr_str;
use std::collections::HashMap;
use crate::types::{MalType, mal_to_hash,hash_to_mal};
use crate::reader::read_str;
use std::cell::RefCell;
use std::rc::Rc;

fn join_args (inputs: &[MalType], readably: bool, separator: &str) -> Result<String,String> {
    Ok(inputs.iter().map(|x| pr_str(x,readably)).collect::<Result<Vec<String>, String>>()?.join(separator))
}

pub fn is_nil (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Nil] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn is_true (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Bool(true)] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn is_false (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Bool(false)] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn is_symbol (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Symbol(_)] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn is_keyword (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Keyword(_)] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn is_vector (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Vector(_)] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn is_map (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::HMap(_)] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn is_sequential (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Vector(_)]|[MalType::List(_)] => Ok(MalType::Bool(true)),
        _ => Ok(MalType::Bool(false)),
    }
}

pub fn symbol (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Str(s)] => Ok(MalType::Symbol(s.clone())),
        _ => Err(String::from("symbol? expects a string as it's parameter")),
    }
}

pub fn keyword (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::Str(s)] => Ok(MalType::Keyword(format!(":{}",s))),
        [k @ MalType::Keyword(_)] => Ok(k.clone()),
        _ => Err(String::from("keyword expects a string as it's parameter")),
    }
}

pub fn vector (inputs: Vec<MalType>) -> Result<MalType,String> {
    Ok(MalType::Vector(inputs))
}

pub fn hash_map (inputs: Vec<MalType>) -> Result<MalType,String> {
    if inputs.len() % 2 == 0 {
        let mut hash = HashMap::new();
        for pair in inputs.chunks(2){
            hash.insert(mal_to_hash(&pair[0])?,pair[1].clone());
        }
        Ok(MalType::HMap(hash))
    }else {
        Err(String::from("hash-map expectss an even number of elements"))
    }
}

pub fn assoc (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::HMap(map),rest @ ..] if rest.len() % 2 == 0=>{
            let mut new_map = map.clone();
            for pair in rest.chunks(2){
                new_map.insert(mal_to_hash(&pair[0])?,pair[1].clone());
            }
            Ok(MalType::HMap(new_map))
        }
        _ => Err(String::from("assoc expects an odd number of values with the first one being a hash map")),
    }
}

pub fn dissoc (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::HMap(map),rest @ ..] =>{
            let mut new_map = map.clone();
            for key in rest{
                let _ = new_map.remove(&mal_to_hash(key)?);
            }
            Ok(MalType::HMap(new_map))
        }
        _ => Err(String::from("dissoc expects a hash map as it's first parameter")),
    }
}

pub fn get (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::HMap(map),key @ MalType::Str(_)]|[MalType::HMap(map),key @ MalType::Keyword(_)] =>{
            match map.get(&mal_to_hash(key)?) {
                Some(value) => Ok(value.clone()),
                None => Ok(MalType::Nil),
            }
        }
        [MalType::Nil,MalType::Str(_)]|[MalType::Nil,MalType::Keyword(_)] => Ok(MalType::Nil),
        _ => Err(String::from("get expects a hasmap and a keyword or a string as it's parameters")),
    }
}

pub fn is_contained (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::HMap(map),key @ MalType::Str(_)]|[MalType::HMap(map),key @ MalType::Keyword(_)] =>{
            match map.get(&mal_to_hash(key)?) {
                Some(_) => Ok(MalType::Bool(true)),
                None => Ok(MalType::Bool(false)),
            }
        }
        _ => Err(String::from("contains? expects a hasmap and a keyword or a string as it's parameters")),
    }
}

pub fn keys (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::HMap(map)] =>{
            Ok(MalType::List(map.keys().cloned().map(|x| hash_to_mal(&x)).collect()))
        }
        _ => Err(String::from("keys expects a hasmap as it's parameter")),
    }
}

pub fn vals (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::HMap(map)] =>{
            Ok(MalType::List(map.values().cloned().collect()))
        }
        _ => Err(String::from("keys expects a hasmap as it's parameter")),
    }
}

pub fn print_str (inputs: Vec<MalType>) -> Result<MalType,String> {
    Ok(MalType::Str(join_args(&inputs,true," ")?))
}

pub fn cons (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [m,MalType::List(g)]|[m,MalType::Vector(g)] => {
            let mut new_list: Vec<MalType> = [m.clone()].to_vec();
            new_list.extend_from_slice(&g);
            Ok(MalType::List(new_list))
        }
        _ => Err(String::from("cons expects two parameters with the second one being a list or a vector"))
    }
}

pub fn concat (inputs: Vec<MalType>) -> Result<MalType,String> {
    let mut new_list: Vec<MalType> = Vec::new();
    for mal_type in inputs {

        match mal_type {
            MalType::List(g)|MalType::Vector(g) => new_list.extend_from_slice(&g),
            _ => return Err(String::from("concat expects only lists or vectors as it's parameters")),
        }
    }
    Ok(MalType::List(new_list))
}

pub fn vec (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(g)]|[MalType::Vector(g)] => Ok(MalType::Vector(g.to_vec())),
        _ => return Err(String::from("vec expects only a list or a vector as it's parameter")),
    }
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

pub fn nth (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(g),MalType::Number(i)]|[MalType::Vector(g),MalType::Number(i)] => {
            if g.len() as i64 > *i && *i >= 0 {
                return Ok(g[*i as usize].clone());
            }
            return Err(String::from("tried to read outside a vector's or list's bounds"));
        }
        _ => return Err(String::from("nth expects a list or a vector and a number as its parameters")),
    }
}

pub fn first (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(g)]|[MalType::Vector(g)] => return Ok(g.first().unwrap_or(&MalType::Nil).clone()),
        _ => return Err(String::from("first expects a list or vector as it's parameter")),
    }
}

pub fn rest (inputs: Vec<MalType>) -> Result<MalType,String> {
    match inputs.as_slice() {
        [MalType::List(g)]|[MalType::Vector(g)] => return Ok(MalType::List(g.get(1..).unwrap_or(&Vec::new()).to_vec())),
        _ => return Err(String::from("rest expects a list or vector as it's parameter"))
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
        [MalType::List(l),MalType::Vector(v)]|[MalType::Vector(v),MalType::List(l)] =>return Ok(MalType::Bool(v==l)),

        [MalType::HMap(map1),MalType::HMap(map2)] =>{
            if map1.len() != map2.len() {
                return Ok(MalType::Bool(false));
            }
            for key in map1.keys() {

                if let Some(value) = map2.get(key){
                    if let Ok(MalType::Bool(false)) = equal([map1[key].clone(),value.clone()].to_vec()) {
                        return Ok(MalType::Bool(false));
                    }
                } else {
                    return Ok(MalType::Bool(false));
                }
            }
            return Ok(MalType::Bool(true));
        }

        [a,b] => Ok(MalType::Bool(a==b)),
        _ => return Err(String::from("= expects two arguments"))
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
        ("cons",cons),
        ("concat",concat),
        ("vec",vec),
        ("first",first),
        ("rest",rest),
        ("nth",nth),
        ("nil?",is_nil),
        ("true?",is_true),
        ("false?",is_false),
        ("symbol?",is_symbol),
        ("symbol",symbol),
        ("keyword",keyword),
        ("keyword?",is_keyword),
        ("vector",vector),
        ("vector?",is_vector),
        ("sequential?",is_sequential),
        ("hash-map",hash_map),
        ("map?",is_map),
        ("assoc",assoc),
        ("dissoc",dissoc),
        ("get",get),
        ("contains?",is_contained),
        ("keys",keys),
        ("vals",vals),
    ];
    for &(name, function) in functions {
        env.borrow_mut().set(String::from(name), MalType::Function(function));
    }
}
