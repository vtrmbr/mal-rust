use crate::types::{MalType,hash_to_mal};

pub fn pr_str (input: &MalType,print_readably: bool) -> Result<String,String> {
    match input {

        MalType::Number(n) => Ok(n.to_string()),

        MalType::Symbol(s) => Ok(s.clone()),

        MalType::Keyword(k) => Ok(k.clone()),

        MalType::Nil => Ok(String::from("nil")),

        MalType::Bool(b)=> Ok(format!("{}",b)),

        MalType::Function(_) => Ok(String::from("#<function>")),

        MalType::UserFunction { .. } => Ok(String::from("#<function>")),

        MalType::Atom(a) => Ok(format!("(atom {})",pr_str(&a.borrow(),print_readably)?)),

        MalType::Str(s) => {
            if print_readably {

                let mut string = String::from("\"");

                for character in s.chars() {
                    match character {
                        '\\' => string.push_str("\\\\"),
                        '"' => string.push_str("\\\""),
                        '\n' => string.push_str("\\n"),
                        x => string.push(x),
                    }
                }
                string.push('"');
                Ok(string)

            } else {
                Ok(s.clone())
            }
        }

        MalType::List(l) => {
            let mut texts: Vec<String> = Vec::new();
            for text in l {
                texts.push(pr_str(text,print_readably)?);
            }
            Ok(format!("({})", texts.join(" ")))
        }

        MalType::Vector(v) => {
            let mut texts: Vec<String> = Vec::new();
            for text in v {
                texts.push(pr_str(text,print_readably)?);
            }
            Ok(format!("[{}]", texts.join(" ")))
        }
        MalType::HMap(h) => {
           Ok(format!(
                "{{{}}}",
                h.iter().map(|x| Ok(format!(
                   "{} {}",
                    pr_str(&hash_to_mal(x.0),print_readably)?,
                    pr_str(x.1,print_readably)?))
                ).collect::<Result<Vec<String>,String>>()?.join(" ")
           ))

        }
    }
}
