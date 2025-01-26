
use tensai::lexer::lex::{Lexer};
use tensai::parser::parser::{Parser};

fn main() {
    println!("Hello, Tensaï: A Genius Tensor Language!!!!!!");
    println!("\n");

    // let code_sourde = r#"obj.data[start+ offset].proccess(x,y).data[index]"#;
    let code_sourde = r#"array[1..10:2]"#;

    let mut lexer = Lexer::new(code_sourde);
    let tokens = lexer.tokenize();

    for (i,tok) in tokens.iter().enumerate(){
        println!("Token {}: {:?}",i+1,tok);
    }
    let mut parser = Parser::new(tokens);
    // let ast = parser.parse_program();

    while !parser.is_at_end(){
        match parser.parse_program() {
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
