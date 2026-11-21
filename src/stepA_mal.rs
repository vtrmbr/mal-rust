mod reader;
mod types;
mod printer;
mod evaluator;
mod env;
mod mal_core;


use std::io::{Write, stdin, stdout};


use crate::reader::read_str;
use crate::printer::pr_str;
use crate::evaluator::eval_tok;
use crate::types::{MalType, MalError};
use crate::env::{Env,EnvRef};






fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}



fn eval(line: MalType,repl_env: EnvRef) -> Result<MalType, MalError> {
    eval_tok(line,repl_env)
}

fn print(line: MalType) -> Result<String, String> {
    pr_str(&line,true)
}

fn rep(line: &str,repl_env: EnvRef) -> Result<String, MalError> {
    Ok(print(eval(read(line)?,repl_env)?)?)
}


fn err_ds(error: &MalError) {

    match error {
        MalError::Text(t) => println!("{}",t),
        MalError::Exception(m) => {
            if let Ok(err) = pr_str(m,true){
                println!("Exception.{}",err);
            }else {
                println!("Exception.{:?}", m);
            }
        }
    }


}


fn main() {
    let repl_env = Env::root();

    let _ = rep(r#"(def! load-file (fn* (f) (eval (read-string (str "(do " (slurp f) "\nnil)")))))"#,repl_env.clone()).expect("load-file definition failed");

    let _ = rep(r#"(def! *host-language* "rust")"#,repl_env.clone()).expect("host-language definition failed");

    let _ = rep(r#"(def! not(fn* (a) (if a false true)))"#,repl_env.clone()).expect("not definition failed");

    let _ = rep(r#"(def! throw (fn* (x) (throw x)))"#,repl_env.clone()).expect("throw definition failed");

    let _ = rep(r#"(defmacro! cond (fn* (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw "odd number of forms to cond")) (cons 'cond (rest (rest xs)))))))"#,repl_env.clone()).expect("cond definition failed");




    let arguments: Vec<String> = std::env::args().collect();

    if let Some(file_name) = arguments.get(1) {

        repl_env.borrow_mut().set(
            String::from("*ARGV*"),
                                MalType::List(
                                    arguments.iter().skip(2).map(|x| MalType::Str(x.clone())).collect(),
                                    Box::new(MalType::Nil)
                                )
        );

        let _ = rep(r#"(if (= (first *ARGV*) "DEBUG") (def! DEBUG-EVAL true) )"#,repl_env.clone()).expect("debug mode verification failed");

        let _ = rep(
            &format!(
                "(load-file {})",
                     pr_str(&MalType::Str(file_name.clone()),true).expect("pr_str load file call failed")
            ),
            repl_env.clone()
        )
        .inspect_err(|x| err_ds(x));


        return;
    } else {
        repl_env.borrow_mut().set(String::from("*ARGV*"),MalType::List(Vec::new(),Box::new(MalType::Nil)));
        let _ = rep(r#"(println (str "Mal [" *host-language* "]"))"#,repl_env.clone()).expect("host language print failed");
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
                Err(n) => err_ds(&n),
            }
        } else {
            break;
        }
    }
}
