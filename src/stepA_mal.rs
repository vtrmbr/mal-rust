mod reader;
mod types;
mod printer;
mod env;
mod mal_core;

use std::collections::HashMap;
use std::io::{Write, stdin, stdout};


use crate::reader::read_str;
use crate::printer::pr_str;
use crate::types::{MalType, MalError};
use crate::env::{Env,EnvRef,root_of};

fn apply_fn (f: &MalType, args: Vec<MalType>) -> Result<MalType,MalError> {
    match f {
        MalType::Function(fun,_) => Ok(fun(args)?),
        MalType::UserFunction { params, body, env, .. } => eval(*body.clone(),Env::new_with_binds(env.clone(),&params,&args)?),
        _ => Err(format!("{:?} isn't a function",f).into()),
    }
}


fn no_unquote_quasiquote (group: Vec<MalType>) -> Result<MalType,String> {
    let mut result = MalType::List(Vec::new(),Box::new(MalType::Nil));
    if !group.is_empty() {

        for elt in group.iter().rev(){

            match elt {
                MalType::List(l,_) if l.first() == Some(&MalType::Symbol("splice-unquote".to_string())) => {
                    result = MalType::List([
                        MalType::Symbol(String::from("concat")),
                        l.get(1).ok_or(String::from("splice-unquote expects at least a paramater")).cloned()?,
                        result,
                    ].to_vec(),Box::new(MalType::Nil));
                }

                _ => {
                    result = MalType::List([
                        MalType::Symbol(String::from("cons")),
                        quasiquote(elt.clone())?,
                        result,
                    ].to_vec(),Box::new(MalType::Nil));
                }
            }
        }
    }
    return Ok(result);
}

fn quasiquote (ast: MalType) -> Result<MalType,String> {
    match ast {
        MalType::List(lis,_) => {

            match lis.as_slice() {
                [MalType::Symbol(s)] if s  == "unquote" => return Err(String::from("unquote expects at least a paramater")),
                [MalType::Symbol(s), _x @ ..] if s  ==  "unquote" => return  Ok(lis[1].clone()),
                _ => return no_unquote_quasiquote(lis),
            }

            // if lis.first() == Some(&MalType::Symbol("unquote".to_string()))  {
            //     return lis.get(1).ok_or(String::from("unquote expects at least a paramater")).cloned();
            // }else {
            //     return no_unquote_quasiquote(lis);
            // }
        }

        MalType::Vector(v,_) => {
            return Ok(MalType::List([
                MalType::Symbol(String::from("vec")),
                no_unquote_quasiquote(v)?,
            ].to_vec(),Box::new(MalType::Nil)));
        }

        a @ MalType::HMap(_,_) | a @ MalType::Symbol(_)=> return Ok(MalType::List(vec![MalType::Symbol(String::from("quote")), a],Box::new(MalType::Nil))),

        a => return Ok(a)
    }
}

fn read(line: &str) -> Result<MalType, String> {
    read_str(line)
}



