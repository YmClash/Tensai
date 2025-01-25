use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::parser::ast::{DataType, GenericType};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_error::ParserErrorType::{ExpectedTypeAnnotation, InvalidTypeAnnotation};

impl Parser{

    pub fn parse_type(&mut self) -> Result<DataType, ParserError> {
        let token = self
            .current_token()
            .ok_or_else(|| ParserError::new(ExpectedTypeAnnotation, self.current_position()))?;

        println!("Parsing type: {:?}", token);

        match &token.token_type {
            TokenType::KEYWORD(Keywords::INT) => {
                self.advance(); // Consomme le token `int`
                Ok(DataType::Int)
            }
            TokenType::KEYWORD(Keywords::FLOAT) => {
                self.advance(); // Consomme le token `float`
                Ok(DataType::Float)
            }
            TokenType::KEYWORD(Keywords::BOOL) => {
                self.advance(); // Consomme le token `bool`
                Ok(DataType::Bool)
            }
            TokenType::KEYWORD(Keywords::STR) => {
                self.advance(); // Consomme le token `string`
                Ok(DataType::String)
            }
            TokenType::KEYWORD(Keywords::CHAR) => {
                self.advance(); // Consomme le token `char`
                Ok(DataType::Char)
            }
            TokenType::IDENTIFIER { name } => {
                let base_name = name.clone();
                self.advance();
                // let base_name = name.clone();

                // Vérifie s'il y a des paramètres génériques
                if self.check(&[TokenType::OPERATOR(Operators::LESS)]) {
                    // Parse les paramètres génériques
                    let mut type_params = Vec::new();
                    self.consume(TokenType::OPERATOR(Operators::LESS))?;

                    loop {
                        type_params.push(self.parse_type()?);

                        if self.check(&[TokenType::OPERATOR(Operators::GREATER)]) {
                            self.advance();
                            break;
                        } else if self.check(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                            self.advance();
                        } else {
                            return Err(ParserError::new(InvalidTypeAnnotation, self.current_position()));
                        }
                    }

                    Ok(DataType::Generic(GenericType {
                        base: base_name,
                        type_parameters: type_params,
                    }))
                } else {
                    Ok(DataType::Named(base_name))
                }
            }
            _ => {
                println!("Unexpected token: {:?}", token);
                // Si le token actuel n'est pas un type valide, renvoyer une erreur
                Err(ParserError::new(
                    InvalidTypeAnnotation,
                    self.current_position(),
                ))
            }
        }
    }
}