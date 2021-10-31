use crate::lexer::{Lexer, Token};
use std::error::Error;
use std::fmt;
pub struct RuntimeParameter {}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    name: Option<Vec<char>>,
    value: Vec<char>,
}

pub enum ResultType {
    Exit(i32),
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
        params: Option<RuntimeParameter>,
    ) -> Result<ResultType, Box<dyn Error>> {
        (self.runner)(self.name, args, params)
    }
}
// https://github.com/rust-lang/rust/issues/41517 waitiung for trait alias
//trait Runnable = Fn(Vec<FunctionArgument>, Option<RuntimeParameter>) -> Result<ResultType, Box<dyn Error>>;

trait CloneableFn:
    Fn(String, Vec<FunctionArgument>, Option<RuntimeParameter>) -> Result<ResultType, Box<dyn Error>>
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
            Option<RuntimeParameter>,
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
    ) -> Result<ResultType, Box<dyn Error>> {
        let not_found = &Func {
            name: name.to_string(),
            arg_len: args.len(),
            runner: Box::new(|s, _a, _p| Err(Box::new(UnexpectedError { description: s }))),
        };
        let func = self
            .functions
            .iter()
            .find(|x| x.name == name.to_string() && x.arg_len == args.len())
            .unwrap_or(not_found);
        return (func.runner)(name.to_string(), args, None);
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
        result
    }
}

enum State {
    Init,
    InWord(Vec<char>),
    InFunction(String, Vec<FunctionArgument>),
    Function(String, Vec<FunctionArgument>),
    ResultFunction(ResultType),
    Failure(Vec<char>),
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Exit(i32),
    EOF,
    Invalid(char),
    NotImplemented,
}

pub fn interpret(lexer: Lexer, known_functions: Option<NASLFunctions>) -> InterpretResult {
    let scoped_functions = known_functions.unwrap_or_default().clone();
    let mut state = State::Init;
    for token in lexer {
        match token {
            Token::Illegal(a) => return InterpretResult::Invalid(a),
            Token::Word(a) => match state {
                State::Init => state = State::InWord(a),
                State::InFunction(f, ar) => {
                    let mut args = ar;
                    args.push(FunctionArgument {
                        name: None,
                        value: a,
                    });
                    state = State::InFunction(f, args);
                }
                _ => return InterpretResult::NotImplemented,
            },
            Token::LParen => match state {
                State::InWord(f) => state = State::InFunction(f.iter().collect(), vec![]),
                _ => return InterpretResult::NotImplemented,
            },
            Token::RParen => match state {
                State::InFunction(f, a) => state = State::Function(f, a),
                _ => state = State::Failure(vec![]),
            },
            Token::Semicolon => match state {
                State::Function(f, a) => {
                    let functions = scoped_functions.clone();
                    match functions.run(f.as_str(), a) {
                        Ok(rt) => match rt {
                            ResultType::Exit(rc) => return InterpretResult::Exit(rc),
                            _ => state = State::ResultFunction(rt),
                        },
                        Err(_e) => state = State::Failure(vec![]),
                    }
                }
                _ => return InterpretResult::NotImplemented,
            },
        }
    }
    return InterpretResult::EOF;
}
