use::nasl;

fn main() {
    let input: Vec<char> = String::from(" exit(0); \0").chars().collect();
    let l = nasl::Lexer::new(input);
    let rc = nasl::interpreter::interpret(l, None);
    
    println!("result of interpreter = {:?}", rc);
}
