mod reader;
mod types;
mod printer;
mod env;
mod mal_core;

use std::collections::HashMap;
use std::io::{Write, stdin, stdout};


use crate::reader::read_str;
use crate::printer::pr_str;
use crate::types::MalType;
use crate::env::{Env,EnvRef,root_of};



fn no_unquote_quasiquote (group: Vec<MalType>) -> Result<MalType,String> {
    let mut result = MalType::List(Vec::new());
    if !group.is_empty() {

        for elt in group.iter().rev(){

            match elt {
                MalType::List(l) if l.first() == Some(&MalType::Symbol("splice-unquote".to_string())) => {
                    result = MalType::List([
                        MalType::Symbol(String::from("concat")),
                        l.get(1).ok_or(String::from("splice-unquote expects at least a paramater")).cloned()?,
                        result,
                    ].to_vec());
                }

                _ => {
                    result = MalType::List([
                        MalType::Symbol(String::from("cons")),
                        quasiquote(elt.clone())?,
                        result,
                    ].to_vec());
                }
            }
        }
    }
    return Ok(result);
}

fn quasiquote (ast: MalType) -> Result<MalType,String> {
    match ast {
        MalType::List(lis) => {
            if lis.first() == Some(&MalType::Symbol("unquote".to_string()))  {
                return lis.get(1).ok_or(String::from("unquote expects at least a paramater")).cloned();
            }else {
                return no_unquote_quasiquote(lis);
            }
        }

        MalType::Vector(v) => {
            return Ok(MalType::List([
                MalType::Symbol(String::from("vec")),
                no_unquote_quasiquote(v)?,
            ].to_vec()));
        }

        a @ MalType::HMap(_) | a @ MalType::Symbol(_)=> return Ok(MalType::List(vec![MalType::Symbol(String::from("quote")), a])),

        a => return Ok(a)
    }
}

fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}



