use std::char;

pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
    read_pos: usize,
    ch: char,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal(Vec<char>),
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
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            pos: 0,
            read_pos: 0,
            ch: ' ',
        }
    }

    fn read(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos = self.read_pos + 1;
    }

    fn read_word(&mut self) -> Vec<char> {
        let pos = self.pos;
        while self.ch.is_alphanumeric() {
            self.read()
        }
        return self.input[pos..self.pos].to_vec();
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
                    result = Token::Illegal(self.input[self.pos..].to_vec());
                }
            }
        }
        self.read();
        return Some(result);
    }
}
