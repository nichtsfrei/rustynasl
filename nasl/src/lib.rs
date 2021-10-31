pub mod interpreter;
mod lexer;

pub use interpreter::InterpretResult;
pub use lexer::Lexer;
#[cfg(test)]
mod tests {
    use crate::interpreter::{interpret, InterpretResult};
    use crate::lexer::{Lexer, Token};
    #[test]
    fn it_interprets_exit() {
        let expected: Vec<Token> = vec![
            Token::Word("exit".chars().collect()),
            Token::LParen,
            Token::Word(vec!['4', '2']),
            Token::RParen,
            Token::Semicolon,
        ];
        let mut actual: Vec<Token> = vec![];
        let data = "  \r\nexit(42);";
        for t in Lexer::new(Box::new(data.chars())) {
            actual.push(t);
        }
        assert_eq!(expected, actual);

        assert_eq!(
            InterpretResult::Exit(42),
            interpret(Lexer::new(Box::new(data.chars())), None)
        );
    }
}
