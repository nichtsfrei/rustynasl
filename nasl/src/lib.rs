pub mod lexer;
pub mod interpreter;

pub use lexer::Lexer;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
