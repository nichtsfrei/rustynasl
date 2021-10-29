mod lexer;
pub use lexer::Lexer;
pub use lexer::Token;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