fn eval(mut line: MalType,mut repl_env: EnvRef) -> Result<MalType, MalError> {


    loop {

        if let Some(a) =  repl_env.borrow().get("DEBUG-EVAL") {
            if a != MalType::Nil && a != MalType::Bool(false){
                println!("EVAL: {} \t is a {:?} ",pr_str(&line,true)?,line);

            }
        }


        match line {
            MalType::Number(num) => return Ok(MalType::Number(num)),

            MalType::Str(stri) => return Ok(MalType::Str(stri)),

            MalType::Symbol(sym) => return repl_env.borrow().get(sym.as_str()).ok_or(format!("'{}' not found", sym).into()),

            MalType::Keyword(key) => return Ok(MalType::Keyword(key)),

            MalType::Bool(boo) => return Ok(MalType::Bool(boo)),

            MalType::Nil => return Ok(MalType::Nil),

            MalType::Atom(ato) => return Ok(MalType::Atom(ato)),

            MalType::Function(fun,meta) => return Ok(MalType::Function(fun,meta)),

            MalType::UserFunction { params, body, env, is_macro, meta } => return Ok(MalType::UserFunction { params, body, env, is_macro, meta }),

            MalType::Vector(vec,meta) => {
                let mut evaluated_vec = Vec::new();
                for token in vec {
                    evaluated_vec.push(eval(token,repl_env.clone())?);
                }
                return Ok(MalType::Vector(evaluated_vec,meta));
            }

            MalType::HMap(map,meta) => {
                let mut eval_map =HashMap::new();
                for pair in map.iter() {
                    eval_map.insert(pair.0.clone(),eval(pair.1.clone(),repl_env.clone())?);
                }
                return Ok(MalType::HMap(eval_map,meta));
            }

            MalType::List(lis,meta) => {
                match lis.first() {

                    Some(MalType::Symbol(s)) if s == "apply" => {
                        match lis.as_slice() {
                            [_,_,_,..] => {

                                let eval_tokens = lis[1..].iter().map(|x| eval(x.clone(),repl_env.clone())).collect::<Result<Vec<_>,MalError>>()?;
                                match eval_tokens.as_slice() {
                                    [f,middle @ ..,MalType::Vector(g,_)]|[f,middle @ ..,MalType::List(g,_)] => return apply_fn(&f,[middle,g.as_slice()].concat()),
                                    _ => return Err(MalError::from("apply expects a vector or a list as it's last paramater")),
                                }
                            }
                            _ => return Err(MalError::from("apply expects at least two parameters")),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "map" => {
                        match lis.as_slice() {
                            [_,_,_] => {

                                let eval_tokens = lis[1..].iter().map(|x| eval(x.clone(),repl_env.clone())).collect::<Result<Vec<_>,MalError>>()?;
                                match eval_tokens.as_slice() {
                                    [f,MalType::Vector(g,_)]|[f,MalType::List(g,_)] => {
                                        return Ok(MalType::List(
                                            g.iter().map(|x| apply_fn(f,[x.clone()].to_vec())).collect::<Result<Vec<_>,MalError>>()?,
                                            Box::new(MalType::Nil)
                                        ));
                                    }
                                    _ => return Err(MalError::from("map expects a vector or a list as it's second paramater")),
                                }
                            }
                            _ => return Err(MalError::from("map expects two parameters")),
                        }
                    }




                    Some(MalType::Symbol(s)) if s == "try*" => {
                        match lis.as_slice() {

                            [_,a] =>{
                                line = a.clone();
                                continue;
                            }

                            [_,a,MalType::List(l,_)] => {
                                match l.as_slice(){

                                    [MalType::Symbol(s),MalType::Symbol(b),c] if s == "catch*" => {
                                        match eval(a.clone(),repl_env.clone()) {
                                            Err(MalError::Text(err)) => {
                                                let catch_env = Env::new(repl_env);
                                                catch_env.borrow_mut().set(b.clone(),MalType::Str(err));
                                                line = c.clone();
                                                repl_env = catch_env;
                                                continue;
                                            }
                                            Err(MalError::Exception(err)) => {
                                                let catch_env = Env::new(repl_env);
                                                catch_env.borrow_mut().set(b.clone(),err);
                                                line = c.clone();
                                                repl_env = catch_env;
                                                continue;
                                            }
                                            ok => return ok,
                                        }

                                    }
                                    _ => return Err(MalError::from("try* expects its second paramater to be a list calling catch* with symbol and any value"))
                                }

                            }
                            _ => return Err(MalError::from("try* expects one or two paramaters with the second one being a list"))
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "throw" => {
                        match lis.as_slice() {
                            [_,m] => return Err(MalError::Exception(eval(m.clone(),repl_env.clone())?)),
                            _ => return Err(MalError::from("throw expects one paramater")),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "defmacro!"=> {
                        match lis.as_slice(){
                            [_,MalType::Symbol(sy),m]=> {
                                if let Ok(MalType::UserFunction { params, body, env,..}) = eval(m.clone(),repl_env.clone()){
                                    repl_env.borrow_mut().set(sy.to_string(),MalType::UserFunction { params: params.clone(), body: body.clone(), env: env.clone(), is_macro: true, meta: Box::new(MalType::Nil) } );
                                    return Ok(MalType::UserFunction { params, body, env, is_macro:true, meta: Box::new(MalType::Nil) });
                                } else {
                                    return Err(String::from("defmacro! expects a user made function as its second paramater").into());
                                }
                            }
                            _=> return Err(String::from("defmacro! expects two paramaters the first one being a symbol").into()),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "quote"=> {
                        match lis.as_slice() {
                            [_,e] => return Ok(e.clone()),
                            _ => return Err(String::from("quote expects one paramater").into()),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "quasiquote"=> {
                        match lis.as_slice() {
                            [_,a] => {
                                line = quasiquote(a.clone())?;
                                continue;
                            }
                            _ => return Err(String::from("quasiquote expects one paramater").into()),
                        }
                    }



                    Some(MalType::Symbol(s)) if s == "swap!" => {
                        match lis.as_slice() {
                            [_,_,_,..]  => {

                                let mut eval_tokens = Vec::new();
                                for token in &lis[1..] {
                                    eval_tokens.push(eval(token.clone(),repl_env.clone())?);
                                }

                                match eval_tokens.as_slice() {
                                    [MalType::Atom(a), f, rest @ ..] => {
                                        let current = a.borrow().clone();
                                        let mut args = [current].to_vec();
                                        for e in rest {
                                            args.push(e.clone());
                                        }
                                        let result = apply_fn(&f,args)?;
                                        *a.borrow_mut() = result.clone();
                                        return Ok(result);

                                    }
                                    _=> return Err(String::from("swap! expects an atom it's first paramater").into()),
                                }
                            }
                            _=> return Err(String::from("swap! needs 2 or more paramaters").into()),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "eval" => {

                        match lis.as_slice() {
                            [_,argument] => {
                                line = eval(argument.clone(),repl_env.clone())?;
                                repl_env = root_of(&repl_env);
                                continue;
                            }
                            _ => return Err(String::from("eval expects one paramater").into())
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "if" => {

                        match lis.as_slice() {
                            [_,condition,action] => {
                                match eval(condition.clone(),repl_env.clone())? {

                                    MalType::Nil | MalType::Bool(false) => return Ok(MalType::Nil),

                                    _ => {
                                        line = action.clone();
                                        continue;
                                    }
                                }
                            }
                            [_,condition,action,contingency] =>{
                                match eval(condition.clone(),repl_env.clone())? {

                                    MalType::Nil | MalType::Bool(false) => {
                                        line = contingency.clone();
                                        continue;
                                    }

                                    _ => {
                                        line = action.clone();
                                        continue;
                                    }
                                }
                            }
                            _ => return Err(String::from("malformed if conditional").into()),
                        }
                    }

                    Some(MalType::Symbol(s)) if s == "do" => {

                        if lis.len() > 2{
                            for element in &lis[1..lis.len() - 1]{
                                eval(element.clone(), repl_env.clone())?;
                            }
                        }
                        if lis.len()!=1 {
                            line = lis[lis.len()-1].clone();
                            continue;
                        }
                        return Ok(MalType::Nil)
                    }

                    Some(MalType::Symbol(s)) if s == "fn*" => {

                        match lis.as_slice() {
                            [_,MalType::List(g,_),f_body]|[_,MalType::Vector(g,_),f_body] => {
                                let mut names = Vec::new();
                                for element in g {
                                    match element {
                                        MalType::Symbol(sym) => {
                                            names.push(sym.to_string());
                                        }
                                        m => {
                                            return Err(format!("{:?} isn't a symbol",m).into());
                                        }
                                    }
                                }
                                return Ok(MalType::UserFunction { params:names , body:Box::new(f_body.clone()) , env:repl_env.clone(), is_macro:false, meta: Box::new(MalType::Nil)});
                            }
                            _ => return Err("fn* expects two paramaters with the first one being a list or a vector".into())
                        }

                    }

                    Some(MalType::Symbol(s)) if s == "def!" => {


                        // let key;
                        // let value;

                        match lis.as_slice() {
                            [_,MalType::Symbol(sym),m] => {
                                let value = eval(m.clone(),repl_env.clone())?;
                                repl_env.borrow_mut().set(sym.to_string(),value.clone());
                                return Ok(value)
                            }
                            _ => return Err(MalError::from("def! expects two parameters with the first one being a symbol"))
                        }
                        /*
                         *                        match lis.get(1) {
                         *                            Some(MalType::Symbol(sy)) => key = sy,
                         *                            Some(m) => return Err(format!("{:?} isn't a symbol",m).into()),
                         *                            None => return Err(String::from("No symbol to be defined").into()),
                    }

                    match lis.get(2) {
                    Some(m) => value = eval(m.clone(),repl_env.clone())?,
                    None => return Err(format!("No value to be a definition for {}", key).into()),
                    }

                    repl_env.borrow_mut().set(key.to_string(),value.clone());
                    return Ok(value)*/
                    }

                    Some(MalType::Symbol(s)) if s == "let*" => {
                        let new_env = Env::new(repl_env);

                        match lis.get(1) {
                            Some(MalType::List(l,_))|Some(MalType::Vector(l,_)) => {
                                let mut i = 0;
                                let mut key;
                                let mut value;

                                loop {
                                    match l.get(i) {
                                        Some(MalType::Symbol(s)) => key = s,
                                        Some(m)=> return  Err(format!("{:?} isn't a symbol",m).into()),
                                        None => break,
                                    }
                                    i+=1;

                                    match l.get(i) {
                                        Some(m) => value = eval(m.clone(),new_env.clone())?,
                                        None => return Err(format!("No value to be a definition for {}", key).into()),
                                    }
                                    i+=1;

                                    new_env.borrow_mut().set(key.to_string(),value);
                                }
                            }
                            Some(m) => return Err(format!("{:?} isn't a list",m).into()),
                            None=> return Err(format!("let* needs a binding list even if it's a empty list").into())

                        }
                        match lis.get(2) {
                            Some(m) => {
                                line = m.clone();
                                repl_env = new_env;
                                continue;
                            }
                            None => return Err(String::from("let* requires an output after the binding list").into()),
                        }
                    }
                    _ => {

                        if let Some(first)= lis.first() {
                            match eval(first.clone(),repl_env.clone())? {
                                MalType::Function(fun,_) => return Ok(fun(lis[1..].iter().map(|x| eval(x.clone(),repl_env.clone())).collect::<Result<Vec<_>, MalError>>()?)?),
                                MalType::UserFunction { params, body, env, is_macro, meta:_ } =>{

                                    if is_macro {
                                        let macro_env = Env::new_with_binds(env.clone(),&params ,&lis[1..])?;
                                        line = eval(*body,macro_env.clone())?;
                                        continue;
                                    }else {
                                        let function_env = Env::new_with_binds(env.clone(), &params ,&lis[1..].iter().map(|x| eval(x.clone(),repl_env.clone())).collect::<Result<Vec<_>, MalError>>()?)?;
                                        line = *body.clone();
                                        repl_env = function_env;
                                        continue;
                                    }
                                }
                                t => return  Err(format!("{:?} isn't a function",t).into()),
                            }
                        } else {
                            return Ok(MalType::List(Vec::new(),meta));
                        }

                    }
                }
            }
        }
    }
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
