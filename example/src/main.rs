use nasl;

use std::fs::File;
use std::io::Read;
use std::str::from_utf8;
use std::collections::HashMap;

struct BytesToChar {
    bytes: Box<dyn Iterator<Item = u8>>,
}

impl BytesToChar {
    pub fn from_file(f: File) -> Self {
        return BytesToChar {
            bytes: Box::new(f.bytes().filter_map(|b| b.ok())),
        };
    }
}

impl Iterator for BytesToChar {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let mut pc = self.bytes.next()?;
        if pc.is_ascii() {
            return Some(pc as char);
        }
        let mut buffer: Vec<u8> = vec![];
        while !pc.is_ascii() {
            buffer.push(pc);
            // we need to check eagerly to prevent skipping chars between two non ascii: 䉂?⌣
            // 3 bytes are minimum for utf8; se we just need to verify when it is met.
            if buffer.len() >= 3 {
                match from_utf8(&buffer) {
                    Ok(s) => return s.chars().next(),
                    Err(_e) => {}
                }
            }
            pc = self.bytes.next()?;
        }
        match from_utf8(&buffer) {
            Ok(s) => s.chars().next(),
            Err(_e) => None,
        }
    }
}

fn main() {
    let mut rp: HashMap<String, Vec<char>> = HashMap::new();
    rp.insert("description".to_string(), vec!('1'));
    match File::open("plugins/conditional-exit.nasl") {
        Ok(file) => {
            let l = nasl::Lexer::new(Box::new(BytesToChar::from_file(file)));
            match nasl::interpreter::interpret(l, rp, None) {
                nasl::InterpretResult::Exit(rc) => std::process::exit(rc),
                r => panic!("unexpected result of interpreter = {:?}", r),
            }
        }
        Err(e) => panic!("cannot open test.nasl: {:?}", e),
    }
}
