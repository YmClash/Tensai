


//  ici on vas parser  les statement

use crate::parser::ast::{Body, DataType, ForStatement, IfStatement, LoopStatement, Statement, VariableDeclaration, WhileStatement};
use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::parser::ast::{ASTNode, Mutability, Visibility};
// use crate::parser::ast::ASTNode::Body;
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_error::ParserErrorType::ExpectedCommaOrCloseBrace;

impl Parser{

    pub fn parse_statement(&mut self) -> Result<ASTNode, ParserError> {
        let visibility = self.parse_visibility();
        if self.check(&[TokenType::KEYWORD(Keywords::LET)]) {
            self.parse_variable_declaration()
        }else if self.check(&[TokenType::KEYWORD(Keywords::TENSOR)]) {
            let visibility = visibility.unwrap_or(Visibility::Private);
            self.tensor_declaration(visibility)
        }else if self.check(&[TokenType::KEYWORD(Keywords::IF)]) {
            self.parse_if_statement()
        }else if self.check(&[TokenType::KEYWORD(Keywords::WHILE)]) {
            self.parse_while_statement()
        }else if self.check(&[TokenType::KEYWORD(Keywords::FOR)]) {
            self.parse_for_statement()
        }else if self.check(&[TokenType::KEYWORD(Keywords::LOOP)]) {
            self.parse_loop_statement()
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
    pub fn parse_block(&mut self) -> Result<Vec<ASTNode>, ParserError> {
        println!("Parsing braced block");
        self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;
        let mut statements = Vec::new();

        while !self.check(&[TokenType::DELIMITER(Delimiters::RCURBRACE), TokenType::EOF]) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            // je vais ajoute un code qui  m'aiderai  a  parse le  body de parse_declaration_body
            if self.check(&[TokenType::DELIMITER(Delimiters::COMMA)]){
                self.consume(TokenType::DELIMITER(Delimiters::COMMA))?;
            }else if !self.check(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
                return Err(ParserError::new(ExpectedCommaOrCloseBrace, self.current_position()));
            }
        }
        self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;
        Ok(statements)
        // Ok(Body::Body(statements))

    }

    pub fn parse_if_statement(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de l'instruction if");

        self.consume(TokenType::KEYWORD(Keywords::IF))?;
        let condition = self.parse_expression(0)?;
        //let then_block = self.parse_body_block()?;; // block_expression
        let elif_block = self.parse_block()?;

        let else_block = if self.check(&[TokenType::KEYWORD(Keywords::ELIF)]) {
            // self.advance();
            self.consume(TokenType::KEYWORD(Keywords::ELIF))?;

            let elif_condition = self.parse_expression(0)?;
            let elif_then_block = self.parse_block()?;

            let elif_else_block = if self.check(&[TokenType::KEYWORD(Keywords::ELIF)]){
                Some(self.parse_block()?)
            }else { None };


            Some(vec![ASTNode::Statement(Statement::IfStatement(IfStatement{
                condition: elif_condition,
                elif_block: elif_then_block,
                else_block: elif_else_block,
            }))])

        }else if self.check(&[TokenType::KEYWORD(Keywords::ELSE)]){
            self.consume(TokenType::KEYWORD(Keywords::ELSE))?;
            //Some(self.parse_body_block()?)
            Some(self.parse_block()?)
        }else { None };

        println!("Fin du parsing de l'instruction if OKKKKKKKKKKKK");
        Ok(ASTNode::Statement(Statement::IfStatement(IfStatement{
            condition,
            elif_block,
            else_block,
        })))

    }

    pub fn  parse_while_statement(&mut self) ->Result<ASTNode,ParserError>{
        println!("Debut du parsing de l'instruction while");
        self.consume(TokenType::KEYWORD(Keywords::WHILE))?;
        let condition = self.parse_expression(0)?;
        let body = self.parse_block()?;
        println!("Fin du parsing de l'instruction while OKKKKKKKKKKKK");
        Ok(ASTNode::Statement(Statement::WhileStatement(WhileStatement{
            condition,
            body,
        })))
    }

    pub fn parse_for_statement(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de l'instruction for");

        self.consume(TokenType::KEYWORD(Keywords::FOR))?;

        let iterator = self.consume_identifier()?;
        self.consume(TokenType::KEYWORD(Keywords::IN))?;
        let iterable = self.parse_expression(0)?;
        let body = self.parse_block()?;
        println!("Fin du parsing de l'instruction for OK!!!!!!!!!!!!!!!");
        Ok(ASTNode::Statement(Statement::ForStatement(ForStatement{
            iterator,
            iterable,
            body
        })))

    }

    pub fn parse_loop_statement(&mut self) -> Result<ASTNode,ParserError> {
        println!("Debut du parsing de l'instruction loop");
        let label = self.check_for_label()?;

        self.consume(TokenType::KEYWORD(Keywords::LOOP))?;
        let body = self.parse_block()?;
        println!("Fin du parsing de l'instruction loop OKKKKKKKKKKKK");
        Ok(ASTNode::Statement(Statement::LoopStatement(LoopStatement{
            body,
        })))
    }

    pub fn check_for_label(&mut self) -> Result<Option<String>, ParserError> {
        // Vérifie si le token actuel est un identifiant
        if let Some(current) = self.peek_token() {
            if let Some(next) = self.peek_next_token() {
                // Vérifie si c'est un label (identifiant suivi de ':')
                match (&current.token_type, &next.token_type) {
                    (
                        TokenType::IDENTIFIER { name },
                        TokenType::DELIMITER(Delimiters::COLON)
                    ) => {
                        // Clone le nom avant d'avancer
                        let label_name = name.clone();

                        // Consomme l'identifiant et le ':'
                        self.advance(); // Consomme l'identifiant
                        self.advance(); // Consomme le ':'

                        return Ok(Some(label_name));
                    }
                    _ => return Ok(None)
                }
            }
        }
        Ok(None)
    }














}