use crate::types::MalType;

pub fn pr_str (input: &MalType) -> Result<String,String> {
    match input {
        MalType::Number(n) => Ok(n.to_string()),
        MalType::Symbol(s) => Ok(s.clone()),
        MalType::List(l) => {
                let mut texts: Vec<String> = Vec::new();
                for text in l {
                    texts.push(pr_str(text)?);
                }
                Ok(format!("({})", texts.join(" ")))
        }
        MalType::Function(_) => Ok(String::from("#<function>"))
    }
}
