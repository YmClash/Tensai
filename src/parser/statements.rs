


//  ici on vas parser  les statement

use crate::parser::ast::{DataType, Statement, VariableDeclaration};
use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::parser::ast::{ASTNode, Mutability, Visibility};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;

impl Parser{

    pub fn parse_statement(&mut self) -> Result<ASTNode, ParserError> {
        let visibility = self.parse_visibility();
        if self.check(&[TokenType::KEYWORD(Keywords::LET)]) {
            self.parse_variable_declaration()
        }else if self.check(&[TokenType::KEYWORD(Keywords::TENSOR)]) {
            let visibility = visibility.unwrap_or(Visibility::Private);
            self.tensor_declaration(visibility)
        }
        else {
            self.parse_expression_statement()
        }

    }

    fn parse_expression_statement(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de l'expression statement");
        let expr = self.parse_expression(0);
        println!("Expression parsée : {:?}", expr);
        // //self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON))?;
        // self.consume_seperator();
        // println!("Separateur consommé");
        Ok(ASTNode::Expression(expr?))

    }





    // fn tensor_declaration(&mut self) -> Result<ASTNode, ParserError> {
    //     let name = self.consume_identifier("Expected tensor name")?;
    //
    //     // Parse le type du tenseur: <f32>, <f64>, etc.
    //     self.consume(&TokenType::DELIMITER(Delimiters::LESS), "Expected '<' after tensor name")?;
    //     let data_type = self.parse_tensor_type()?;
    //     self.consume(&TokenType::DELIMITER(Delimiters::GREATER), "Expected '>' after tensor type")?;
    //
    //     // Parse les dimensions: [2,3,4]
    //     self.consume(&TokenType::DELIMITER(Delimiters::LSBRACKET), "Expected '[' for tensor dimensions")?;
    //     let dimensions = self.parse_dimensions()?;
    //     self.consume(&TokenType::DELIMITER(Delimiters::RSBRACKET), "Expected ']' after dimensions")?;
    //
    //     // Gestion optionnelle de l'initialisation
    //     let initial_value = if self.match_token(&[TokenType::OPERATOR(Operators::EQUAL)]) {
    //         Some(self.expression()?)
    //     } else {
    //         None
    //     };
    //
    //     self.consume(&TokenType::DELIMITER(Delimiters::SEMICOLON), "Expected ';' after tensor declaration")?;
    //
    //     Ok(ASTNode::Declaration(Declaration::TensorDeclaration(TensorDeclaration {
    //         name,
    //         shape: TensorDimension::Fixed(dimensions),
    //         data_type,
    //         layout: TensorLayout::Contiguous,
    //         visibility: Visibility::Private,
    //         mutability: Mutability::Immutable,
    //         device: Device::CPU,
    //     })))
    // }










    pub fn parse_mutability(&mut self) -> Result<Mutability, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::MUT)]){
            Ok(Mutability::Mutable)
        } else {
            Ok(Mutability::Immutable)
        }
    }
    fn parse_visibility(&mut self) -> Result<Visibility, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]){
            Ok(Visibility::Public)
        } else {
            Ok(Visibility::Private)
        }
    }

}