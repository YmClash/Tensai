use num_bigint::BigInt;
use crate::lexer::lex::Token;
use crate::parser::ast::ASTNode;
use crate::parser::parser_error::{ ParserError};
use crate::parser::parser_error::Position;

pub struct Parser{
    pub tokens:Vec<Token>,
    pub current: usize,
}

impl Parser{
    pub fn new(tokens:Vec<Token>) -> Self{
        Parser{
            tokens,
            current: 0,
        }
    }

    pub fn parse_program(&mut self) -> Result<ASTNode, ParserError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(ASTNode::Program(statements))
    }

    pub fn current_position(&self) -> Position{
        Position{
            index: self.current,
        }
    }



}