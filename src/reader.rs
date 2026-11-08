
use std::collections::HashMap;

use crate::types::{MalType, mal_to_hash};


pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {

    fn next(&mut self) -> Option<String> {
        let output = self.tokens.get(self.position).cloned();
        self.position += 1;
        output
    }

    fn peek(&self) -> Option<&String> {
        self.tokens.get(self.position)
    }
}
fn is_boundary(character: char) -> bool {
    character.is_whitespace() || match character {
        ','|'~'|'['|']'|'{'|'}'|'('|')'|'\''|'^'|'@'|'`'|'"'|';' => true,
        _ => false,
    }
}

fn hash_maper(list: Vec<MalType>) -> Result<MalType,String> {

    if list.len() % 2 == 0 {
        let mut hash = HashMap::new();
        for pair in list.chunks(2){
            hash.insert(mal_to_hash(&pair[0])?,pair[1].clone());
        }
        Ok(MalType::HMap(hash))
    }else {
        Err(String::from("A hash map requires an even number of elements"))
    }
}

pub fn tonkenize(entry:&str) -> Vec<String> {

    let mut chars = entry.chars().peekable();
    let mut tokens: Vec<String> = Vec::new();

    while let Some(&c) = chars.peek() {

        let mut current_token: String = String::new();

        match c {

            _ if c.is_whitespace() || c == ',' => {
                chars.next();
                continue;
            }

            '~' => {

                current_token.push(c);
                chars.next();

                if let Some(&next_c) = chars.peek() {
                    if next_c == '@' {
                        current_token.push(next_c);
                        chars.next();
                    }
                }
            }

            '['|']'|'{'|'}'|'('|')'|'\''|'^'|'@'|'`' => {
                current_token.push(c);
                chars.next();
            }

            '"' => {
                current_token.push(c);
                chars.next();
                while let Some(&next_c) = chars.peek() {
                    current_token.push(next_c);
                    if next_c == '\\'{
                        chars.next();
                        if let Some(&new_next_c) = chars.peek(){
                            current_token.push(new_next_c);
                        }
                    } else if next_c == '"' {
                        chars.next();
                        break;
                    }
                    chars.next();
                }
            }

            ';' => {
                current_token.push(c);
                chars.next();
                while let Some(&next_c) = chars.peek(){
                    current_token.push(next_c);
                    if next_c == '\n' {
                        chars.next();
                        break;
                    }
                    chars.next();
                }
            }

            _ => {

                current_token.push(c);
                chars.next();
                while let Some(&next_c) = chars.peek(){
                    if is_boundary(next_c) {
                        break;
                    }
                    current_token.push(next_c);
                    chars.next();
                }

            }
        }

        tokens.push(current_token);
    }
    tokens
}

pub fn read_atom(reader: &mut Reader) -> Result<MalType, String> {
    let token = reader.next().ok_or("ERROR: empty token".to_string())?;

    match token.as_str() {
        "true" => Ok(MalType::Bool(true)),
        "false" => Ok(MalType::Bool(false)),
        "nil" => Ok(MalType::Nil),

        t if t.starts_with('"') => {

            let mut chars = t.chars().peekable();
            let mut string = String::new();

            chars.next();
            while chars.peek() != Some(&'"') {

                if chars.peek() == Some(&'\\') {
                    chars.next();
                    match chars.peek() {
                        Some(&'\\')|Some(&'"') => string.push(*chars.peek().unwrap()),

                        Some(&'n') => string.push('\n'),

                        _ => return Err(String::from("invalid sequece to \\")),
                    }
                } else {
                    string.push(*chars.peek().ok_or(String::from("Error"))?);
                }
                chars.next();
            }
            Ok(MalType::Str(string))
        }

        t if t.starts_with(':') => Ok(MalType::Keyword(token)),

        _ =>{
            match token.parse::<i64>() {
                Ok(n) => Ok(MalType::Number(n)),
                Err(_) => Ok(MalType::Symbol(token)),
            }
        }
    }
}

pub fn read_list(reader: &mut Reader) -> Result<MalType, String> {
    let mut list: Vec<MalType> = Vec::new();
    let closing_character;
    match reader.peek().map(|s| s.as_str()) {
        Some("(")=> closing_character = ")",
        Some("[") => closing_character = "]",
        Some("{") => closing_character = "}",
        Some(a) => return Err(format!("{} can't be a opening character for a group",a)),
        None => return Err(String::from("empty token passsed as a group opening character")),
    }
    reader.next();
    loop {
        let possible_token = reader.peek();
        let token = possible_token.ok_or("unbalanced list".to_string())?;

        if token.as_str() == closing_character {

            reader.next();
            match closing_character {
                ")" => return Ok(MalType::List(list)),
                "]" => return Ok(MalType::Vector(list)),
                "}" => return Ok(hash_maper(list)?),

                a => return Err(format!("{} can't be closing character",a))
            }
        }else{
            list.push(read_form(reader)?);
        }
    }
}




pub fn read_form(reader: &mut Reader) -> Result<MalType, String> {
    let possible_token = reader.peek();
    let token = possible_token.ok_or("ERROR: empty token".to_string())?;
    match token.as_str() {
        "("|"["|"{" => read_list(reader),
        "@" => {
            reader.next();
            Ok(MalType::List([MalType::Symbol(String::from("deref")),read_form(reader)?].to_vec()))
        }
        _ => read_atom(reader),
    }
}


pub fn read_str(input: &str) -> Result<MalType, String> {
    let tokens = tonkenize(input).into_iter().filter(|x| !x.starts_with(';')).collect::<Vec<String>>();
    if tokens.is_empty() {
        Ok(MalType::Nil)

    } else {
        let mut reader = Reader { tokens, position: 0};
        read_form (&mut reader)
    }
}