fn eval(mut line: MalType,mut repl_env: EnvRef) -> Result<MalType, String> {


    loop {

        if let Some(a) =  repl_env.borrow().get("DEBUG-EVAL") {
            if a != MalType::Nil && a != MalType::Bool(false){
                println!("EVAL: {}",pr_str(&line,true)?);
            }
        }


        match line {
            MalType::Number(num) => return Ok(MalType::Number(num)),

            MalType::Str(stri) => return Ok(MalType::Str(stri)),

            MalType::Symbol(sym) => return repl_env.borrow().get(sym.as_str()).ok_or(format!("'{}' not found", sym)),

            MalType::Keyword(key) => return Ok(MalType::Keyword(key)),

            MalType::Bool(boo) => return Ok(MalType::Bool(boo)),

            MalType::Nil => return Ok(MalType::Nil),

            MalType::Atom(ato) => return Ok(MalType::Atom(ato)),

            MalType::Function(fun) => return Ok(MalType::Function(fun)),

            MalType::UserFunction { params, body, env } => return Ok(MalType::UserFunction { params, body, env }),

            MalType::Vector(vec) => {
                let mut evaluated_vec = Vec::new();
                for token in vec {
                    evaluated_vec.push(eval(token,repl_env.clone())?);
                }
                return Ok(MalType::Vector(evaluated_vec));
            }

            MalType::HMap(map) => {
                let mut eval_map =HashMap::new();
                for pair in map.iter() {
                    eval_map.insert(pair.0.clone(),eval(pair.1.clone(),repl_env.clone())?);
                }
                return Ok(MalType::HMap(eval_map));
            }

            MalType::List(lis) => {
                match lis.first() {


                    Some(MalType::Symbol(s)) if s == "quote"=> {
                        match lis.as_slice() {
                            [_,e] => return Ok(e.clone()),
                            _ => return Err(String::from("quote expects one paramater")),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "quasiquote"=> {
                        match lis.as_slice() {
                            [_,a] => {
                                line = quasiquote(a.clone())?;
                                continue;
                            }
                            _ => return Err(String::from("quasiquote expects one paramater")),
                        }
                    }



                    Some(MalType::Symbol(s)) if s == "swap!" => {
                        match lis.as_slice() {
                            [_,_,_,..]  => {

                                let mut eval_tokens = Vec::new();
                                for token in &lis[1..] {
                                    eval_tokens.push(eval(token.clone(),repl_env.clone())?);
                                }

                                match eval_tokens.as_slice() {
                                    [MalType::Atom(a), f, rest @ ..] => {
                                        let current = a.borrow().clone();
                                        let mut args = [current].to_vec();
                                        for e in rest {
                                            args.push(e.clone());
                                        }
                                        let result;
                                        match f {
                                            MalType::Function(fun) => result = fun(args)?,

                                            MalType::UserFunction { params, body, env } => result = eval(*body.clone(),Env::new_with_binds(env.clone(),&params,&args)?)?,

                                            _ => return Err(String::from("swap! expects an function as it's first paramater")),
                                        }
                                        *a.borrow_mut() = result.clone();
                                        return Ok(result);

                                    }
                                    _=> return Err(String::from("swap! expects an atom it's first paramater")),
                                }
                            }
                            _=> return Err(String::from("swap! needs 2 or more paramaters")),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "eval" => {

                        match lis.as_slice() {
                            [_,argument] => {
                                line = eval(argument.clone(),repl_env.clone())?;
                                repl_env = root_of(&repl_env);
                                continue;
                            }
                            _ => return Err(String::from("eval expects one paramater"))
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "if" => {

                        match lis.as_slice() {
                            [_,condition,action] => {
                                match eval(condition.clone(),repl_env.clone())? {

                                    MalType::Nil | MalType::Bool(false) => return Ok(MalType::Nil),

                                    _ => {
                                        line = action.clone();
                                        continue;
                                    }
                                }
                            }
                            [_,condition,action,contingency] =>{
                                match eval(condition.clone(),repl_env.clone())? {

                                    MalType::Nil | MalType::Bool(false) => {
                                        line = contingency.clone();
                                        continue;
                                    }

                                    _ => {
                                        line = action.clone();
                                        continue;
                                    }
                                }
                            }
                            _ => return Err(String::from("malformed if conditional")),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "do" => {

                        if lis.len() > 2{
                            for element in &lis[1..lis.len() - 1]{
                                eval(element.clone(), repl_env.clone())?;
                            }
                        }
                        if lis.len()!=1 {
                            line = lis[lis.len()-1].clone();
                            continue;
                        }
                        return Ok(MalType::Nil)
                    }

                    Some(MalType::Symbol(s)) if s == "fn*" => {
                        match lis.get(1) {
                            Some(MalType::List(l))| Some(MalType::Vector(l)) => {
                                match lis.get(2) {
                                    None => return Err(String::from("Function has no body")),
                                    Some(body) => {
                                        let mut names: Vec<String> = Vec::new();
                                        for element in l {
                                            match element {
                                                MalType::Symbol(sym) => {
                                                    names.push(sym.to_string());
                                                }
                                                m => {
                                                    return Err(format!("{:?} isn't a symbol",m));
                                                }
                                            }
                                        }
                                        return Ok(MalType::UserFunction { params:names , body:Box::new(body.clone()) , env:repl_env.clone()})
                                    }
                                }
                            }
                            Some(m) => return Err(format!("{:?} isn't a list",m)),
                            None => return Err(String::from("Expected a list, found nothing")),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "def!" => {


                        let key;
                        let value;

                        match lis.get(1) {
                            Some(MalType::Symbol(sy)) => key = sy,
                            Some(m) => return Err(format!("{:?} isn't a symbol",m)),
                            None => return Err(String::from("No symbol to be defined")),
                        }

                        match lis.get(2) {
                            Some(m) => value = eval(m.clone(),repl_env.clone())?,
                            None => return Err(format!("No value to be a definition for {}", key)),
                        }

                        repl_env.borrow_mut().set(key.to_string(),value.clone());
                        return Ok(value)
                    }

                    Some(MalType::Symbol(s)) if s == "let*" => {
                        let new_env = Env::new(repl_env);

                        match lis.get(1) {
                            Some(MalType::List(l))|Some(MalType::Vector(l)) => {
                                let mut i = 0;
                                let mut key;
                                let mut value;

                                loop {
                                    match l.get(i) {
                                        Some(MalType::Symbol(s)) => key = s,
                                        Some(m)=> return  Err(format!("{:?} isn't a symbol",m)),
                                        None => break,
                                    }
                                    i+=1;

                                    match l.get(i) {
                                        Some(m) => value = eval(m.clone(),new_env.clone())?,
                                        None => return Err(format!("No value to be a definition for {}", key)),
                                    }
                                    i+=1;

                                    new_env.borrow_mut().set(key.to_string(),value);
                                }
                            }
                            Some(m) => return Err(format!("{:?} isn't a list",m)),
                            None=> return Err(format!("let* needs a binding list even if it's a empty list"))

                        }
                        match lis.get(2) {
                            Some(m) => {
                                line = m.clone();
                                repl_env = new_env;
                                continue;
                            }
                            None => return Err(String::from("let* requires an output after the binding list")),
                        }
                    }
                    _ => {

                        let mut eval_terms = Vec::new();
                        for token in lis {
                            eval_terms.push(eval(token,repl_env.clone())?);
                        }


                        match eval_terms.first() {
                            Some(MalType::Function(fun)) => return fun(eval_terms[1..].to_vec()),
                            Some(MalType::UserFunction { params, body, env }) =>{
                                let function_env = Env::new_with_binds(env.clone(), &params ,&eval_terms[1..])?;
                                line = *body.clone();
                                repl_env = function_env;
                                continue;
                            }
                            Some(t) => return  Err(format!("{:?} isn't a function",t)),
                            None => return Ok(MalType::List(Vec::new()))
                        }
                    }
                }
            }
        }
    }
}

fn print(line: MalType) -> Result<String, String> {
    pr_str(&line,true)
}

fn rep(line: &str,repl_env: EnvRef) -> Result<String, String> {
    print(eval(read(line)?,repl_env)?)
}


fn err_ds(error: String) {
    println!("{}",error);
}


fn main() {
    let repl_env = Env::root();

    let _ = rep(r#"(def! load-file (fn* (f) (eval (read-string (str "(do " (slurp f) "\nnil)")))))"#,repl_env.clone()).expect("load-file definition failed");

    let arguments: Vec<String> = std::env::args().collect();

    if let Some(file_name) = arguments.get(1) {

        repl_env.borrow_mut().set(
            String::from("*ARGV*"),
                                  MalType::List(
                                      arguments.iter().skip(2)
                                      .map(|x| MalType::Str(x.clone())).collect()
                                  )
        );

        let _ = rep(
            &format!(
                "(load-file {})",
                     pr_str(&MalType::Str(file_name.clone()),true).expect("pr_str load file call failed")
            ),
            repl_env.clone()
        )
        .inspect_err(|x| err_ds(x.to_string()));

        return;
    } else {
        repl_env.borrow_mut().set(String::from("*ARGV*"),MalType::List(Vec::new()));
    }

    loop{
        let mut line = String::new();



        print!("user> ");
        stdout().flush().unwrap();

        let size = stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
        if size != 0 {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty(){
                continue;
            }
            match rep(trimmed_line,repl_env.clone()) {
                Ok(n) => println!("{}",n),
                Err(n) => err_ds(n),
            }
        } else {
            break;
        }
    }
}
