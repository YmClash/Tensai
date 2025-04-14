

// #[allow(dead_code)]
// #[derive(Debug, Clone, PartialEq)]
// pub struct Span {
//     pub start: Position,
//     pub end: Position,
// }
//

use std::fmt;
use std::fmt::{Display, Formatter};
use crate::lexer::lexer_error::LexerErrorType;
use crate::parser::ast::DataType;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub error: SemanticErrorType,
    pub message: String,
    pub position: Position
}

#[allow(dead_code)]
#[derive(Debug,Clone,PartialEq)]
pub struct IncompatibleTypes{
    expected: DataType,
    got: DataType,
    context:String,
}



#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticErrorType{
    UndefinedVariable,
    UndefinedFunction,
    UndefinedTensor,
    UndefinedType,
    InvalidType,
    InvalidShape,
    // TypeError(TypeError),
    UndefinedSymbol(String),
    DuplicateSymbol(String),
    // IncompatibleTypes {
    //     expected: DataType,
    //     got: DataType,
    //     context: String,
    // },
    IncompatibleTypes(IncompatibleTypes),
    RootError

}


#[allow(dead_code)]
impl Position{
    fn new() -> Self{
        Position{
            line: 1,
            column: 1,
        }
    }
    fn advance(&mut self, ch:char){
        self.column += 1;
        if ch == '\n'{
            self.line += 1;
            self.column = 1;
        }
    }
    fn move_left(&mut self){
        self.column -=1;
    }
}

#[allow(dead_code)]
impl Display for Position{
    fn fmt(&self,f:&mut Formatter<'_>) -> fmt:: Result{
        write!(f,"line {}, column {}",self.line,self.column)
    }
}


impl Display for SemanticErrorType{
    fn fmt(&self,f:&mut Formatter<'_>) -> fmt:: Result{
        todo!()
        // match self{
        //     todo
        // }
    }
}


impl SemanticError {
    pub fn new(error: SemanticErrorType, message: String, position:Position) -> Self{
        SemanticError{
            error,
            message,
            position
        }
    }


    pub fn undefinedVariable(c: char, position: Position) -> Self {
        Self::new(
            SemanticErrorType::UndefinedVariable,
            // format!("Undefined Variable: {}", c),
            "UndefinedVariable".to_string(),
            position,
        )
    }




}



