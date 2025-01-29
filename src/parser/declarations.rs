use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::parser::ast::{ASTNode, DataType, Declaration, Device, Mutability, TensorDeclaration, TensorDimension, TensorLayout, VariableDeclaration, Visibility};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;

impl Parser{


    pub fn parse_variable_declaration(&mut self) -> Result<ASTNode, ParserError>{
        println!("Début de la déclaration de variable");
        self.consume(TokenType::KEYWORD(Keywords::LET))?;

        let mutability = self.parse_mutability()?;
        let name = self.consume_identifier()?;
        println!("Nom de la variable {:?}",name);

        let variable_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.parse_type()?

        } else {
            DataType::Infer
        };

        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;

        let value = self.parse_expression(0)?;

        // self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON))?;
        self.consume_seperator();

        println!("Fin de la déclaration de variable");
        Ok(ASTNode::Declaration(Declaration::VariableDeclaration(VariableDeclaration{
            name,
            variable_type:Some(variable_type),
            mutability,
            value: Some(value),
        })))

    }

    pub fn tensor_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        println!("Début de la déclaration de tenseur");
        self.consume(TokenType::KEYWORD(Keywords::TENSOR))?;

        let name = self.consume_identifier()?;

        let data_type = if self.match_token(&[TokenType::OPERATOR(Operators::LESS)])||self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.parse_type()?
        } else {
            DataType::Infer
        };

        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;

        let value = self.parse_expression(0)?;




        todo!()

    //     let name = self.consume_identifier()?;
    //
    //     // Parse le type du tenseur: <f32>, <f64>, etc.
    //     self.expect(&TokenType::OPERATOR(Operators::LESS),)?;
    //     // let data_type = self.parse_tensor_type()?;
    //     let data_type = self.parse_type()?;
    //     self.expect(&TokenType::OPERATOR(Operators::GREATER), )?;
    //
    //
    //     // Parse les dimensions: [2,3,4]
    //     self.consume(TokenType::DELIMITER(Delimiters::LSBRACKET),)?;
    //     let dimensions = self.parse_dimensions()?;
    //     self.consume(TokenType::DELIMITER(Delimiters::RSBRACKET),)?;
    //
    //     // Gestion optionnelle de l'initialisation
    //     let initial_value = if self.match_token(&[TokenType::OPERATOR(Operators::EQUAL)]) {
    //         Some(self.expression()?)
    //     } else {
    //         None
    //     };
    //
    //     self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON))?;
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
    }
}