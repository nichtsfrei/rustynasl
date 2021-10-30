use std::char;

pub struct Lexer{
    pub input: Box<dyn Iterator<Item=char>>,
    ch: char,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal(char),
    LParen, //(
    RParen, //)
    Semicolon,
    //LBRACE, //{
    //RBRACE, //}
    //LBRACKET, //[
    //RBRACKET, //]
    //PLUS,
    //MINUS,
    Word(Vec<char>),
}

impl Lexer {
    pub fn new(input: Box<dyn Iterator<Item=char>>) -> Self {
        Self {
            input,
            ch: ' ',
        }
    }

    fn read(&mut self) {
        match self.input.next() {
            Some(c) => self.ch = c,
            None => self.ch = '\0',
        }
    }

    fn read_word(&mut self) -> Vec<char> {
        let mut result = vec![ ];
        while self.ch.is_alphanumeric() {
            result.push(self.ch);
            self.read();
        }
        return result;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read();
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let result: Token;
        self.skip_whitespace();
        match self.ch {
            '(' => result = Token::LParen,
            ')' => result = Token::RParen,
            ';' => result = Token::Semicolon,
            '\0' => return None,
            _ => {
                // we need to skip the self.read on alphabetic or word
                if self.ch.is_alphanumeric() {
                    return Some(Token::Word(self.read_word()));
                } else {
                    result = Token::Illegal(self.ch);
                }
            }
        }
        self.read();
        return Some(result);
    }
}
