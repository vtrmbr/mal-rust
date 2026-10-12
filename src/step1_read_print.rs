mod reader;
mod types;
mod printer;

use std::io::{Write, stdin, stdout};
use crate::reader::read_str;
use crate::printer::pr_str;
use crate::types::MalType;


fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}

fn eval(line: Result<MalType, String>) -> Result<MalType, String> {
    line
}

fn print(possible_line: Result<MalType, String>) -> Result<String, String> {
    let line = possible_line?;
    pr_str(&line)
}

fn rep(line: &str) -> Result<String, String> {
    print(eval(read(line)))
}


fn err_ds(error: String) {
    println!("{}",error);
}


fn main() {
    loop{
        let mut line = String::new();

        print!("user> ");
        stdout().flush().unwrap();

        let size = stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if size != 0 {
            let trimmed_line = line.trim();


            match rep(trimmed_line) {
                Ok(n) => println!("{}",n),
                Err(n) => err_ds(n),
            }
        } else {
            break;
        }
    }
}
