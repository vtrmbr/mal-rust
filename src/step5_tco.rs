mod reader;
mod types;
mod printer;
mod env;
mod mal_core;

use std::io::{Write, stdin, stdout};
use crate::reader::read_str;
use crate::printer::pr_str;
use crate::types::MalType;
use crate::env::{Env,EnvRef};



fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}



fn eval(mut line: MalType,mut repl_env: EnvRef) -> Result<MalType, String> {

    loop {
        match line {
            MalType::Number(num) => return Ok(MalType::Number(num)),

            MalType::Symbol(sym) => return repl_env.borrow().get(sym.as_str()).ok_or(format!("'{}' not found", sym)),

            MalType::Bool(boo) => return Ok(MalType::Bool(boo)),

            MalType::Nil => return Ok(MalType::Nil),

            MalType::Function(fun) => return Ok(MalType::Function(fun)),

            MalType::UserFunction { params, body, env } => return Ok(MalType::UserFunction { params, body, env }),

            MalType::List(lis) => {
                match lis.first() {

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
                            Some(MalType::List(l)) => {
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
                            Some(MalType::List(l)) => {
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
    pr_str(&line)
}

fn rep(line: &str,repl_env: EnvRef) -> Result<String, String> {
    print(eval(read(line)?,repl_env)?)
}


fn err_ds(error: String) {
    println!("{}",error);
}


fn main() {
    let repl_env = Env::root();
    loop{
        let mut line = String::new();

        print!("user> ");
        stdout().flush().unwrap();

        let size = stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
        if size != 0 {
            let trimmed_line = line.trim();


            match rep(trimmed_line,repl_env.clone()) {
                Ok(n) => println!("{}",n),
                Err(n) => err_ds(n),
            }
        } else {
            break;
        }
    }
}
