use std::char;

pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
    read_pos: usize,
    ch: char
}

#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL(Vec<char>),
    EOF,
    LPAREN, //(
    RPAREN, //)
    SEMICOLON,
    //LBRACE, //{
    //RBRACE, //}
    //LBRACKET, //[
    //RBRACKET, //]
    //PLUS,
    //MINUS,
    NUMERIC(Vec<char>), // we keep it as vec char because we may need the sign for interpration
    WORD(Vec<char>),
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            pos: 0,
            read_pos: 0,
            ch: ' ',
        }
    }

    fn read(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos = self.read_pos + 1;
    }

    fn read_word(&mut self) -> Vec<char> {
        let pos = self.pos;
        while self.ch.is_alphabetic() {
            self.read()
        }
        return self.input[pos..self.pos].to_vec()
    }

    fn read_numeric(&mut self) -> Vec<char> {
        let pos = self.pos;
        while self.ch.is_numeric() {
            self.read()
        }
        return self.input[pos..self.pos].to_vec()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read();
        }
    }

    pub fn next(&mut self) -> Token {
        let result: Token;
        self.skip_whitespace();
        match self.ch {
            '(' => { result = Token::LPAREN; }
            ')' => { result = Token::RPAREN; }
            ';' => { result = Token::SEMICOLON; }
            '\0' => { result = Token::EOF; } 
            _ => {
                // we need to skip the self.read on alphabetic or word
                if self.ch.is_alphabetic() {
                    return Token::WORD(self.read_word())
                } else if self.ch.is_numeric() {
                    return Token::NUMERIC(self.read_numeric())
                } else {
                    result = Token::ILLEGAL(self.input[self.pos..].to_vec());
                }

            }
        }
        self.read();
        return result;
     }
}
