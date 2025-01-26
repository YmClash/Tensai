use std::fmt;
use std::fmt::{Display, Formatter};




#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct LexerError{
    pub error: LexerErrorType,
    pub message: String,
    pub position: Position,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorType{
    InvalidCharacter(char),
    InvalidToken(String),
    InvalidFloat(String),
    InvalidInteger(String),
    InvalidHexadecimal(String),
    UnterminatedString,
    UnterminatedComment,
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

    fn range(start:&Position,end:&Position) -> String{
        if start.line == end.line{
            format!("line {}, column {}-{}",start.line,start.column,end.column)
        }else {
            format!("line {}, column {}-line {}, column {}",start.line,start.column,end.line,end.column)
        }
    }

}

impl Display for Position{
    fn fmt(&self,f:&mut Formatter<'_>) -> fmt:: Result{
        write!(f,"line {}, column {}",self.line,self.column)
    }
}

#[allow(dead_code)]
impl Display for LexerErrorType{
    fn fmt(&self,f:&mut Formatter<'_>) -> fmt:: Result{
        match self{
            LexerErrorType::InvalidCharacter(ch) => write!(f,"Invalid character: '{}'",ch),
            LexerErrorType::InvalidToken(token) => write!(f,"Invalid token: '{}'",token),
            LexerErrorType::InvalidFloat(float) => write!(f,"Invalid float: '{}'",float),
            LexerErrorType::InvalidInteger(integer) => write!(f,"Invalid integer: '{}'",integer),
            LexerErrorType::InvalidHexadecimal(hex) => write!(f,"Invalid hexadecimal: '{}'",hex),
            LexerErrorType::UnterminatedString => write!(f,"Unterminated string"),
            LexerErrorType::UnterminatedComment => write!(f,"Unterminated comment"),
        }
    }
}



#[allow(dead_code)]
impl LexerError {
    pub fn new(error: LexerErrorType, message: String, position: Position) -> Self {
        LexerError {
            error,
            message,
            position,
        }
    }
    pub fn invalid_character(c: char, position: Position) -> Self {
        Self::new(
            LexerErrorType::InvalidCharacter(c),
            format!("Invalid character: {}", c),
            position,
        )
    }
    pub fn invalid_token(t: &str, position: Position) -> Self {
        Self::new(
            LexerErrorType::InvalidToken(t.to_string()),
            format!("Invalid token: {}", t),
            position,
        )
    }
    pub fn invalid_integer(i: &str, position: Position) -> Self {
        Self::new(
            LexerErrorType::InvalidInteger(i.to_string()),
            format!("Invalid integer: {}", i),
            position,
        )
    }
    pub fn invalid_float(f: &str, position: Position) -> Self {
        Self::new(
            LexerErrorType::InvalidFloat(f.to_string()),
            format!("Invalid float: {}", f),
            position,
        )
    }

    pub fn invalid_hexadecimal(h: &str, position: Position) -> Self {
        Self::new(
            LexerErrorType::InvalidHexadecimal(h.to_string()),
            format!("Invalid hexadecimal: {}", h),
            position,
        )
    }
    pub fn unterminated_string(position: Position) -> Self {
        Self::new(
            LexerErrorType::UnterminatedString,
            "Unterminated string".to_string(),
            position,
        )
    }
    pub fn unterminated_comment(position: Position) -> Self {
        Self::new(
            LexerErrorType::UnterminatedComment,
            "Unterminated comment".to_string(),
            position,
        )
    }

}



