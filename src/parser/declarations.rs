use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::parser::ast::{ArrayExpression, ASTNode, DataType, Declaration, Decorator, Device, Expression, Literal, Mutability, Shape, TensorDeclaration, TensorDimension, TensorLayout, VariableDeclaration, Visibility};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_error::ParserErrorType::{InvalidShapeValue, UnexpectedEndOfInput, UnexpectedToken};

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

        let mutability = self.parse_mutability()?;

        let name = self.consume_identifier()?;

        let dtype = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            self.parse_type()?
        } else {
            DataType::Infer
        };

        let shape = if self.check(&[TokenType::OPERATOR(Operators::AT)]) {
            Some(self.parse_shape()?)  // renvoie un Vec<usize> ou autre
        } else {
            None
        };

        self.consume(TokenType::OPERATOR(Operators::EQUAL))?;
        let value = self.parse_expression(0)?;


        let device = if self.match_token(&[TokenType::OPERATOR(Operators::AT)]){
            Some(self.parse_device()?)
        }else { None };


        Ok(ASTNode::Declaration(Declaration::TensorDeclaration(TensorDeclaration{
            name,
            // decorators,
            shape,
            dtype,
            value,
            mutability,
            visibility
        })))

    }


    pub fn parse_array_expression(&mut self) -> Result<Expression, ParserError> {

        self.consume(TokenType::DELIMITER(Delimiters::LSBRACKET))?;
        let mut elements = Vec::new();
        loop {
            if !self.check(&[TokenType::DELIMITER(Delimiters::RSBRACKET)]) {
                elements.push(self.parse_expression(0)?);

                if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                    continue;
                }
            }

        }

    }

    // fn parse_decorators(&mut self) -> Result<Vec<Decorator>, ParserError> {
    //     let mut decorators = Vec::new();
    //     while self.check_decorator() {  // check si token courant est TokenType::OPERATOR(Operators::AROBAS) ou similaire
    //         let decorator = self.parse_decorator()?;
    //         decorators.push(decorator);
    //     }
    //     Ok(decorators)
    // }
    // // parse_decorator() -> ex. @shape(2,2) ou @gpu etc.
    // pub fn  parse_decorator(&mut self) -> Result<Decorator,ParserError> {
    //     println!("Début de la décoration");
    //     self.consume(TokenType::OPERATOR(Operators::AT))?;
    //     let name = self.consume_identifier()?;
    //     let arguments = if self.match_token(&[TokenType::DELIMITER(Delimiters::LPAR)]) {
    //         // self.parse_decorator_arguments()?
    //         self.parse_arguments_list()?
    //     } else {
    //         Vec::new()
    //         // None
    //     };
    //     Ok(Decorator{
    //         name,
    //         arguments
    //     })
    //
    // }
    //
    //

    pub fn parse_array_or_matrix_literal(&mut self) -> Result<Expression, ParserError> {
        // Consommer '['
        self.consume(TokenType::DELIMITER(Delimiters::LSBRACKET))?;

        let mut all_elements = Vec::new();

        loop {
            // Parse un élément (entier ou flottant)
            match &self.current_token().unwrap().token_type {
                TokenType::FLOAT { value } => {
                    all_elements.push(Expression::Literal(Literal::Float { value: *value }));
                    self.advance();
                },
                TokenType::INTEGER { value } => {
                    all_elements.push(Expression::Literal(Literal::Integer {
                        value: value.clone()
                    }));
                    self.advance();
                },
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            }

            // Vérifie le token suivant
            match &self.current_token().unwrap().token_type {
                TokenType::DELIMITER(Delimiters::COMMA) => {
                    self.advance();
                    continue;
                },
                TokenType::DELIMITER(Delimiters::RSBRACKET) => {
                    self.advance();
                    break;
                },
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            }
        }

        Ok(Expression::Array(Box::new(ArrayExpression {
            elements: all_elements,
        })))
    }


    fn parse_device(&mut self) -> Result<Device, ParserError> {
        if let Some(token) = self.current_token() {
            match &token.token_type {
                TokenType::KEYWORD(Keywords::CPU) => {
                    self.advance();
                    Ok(Device::CPU)
                }
                TokenType::KEYWORD(Keywords::GPU) => {
                    self.advance();
                    // let index = if self.match_token(&[TokenType::DELIMITER(Delimiters::LPAR)]) {
                    //     let index = self.parse_expression(0)?;
                    //     self.consume(TokenType::DELIMITER(Delimiters::RPAR))?;
                    //     if let Expression::Literal(Literal::Integer { value }) = index {
                    //         value.to_usize().ok_or_else(|| {
                    //             ParserError::new(UnexpectedToken, self.current_position())
                    //         })?
                    //     } else {
                    //         return Err(ParserError::new(UnexpectedToken, self.current_position()));
                    //     }
                    // } else {
                    //     0  // GPU par défaut (index 0)
                    // };
                    // Ok(Device::GPU(index))
                    Ok(Device::GPU)
                }
                TokenType::KEYWORD(Keywords::TPU) => {
                    self.advance();
                    Ok(Device::TPU)
                }
                _ => Err(ParserError::new(UnexpectedToken, self.current_position())),
            }
        } else {
            Err(ParserError::new(UnexpectedEndOfInput, self.current_position()))
        }
    }

    pub fn parse_shape(&mut self) -> Result<Shape,ParserError>{
        println!("Debut du parsing des shape");

        self.consume(TokenType::OPERATOR(Operators::AT))?;
        self.consume(TokenType::KEYWORD(Keywords::SHAPE))?;
        self.consume(TokenType::DELIMITER(Delimiters::LPAR))?;
        let mut shape = Vec::new();

        //pour le cas ou on a un shape vide
        if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]){
            self.consume(TokenType::DELIMITER(Delimiters::RPAR))?;
            return Ok(Shape{
                shape:shape
            })
        }

        loop {
            // Parser une expression et la convertir en usize
            let expr = self.parse_expression(0)?;
            if let Expression::Literal(Literal::Integer { value }) = expr {
                shape.push(value.try_into().map_err(|_| ParserError::new(InvalidShapeValue,self.current_position()))?);
            } else {
                return Err(ParserError::new(InvalidShapeValue, self.current_position()));
            }

            // Vérifie si c'est une virgule ou une parenthèse fermante
            if self.check(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                self.consume(TokenType::DELIMITER(Delimiters::COMMA))?;
            } else if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
                self.consume(TokenType::DELIMITER(Delimiters::RPAR))?;
                break;
            } else {
                return Err(ParserError::new(UnexpectedToken, self.current_position()));
            }
        }

        Ok(Shape { shape })

    }






}