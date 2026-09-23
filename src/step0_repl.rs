use std::io::{Write, stdin, stdout};

fn read(line: &str) -> String {
    line.to_string()
}

fn eval(line: &str) -> String {
    line.to_string()
}

fn print(line: &str) -> String {
    line.to_string()
}

fn rep(line: &str) -> String {
    print(&eval(&read(line)))
}


fn main() {
    loop{
        let mut line = String::new();

        print!("user>");
        stdout().flush().unwrap();

        let size = stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if size != 0 {
            let trimmed_line = line.trim();
            let result = rep(trimmed_line);
            println!("{}",result);
        } else {
            break;
        }
    }
}
