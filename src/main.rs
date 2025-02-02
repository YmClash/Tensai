
use tensai::lexer::lex::{Lexer};
use tensai::parser::parser::{Parser};

fn main() {
    println!("Hello, Tensaï: A Genius Tensor Language!!!!!!");
    println!("\n");

    // let code_sourde = r#"obj.data[start+ offset].proccess(x,y).data[index]"#;
    let code_sourde1 = r#"array[1..10:2] array[:1d0]"#;
    let code_sourde2 = r#"array[:10]"#;
    let code_sourde3 = r#"array[1:]"#;
    let code_sourde4 = r#"array[1:10]"#;
    let code_sourde5 = r#"pub tensor A = 10 "#;
    let code_sourde6 = r#"@shape(2,2) tensor C = A @ B"#;

    let code_sourde7 = r#"pub tensor A = 10 for i in range(100) { if i > 0 {print(i)}else{print(0)}} while i < 100 {print(i)}"#;
    let code_sourde8 = r#"tensor A = [1.0, 2.0, 3.0]"#;

    let code_sourde9 = r#"tensor A:f32 @shape(2,2) = [1.0, 2.0, 3.0] "#;

    let mut lexer = Lexer::new(code_sourde9);
    let tokens = lexer.tokenize();

    for (i,tok) in tokens.iter().enumerate(){
        println!("Token {}: {:?}",i+1,tok);
    }
    let mut parser = Parser::new(tokens);
    // let ast = parser.parse_program();

    while !parser.is_at_end(){
        match parser.parse_program(){
            Ok(ast) =>{
                println!("AST OK!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("AST généré pour la déclaration,l'expression ou le statement  :");
                println!("{:#?}",ast);
            }
            Err(e) => {
                println!("Erreur de parsing : {:?}",e);
                break;
            }
        }
    }


    println!("\n");




    println!("////////////////TENSAI//LANG///////by YmC////////////////////////////////");
    println!("\n\n");
}
