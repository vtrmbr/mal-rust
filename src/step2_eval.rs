mod reader;
mod types;
mod printer;
mod env;

use std::io::{Write, stdin, stdout};
use crate::reader::read_str;
use crate::printer::pr_str;
use crate::types::MalType;
use crate::env::Env;



fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}



fn eval(line: MalType,repl_env: &Env) -> Result<MalType, String> {

    match line {
        MalType::Number(num) => Ok(MalType::Number(num)),
        MalType::Symbol(sym) => {
            repl_env.get(sym.as_str()).cloned().ok_or(format!("'{}' not found", sym))
        }
         MalType::List(lis) => {


            let mut eval_terms = Vec::new();

            for token in lis {
                eval_terms.push(eval(token,repl_env)?);
            }


            match eval_terms.first() {
                Some(MalType::Function(fun)) => fun(eval_terms[1..].to_vec()),
                Some(t) => Err(format!("{:?} isn't a function",t)),
                None => Ok(MalType::List(Vec::new()))
            }
        }
        MalType::Function(fun) => Ok(MalType::Function(fun)),
    }
}

fn print(line: MalType) -> Result<String, String> {
    pr_str(&line)
}

fn rep(line: &str,repl_env: &Env) -> Result<String, String> {
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


            match rep(trimmed_line,&repl_env) {
                Ok(n) => println!("{}",n),
                Err(n) => err_ds(n),
            }
        } else {
            break;
        }
    }
}
