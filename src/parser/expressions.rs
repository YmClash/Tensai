use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::parser::ast::{ArrayAccess, Assignment, ASTNode, BinaryOperation, CompoundAssignment, CompoundOperator, DataType, Expression, FunctionCall, Literal, MemberAccess, MethodCall, Operator, Parameter, RangeExpression, UnaryOperation, UnaryOperator};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_error::ParserErrorType::{ExpectedCloseParenthesis, UnexpectedEndOfInput, UnexpectedToken};

impl Parser{




    pub fn parse_expression(&mut self,precedence:u8) -> Result<Expression, ParserError>{
        println!("Début du parsing de l'expression");

        let mut left = self.parse_unary_expression()?;

        if let Some(token) = self.current_token(){
            match &token.token_type {
                TokenType::OPERATOR(Operators::EQUAL) =>{
                    self.advance();
                    let value = self.parse_expression(precedence)?;
                    return Ok(Expression::Assignment(Assignment{
                        target: Box::new(left),
                        value: Box::new(value)
                    }));
                }
                //on a les  operator composer
                TokenType::OPERATOR(op) => {
                    if let Some(compound_op) = self.get_compound_operator(op){
                        self.advance();
                        let value = self.parse_expression(precedence)?;
                        return Ok(Expression::CompoundAssignment(CompoundAssignment{
                            target: Box::new(left),
                            operator: compound_op,
                            value: Box::new(value)
                        }))
                    }
                }
                _ => {}
            }
        }

        while let Some(operator) = self.peek_operator(){
            let operator_precedence = self.get_operator_precedence(&operator);
            if operator_precedence < precedence{
                break;
            }
            self.advance();
            let right = self.parse_expression(operator_precedence + 1)?;

            if let Operator::Range|Operator::RangeInclusive = operator{
                left = Expression::RangeExpression(RangeExpression{
                    left:Some(Box::new(left)),
                    operator,
                    right:Some(Box::new(right))
                });
            }else {
                left = Expression::BinaryOperation(BinaryOperation{
                    left:Box::new(left),
                    operator,
                    right:Box::new(right)
                })
            }
        }

        println!("Fin du parsing de l'expression");
        Ok(left)

    }


    pub fn parse_postfix_expression(&mut self) -> Result<Expression, ParserError>{
        todo!()

    }

    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        println!("Debut du parsing de l'expression unaire");
        println!("Début du parsing de l'expression unaire, current_token = {:?}", self.current_token());

