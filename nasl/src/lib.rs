pub mod interpreter;
pub mod lexer;

pub use lexer::Lexer;
#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    use crate::interpreter::{interpret, InterpreteResult};
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
        let data: Vec<char> = "  \r\nexit(42);".chars().collect();
        for t in Lexer::new(Box::new(data.into_iter())) {
            actual.push(t);
        }

        assert_eq!(expected, actual);

        
    }
}
