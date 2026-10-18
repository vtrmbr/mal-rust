mod reader;
mod types;
mod printer;
mod env;

use std::io::{Write, stdin, stdout};
use crate::reader::read_str;
use crate::printer::pr_str;
use crate::types::MalType;
use crate::env::{Env,EnvRef};



fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}



fn eval(line: MalType,repl_env: EnvRef) -> Result<MalType, String> {

    match line {
        MalType::Number(num) => Ok(MalType::Number(num)),
        MalType::Symbol(sym) => {
            repl_env.borrow().get(sym.as_str()).ok_or(format!("'{}' not found", sym))
        }
        MalType::List(lis) => {
            match lis.first() {

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
                    Ok(value)
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
                        Some(m) => eval(m.clone(),new_env),
                        None => Err(String::from("let* requires an output after the binding list")),
                    }
                }
                _ => {

                    let mut eval_terms = Vec::new();
                    for token in lis {
                        eval_terms.push(eval(token,repl_env.clone())?);
                    }


                    match eval_terms.first() {
                        Some(MalType::Function(fun)) => fun(eval_terms[1..].to_vec()),
                        Some(t) => Err(format!("{:?} isn't a function",t)),
                        None => Ok(MalType::List(Vec::new()))
                    }

                }
            }
        }
        MalType::Function(fun) => Ok(MalType::Function(fun)),
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