        if let Some(token) = self.current_token(){
            match &token.token_type{
                TokenType::OPERATOR(Operators::MINUS) => {
                    self.advance();
                    let right = self.parse_unary_expression()?;
                    return Ok(Expression::UnaryOperation(UnaryOperation{
                        operator: UnaryOperator::Negative,
                        operand: Box::new(right)
                    }))
                }
                TokenType::OPERATOR(Operators::EXCLAMATION) => {
                    self.advance();
                    let right = self.parse_unary_expression()?;
                    return Ok(Expression::UnaryOperation(UnaryOperation{
                        operator: UnaryOperator::Not,
                        operand: Box::new(right)
                    }))
                }
                TokenType::OPERATOR(Operators::AMPER) =>{
                    self.advance();
                    if self.check(&[TokenType::KEYWORD(Keywords::MUT)]){
                        self.advance();
                        let right = self.parse_unary_expression()?;
                        return Ok(Expression::UnaryOperation(UnaryOperation{
                            operator: UnaryOperator::ReferenceMutable,
                            operand: Box::new(right)
                        }))
                    }else {
                        let right = self.parse_unary_expression()?;
                        return Ok(Expression::UnaryOperation(UnaryOperation{
                            operator: UnaryOperator::Reference,
                            operand: Box::new(right)
                        }))
                    }

                }
                TokenType::OPERATOR(Operators::STAR) =>{
                    self.advance();
                    let right = self.parse_unary_expression()?;
                    return Ok(Expression::UnaryOperation(UnaryOperation{
                        operator: UnaryOperator::Dereference,
                        operand: Box::new(right)
                    }))
                }
                _ => self.parse_primary_expression()

            }

        }else { Err(ParserError::new(UnexpectedEndOfInput, self.current_position())) }


    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression primaire");
        if let Some(token) = self.current_token() {
            let expr = match &token.token_type{
                TokenType::IDENTIFIER { name } => {
                    let name = name.clone();
                    self.advance();
                    Expression::Identifier(name)
                }
                TokenType::INTEGER { value } => {
                    let value = value.clone();
                    println!("Valeur entière {:?}", value);
                    self.advance();
                    Expression::Literal(Literal::Integer { value })
                }
                TokenType::FLOAT { value } => {
                    let value = *value;
                    println!("Valeur flottante {:?}", value);
                    self.advance();
                    Expression::Literal(Literal::Float { value })
                }
                TokenType::CHAR { value } => {
                    let value = *value;
                    println!("Valeur char {:?}", value);
                    self.advance();
                    Expression::Literal(Literal::Char(value))
                }
                TokenType::STRING { value,.. } => {
                    let value = value.clone();
                    if value.len() == 1 && self.if_single_quote(&value) {
                        self.advance();
                        Expression::Literal(Literal::Char(value.chars().next().unwrap()))
                    }else {
                        self.advance();
                        Expression::Literal(Literal::String(value))
                    }
                }
                TokenType::KEYWORD(Keywords::TRUE) => {
                    self.advance();
                    Expression::Literal(Literal::Boolean(true))
                }
                TokenType::KEYWORD(Keywords::FALSE) => {
                    self.advance();
                    Expression::Literal(Literal::Boolean(false))
                }
                TokenType::KEYWORD(Keywords::SELF) => {
                    self.advance();
                    let name = "self".to_string();
                    Expression::Identifier(name)
                }
                TokenType::DELIMITER(Delimiters::LPAR) => {
                    self.advance();
                    let expr = self.parse_expression(0)?;
                    if let Some(token) = self.current_token() {
                        if matches!(token.token_type, TokenType::DELIMITER(Delimiters::RPAR)) {
                            self.advance();
                            expr
                        } else {
                            return Err(ParserError::new(
                                ExpectedCloseParenthesis,
                                self.current_position(),
                            ));
                        }
                    } else {
                        return Err(ParserError::new(
                            UnexpectedEndOfInput,
                            self.current_position(),
                        ));
                    }
                }
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            };
            Ok(expr)
        } else {
            Err(ParserError::new(
                UnexpectedEndOfInput,
                self.current_position(),
            ))
        }

    }






    fn get_operator_precedence(&self, operator: &Operator) -> u8 {
        match operator {
            Operator::Multiplication | Operator::Division | Operator::Modulo => 5,
            Operator::Addition | Operator::Substraction => 4,
            Operator::LessThan | Operator::GreaterThan | Operator::LesshanOrEqual | Operator::GreaterThanOrEqual => 3,
            Operator::Range | Operator::RangeInclusive => 3,
            Operator::Equal | Operator::NotEqual => 2,
            Operator::And => 1,
            //Operator::Or => 0,
            _ => 0,
        }
    }

    fn get_compound_operator(&self,op:&Operators) -> Option<CompoundOperator> {
        match op {
            Operators::PLUSEQUAL => Some(CompoundOperator::AddAssign),
            Operators::MINEQUAL => Some(CompoundOperator::SubAssign),
            Operators::STAREQUAL => Some(CompoundOperator::MulAssign),
            Operators::SLASHEQUAL => Some(CompoundOperator::DivAssign),
            Operators::PERCENTEQUAL => Some(CompoundOperator::ModAssign),
            _ => None,
        }
    }

    pub fn peek_operator(&self) -> Option<Operator> {
        let token = self.current_token()?;
        println!("Token: {:?}", token);
        match &token.token_type {
            TokenType::OPERATOR(op) => {
                match op {
                    Operators::PLUS => Some(Operator::Addition),
                    Operators::MINUS => Some(Operator::Substraction),
                    Operators::STAR => Some(Operator::Multiplication),
                    Operators::SLASH => Some(Operator::Division),
                    Operators::PERCENT => Some(Operator::Modulo),
                    Operators::LESS => Some(Operator::LessThan),
                    Operators::GREATER => Some(Operator::GreaterThan),
                    Operators::LESSEQUAL => Some(Operator::LesshanOrEqual),
                    Operators::GREATEREQUAL => Some(Operator::GreaterThanOrEqual),
                    Operators::EQEQUAL => Some(Operator::Equal),
                    Operators::NOTEQUAL => Some(Operator::NotEqual),
                    Operators::AND => Some(Operator::And),
                    Operators::OR => Some(Operator::Or),
                    Operators::DOTDOT => Some(Operator::Range),
                    Operators::DOTDOTEQUAL => Some(Operator::RangeInclusive),

                    Operators::AT => Some(Operator::TENSORPROD),
                    Operators::TRANSPOSE => Some(Operator::TRANSPOSE),
                    Operators::CONTRACT => Some(Operator::CONTRACT),
                    Operators::CONTRACTDIM => Some(Operator::CONTRACTDIM),


                    _ => None,
                }
            }
            _ => None,
        }

    }





    pub fn parse_arguments_list(&mut self) -> Result<Vec<Expression>, ParserError> {
        println!("Début du parsing de la liste d'arguments");
        let mut arguments = Vec::new();
        if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]){
            return Ok(arguments);
        }
        loop {
            let argument = self.parse_expression(0);
            arguments.push(argument?);

            if !self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                break;
            }
        }
        println!("Arguments liste parsés : {:?}", arguments);
        Ok(arguments)

    }

    pub fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, ParserError> {
        println!("Début du parsing de la liste des paramètres");
        let mut parameters = Vec::new();

        if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
            self.advance(); // Consomme ')'
            return Ok(parameters); // Pas de paramètres
        }

        loop {
            let param_name = self.consume_identifier()?;

            // Vérifier s'il y a un type spécifié
            let param_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
                Some(self.parse_type()?)
            } else {
                None
            };

            parameters.push(Parameter {
                name: param_name,
                parameter_type: param_type.unwrap_or(DataType::Infer),
            });

            // Si le prochain token est une virgule, continuer
            if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                continue;
            } else if self.check(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
                //self.advance(); // Consomme ')'
                break;
            } else {
                return Err(ParserError::new(ExpectedCloseParenthesis, self.current_position()));
            }
        }

        Ok(parameters)
    }



    pub fn if_single_quote(&self,s:&str) -> bool {
        if s.starts_with('\'') && s.ends_with('\'')  && s.len() == 3 {
            true
        } else {
            false
        }

        // let chars: Vec<char> = s.chars().collect();
        // if chars.len() != 3 {
        //     return false;
        // }
        //
        // return chars[0] == '\'' &&
        //     chars[2] == '\'' &&
        //     chars[1].is_ascii();

    }






}