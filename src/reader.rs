use crate::types::MalType;

fn is_boundary(character: char) -> bool {
    character.is_whitespace() || match character {
        ','|'~'|'['|']'|'{'|'}'|'('|')'|'\''|'^'|'@'|'`'|'"'|';' => true,
        _ => false,
    }
}

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
    let possible_token = reader.next();
    let token = possible_token.ok_or("ERROR: empty token".to_string())?;
    match token.parse::<i64>() {
        Ok(n) => Ok(MalType::Number(n)),
        Err(_) => Ok(MalType::Symbol(token)),
    }
}

pub fn read_list(reader: &mut Reader) -> Result<MalType, String> {
    let mut list: Vec<MalType> = Vec::new();
    reader.next();
    loop {
        let possible_token = reader.peek();
        let token = possible_token.ok_or("unbalanced list".to_string())?;
        match token.as_str() {
            ")" => {
                reader.next();
                break Ok(MalType::List(list));
            }
            _ => {
                list.push(read_form(reader)?);
            }
        }
    }
}



pub fn read_form(reader: &mut Reader) -> Result<MalType, String> {
    let possible_token = reader.peek();
    let token = possible_token.ok_or("ERROR: empty token".to_string())?;
    match token.as_str() {
        "(" => read_list(reader),
        _ => read_atom(reader),
    }
}


pub fn read_str(input: &str) -> Result<MalType, String> {
    let tokens = tonkenize(input);
    let mut reader = Reader { tokens, position: 0};
    read_form (&mut reader)
}


