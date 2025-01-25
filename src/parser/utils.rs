//  ici on a sles fonction utilitaire pour parseur

use crate::lexer::lex::Token;
use crate::lexer::tok::{Delimiters, Keywords, TokenType};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_error::ParserErrorType::{ExpectIdentifier, UnexpectedEndOfInput, UnexpectedEOF, UnexpectedToken};

impl Parser{


    pub fn consume_identifier(&mut self) -> Result<String, ParserError> {
        let current_token = self.current_token().ok_or_else(|| ParserError::new(UnexpectedEOF,self.current_position()))?;
        if let TokenType::IDENTIFIER {name:_} = &current_token.token_type{
            let name = current_token.text.clone();
            self.advance();
            Ok(name)
        } else { Err(ParserError::new(ExpectIdentifier,self.current_position())) }

    }

    pub fn current_token(&self) -> Option<&Token>{
        self.tokens.get(self.current)
    }
    pub fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    pub fn peek_token(&self) -> Option<&Token>{
        self.tokens.get(self.current + 1)
    }

    pub fn peek_next_token(&self) -> Option<&Token>{
        self.tokens.get(self.current + 1)

    }


    pub fn previous_token(&self) -> Option<&Token> {
        if self.current > 0 {
            // &self.tokens(self.current - 1)
            Some(&self.tokens[self.current - 1])
        } else { None }
    }

    pub fn is_at_end(&self) -> bool{
        self.current >= self.tokens.len() || self.current_token().map_or(true, |t| t.token_type == TokenType::EOF)

    }

    pub fn check(&self, expected:&[TokenType]) -> bool {
        if let Some(token) = self.current_token(){
            expected.contains(&token.token_type)
        } else {
            false
        }
    }

    pub fn match_token(&mut self, expected:&[TokenType]) -> bool {
        if self.check(expected){
            self.advance();
            return true
        } else {
            false
        }
    }



    pub fn consume(&mut self, expected: TokenType) -> Result<(), ParserError> {
        if let Some(token) = self.current_token() {
            if token.token_type == expected {
                println!("Consommation du token {:?}", token);
                //self.print_surrounding_tokens();
                self.advance();
                Ok(())
            } else {
                println!("Tensaiii!!!!!!!!!!!!!!!!!!!! Erreur: token attendu {:?}, token actuel {:?}", expected, token);
                Err(ParserError::new(UnexpectedToken, self.current_position()))
            }
        } else {
            //self.print_surrounding_tokens();
            println!("Tensaii!!!!!!!!!!!!!!!!: Erreur: fin de l'entrée inattendue");
            Err(ParserError::new(UnexpectedEndOfInput, self.current_position()))
        }
    }

    pub fn consume_seperator(&mut self) {
        println!("Braces Mode");
        if self.check(&[TokenType::DELIMITER(Delimiters::SEMICOLON)]) || self.check(&[TokenType::EOF]) {
            let _ = self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON));
        }
    }

    // Gestion des erreurs et synchronisation
    // fn synchronize(&mut self) {
    //     self.advance();
    //
    //     while !self.is_at_end() {
    //         if self.previous().token_type == TokenType::DELIMITER(Delimiters::SEMICOLON) {
    //             return;
    //         }
    //
    //         match self.peek().token_type {
    //             TokenType::KEYWORD(Keywords::FN) |
    //             TokenType::KEYWORD(Keywords::LET) |
    //             TokenType::KEYWORD(Keywords::TENSOR) |
    //             TokenType::KEYWORD(Keywords::FOR) |
    //             TokenType::KEYWORD(Keywords::IF) |
    //             TokenType::KEYWORD(Keywords::WHILE) => return,
    //             _ => {}
    //         }
    //
    //         self.advance();
    //     }
    // }








}
