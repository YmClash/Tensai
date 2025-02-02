use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::parser::ast::{ArrayExpression, ASTNode, DataType, Declaration, Decorator, Device, Expression, FunctionDeclaration, Literal, Mutability, Parameter, ReturnStatement, Shape, Statement, TensorDeclaration, TensorDimension, TensorLayout, VariableDeclaration, Visibility};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_error::ParserErrorType::{ExpectedParameterName, InvalidShapeValue, UnexpectedEndOfInput, UnexpectedToken, ValidationError};

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

    pub fn parse_tensor_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
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

    pub fn parse_function_declaration(&mut self, visibility: Visibility) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de fonction");
        self.consume(TokenType::KEYWORD(Keywords::FN))?;
        let name = self.consume_identifier()?;
        println!("Nom de la fonction parsé : {}", name);

        self.consume(TokenType::DELIMITER(Delimiters::LPAR))?;

        let parameters = self.parse_function_parameters()?;
        // let parameters = self.parse_arguments_list()?;

        self.consume(TokenType::DELIMITER(Delimiters::RPAR))?;

        let return_type = if self.match_token(&[TokenType::OPERATOR(Operators::RARROW)]) {
            self.parse_type()?
        } else {
            DataType::Infer // Ou un type par défaut
        };

        let body = self.parse_block()?;

        // let return_type = self.parse_return_type(return_type, &body)?;

        // self.consume_seperator();  plus de ; apres une fonction

        Ok(ASTNode::Declaration(Declaration::FunctionDeclaration(FunctionDeclaration {
            name,
            parameters,
            return_type: Some(return_type),
            body,
            visibility,
        })))
    }

    pub fn parse_return_statement(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de l'instruction de retour");
        self.consume(TokenType::KEYWORD(Keywords::RETURN))?;
        let value = if !self.match_token(&[TokenType::NEWLINE, TokenType::EOF]) {
            Some(self.parse_expression(0)?)
        } else {
            None
        };
        println!("Valeur de retour parsée : {:?}", value);
        println!("Fin du parsing de l'instruction de retour OK!!!!!!!!!!!!!!");
        Ok(ASTNode::Statement(Statement::ReturnStatement(ReturnStatement{
            value,
        })))


    }

    pub fn parse_function_parameters(&mut self) -> Result<Vec<Parameter>, ParserError> {
        println!("Début du parsing des paramètres de fonction");
        let mut parameters = Vec::new();

        if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]){
            // pas de paramètres
            return Ok(parameters);
        }

        if !self.match_token(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
            loop {
                //let name = self.consume_parameter_name()?;
                let name = self.consume_identifier()?;
                println!("Nom du paramètre parsé : {}", name);
                self.consume(TokenType::DELIMITER(Delimiters::COLON))?;
                let param_type = self.parse_type()?;
                println!("Type du paramètre parsé : {:?}", param_type);

                parameters.push(Parameter { name, parameter_type: param_type });

                if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                    continue;
                } else if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
                    break;
                }else {
                    println!("Erreur lors du parsing des paramètres, token actuel : {:?}", self.current_token());
                    return Err(ParserError::new(ExpectedParameterName, self.current_position()));
                }
            }
        }
        println!("Paramètres parsés : {:?}", parameters);
        Ok(parameters)
    }









}