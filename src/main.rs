
use tensai::lexer::lex::{Lexer};

fn main() {
    println!("Hello, Tensaï: A Genius Tensor Language!!!!!!");
    println!("\n");

    let code_sourde = r#"A #[1,2]> B"#;
    let mut lexer = Lexer::new(code_sourde);
    let tokens = lexer.tokenize();

    for (i,tok) in tokens.iter().enumerate(){
        println!("Token {}: {:?}",i,tok);
    }

    println!("\n");




    println!("///////////////////////by YmC////////////////////////////////");
}
