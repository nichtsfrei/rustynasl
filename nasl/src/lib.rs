pub mod lexer;
pub mod interpreter;

pub use lexer::Lexer;
#[cfg(test)]
mod tests {
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
        for t in Lexer::new("  \r\nexit(42);".chars().collect()) {
            actual.push(t);
        }

        assert_eq!(expected, actual);
    }
}
