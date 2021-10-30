use::nasl;

fn main() {
    let data: Vec<char> = "  \r\nexit(42); \0".chars().collect();
    let l = nasl::Lexer::new(Box::new(data.into_iter()));
    let rc = nasl::interpreter::interpret(l, None);
    
    println!("result of interpreter = {:?}", rc);
}
