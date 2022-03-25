use crate::lexer::{Lexer, Token};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    name: Option<Vec<char>>,
    value: Vec<char>,
}

#[derive(Debug)]
pub enum ResultType {
    Exit(i32),
    IfResult(bool),
    NotFound(String),
}

pub struct Func {
    name: String,
    runner: Box<dyn CloneableFn>,
    arg_len: usize,
}

impl Func {
    pub fn run(
        self,
        args: Vec<FunctionArgument>,
        params: HashMap<String, Vec<char>>,
    ) -> Result<ResultType, Box<dyn Error>> {
        (self.runner)(self.name, args, params)
    }
}
// https://github.com/rust-lang/rust/issues/41517 waitiung for trait alias
//trait Runnable = Fn(Vec<FunctionArgument>, HashMap<String, Vec<char>>) -> Result<ResultType, Box<dyn Error>>;

trait CloneableFn:
    Fn(String, Vec<FunctionArgument>, HashMap<String, Vec<char>>) -> Result<ResultType, Box<dyn Error>>
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn>
    where
        Self: 'a;
}

impl<F> CloneableFn for F
where
    F: Fn(
            String,
            Vec<FunctionArgument>,
            HashMap<String, Vec<char>>,
        ) -> Result<ResultType, Box<dyn Error>>
        + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Func {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            arg_len: self.arg_len,
            runner: self.runner.clone_box(),
        }
    }
}

#[derive(Clone)]
pub struct NASLFunctions {
    functions: Vec<Func>,
}

impl NASLFunctions {
    pub fn new() -> Self {
        NASLFunctions { functions: vec![] }
    }
    pub fn register(&mut self, fun: Func) {
        let mut nf = self.functions.clone();
        nf.push(fun);
        self.functions = nf;
    }

    pub fn run(
        self,
        name: &str,
        args: Vec<FunctionArgument>,
        parameter: HashMap<String, Vec<char>>,
    ) -> Result<ResultType, Box<dyn Error>> {
        let not_found = &Func {
            name: name.to_string(),
            arg_len: args.len(),
            runner: Box::new(|s, _a, _p| Err(Box::new(UnexpectedError { description: s }))),
        };
        // lookup parameter
        let ra: Vec<FunctionArgument> = args
            .iter()
            .map(|x| {
                let input: String = x.value.iter().collect();
                if let Some(nv) = parameter.get(&input) {
                    return FunctionArgument {
                        name: x.name.clone(),
                        value: nv.clone(),
                    };
                }
                return x.clone();
            })
            .collect();
        let func = self
            .functions
            .iter()
            .find(|x| x.name == name.to_string() && x.arg_len == args.len())
            .unwrap_or(not_found);
        return (func.runner)(name.to_string(), ra, parameter);
    }
}

#[derive(Debug, Clone)]
struct UnexpectedError {
    description: String,
}

impl fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.description);
    }
}

impl Error for UnexpectedError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl UnexpectedError {
    pub fn args_len(name: &str, args_len: usize, expected: usize) -> Self {
        UnexpectedError {
            description: format!(
                "{} expects {} arguments but got {}",
                name, args_len, expected
            ),
        }
    }
}

impl Default for NASLFunctions {
    fn default() -> Self {
        let mut result = NASLFunctions::new();
        result.register(Func {
            runner: Box::new(|name, args, _params| match args.len() {
                1 => {
                    let input: String = args[0].value.iter().collect();
                    match input.parse::<i32>() {
                        Ok(i) => return Ok(ResultType::Exit(i)),
                        Err(err) => {
                            return Err(Box::new(err));
                        }
                    }
                }
                n => Err(Box::new(UnexpectedError::args_len(&name, n, 1))),
            }),
            arg_len: 1,
            name: "exit".to_string(),
        });
        result.register(Func {
            runner: Box::new(|name, args, _params| match args.len() {
                1 => {
                    let input: String = args[0].value.iter().collect();
                    match input.parse::<i32>() {
                        Ok(i) => return Ok(ResultType::IfResult(i != 0)),
                        Err(err) => {
                            return Err(Box::new(err));
                        }
                    }
                }
                n => Err(Box::new(UnexpectedError::args_len(&name, n, 1))),
            }),
            arg_len: 1,
            name: "if".to_string(),
        });
        result
    }
}

#[derive(Debug)]
enum State {
    Init,
    InWord(Vec<char>),
    InFunction(String, Vec<FunctionArgument>),
    ResultFunction(ResultType),
    Failure(Vec<char>),
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Exit(i32),
    EOF,
    Invalid(Vec<char>),
    NotImplemented,
}

pub fn interpret(
    lexer: Lexer,
    parameter: HashMap<String, Vec<char>>,
    known_functions: Option<NASLFunctions>,
) -> InterpretResult {
    let scoped_functions = known_functions.unwrap_or_default();
    let mut state = State::Init;

    for token in lexer {
        let combination = (state, token);
        println!("checking {:?}", combination);
        match combination {
            (State::Failure(x), _) => return InterpretResult::Invalid(x),
            (_, Token::Illegal(a)) => return InterpretResult::Invalid(vec![a]),
            (State::InFunction(f, ar), Token::Word(a)) => {
                let mut args = ar;
                args.push(FunctionArgument {
                    name: None,
                    value: a,
                });
                state = State::InFunction(f, args);
            }

            (State::InWord(f), Token::LParen) => {
                state = State::InFunction(f.iter().collect(), vec![])
            }
            (State::InFunction(f, a), Token::RParen) => {
                let functions = scoped_functions.clone();
                match functions.run(f.as_str(), a, parameter.clone()) {
                    Ok(rt) => match rt {
                        ResultType::Exit(rc) => return InterpretResult::Exit(rc),
                        _ => state = State::ResultFunction(rt),
                    },
                    Err(_e) => state = State::Failure(vec![]),
                }
            }
            (State::ResultFunction(ResultType::IfResult(false)), Token::Semicolon) => {
                state = State::Init;
            }
            (State::ResultFunction(ResultType::IfResult(false)), _) => {
                state = State::ResultFunction(ResultType::IfResult(false));
            }
            (_, Token::Semicolon) => return InterpretResult::NotImplemented,
            (_, Token::Word(a)) => state = State::InWord(a),
            (_, Token::LParen) => return InterpretResult::NotImplemented, //TODO adjust to execute
            (_, Token::RParen) => return InterpretResult::NotImplemented, //TODO adjust to execute
        }
    }
    return InterpretResult::EOF;
}
