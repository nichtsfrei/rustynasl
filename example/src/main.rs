use::lexer;

fn main() {
    let input: Vec<char> = String::from(" exit(a: 0); \0").chars().collect();
    let mut l = lexer::Lexer::new(input);
    loop {
        let token = l.next();
        if token == lexer::Token::EOF {
            break;
        }
        println!("Token {:?}", token);
    }
    println!("Bye");
}
