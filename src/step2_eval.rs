mod reader;
mod types;
mod printer;
mod env;
mod mal_core;
mod evaluator;

use std::io::{Write, stdin, stdout};
use crate::reader::read_str;
use crate::printer::pr_str;
use crate::types::{MalError,MalType};
use crate::env::{Env, EnvRef};



fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}



fn eval(line: MalType,repl_env: EnvRef) -> Result<MalType, String> {

    match line {

        MalType::Nil|MalType::Bool(_)|MalType::HMap(_,_)|MalType::Atom(_)|MalType::Vector(_,_)|MalType::Keyword(_)|MalType::Str(_)|MalType::UserFunction {..} => return Err(String::from("type not implemented in this version")),

        MalType::Number(num) => Ok(MalType::Number(num)),
        MalType::Symbol(sym) => {
            repl_env.borrow().get(sym.as_str()).clone().ok_or(format!("'{}' not found", sym))
        }
         MalType::List(lis,_) => {


            let mut eval_terms = Vec::new();

            for token in lis {
                eval_terms.push(eval(token,repl_env.clone())?);
            }


            match eval_terms.first() {
                Some(MalType::Function(fun,_)) => {
                    let a = fun(eval_terms[1..].to_vec());
                    match a {
                        Ok(a) => return Ok(a),
                        Err(MalError::Text(t)) => return Err(t),
                        Err(MalError::Exception(excep)) => return Err(pr_str(&excep,true)?),
                    }
                }
                Some(t) => Err(format!("{:?} isn't a function",t)),
                None => Ok(MalType::List(Vec::new(),Box::new(MalType::Nil)))
            }
        }
        MalType::Function(fun,m) => Ok(MalType::Function(fun,m)),

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
