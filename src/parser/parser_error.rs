use std::fmt;
use std::fmt::{Display, Formatter};


#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub index: usize,
    // pub line: usize,  feature  for future  with span
    // pub column: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ParserError{
    pub error: ParserErrorType,
    pub message: String,
    pub position: Position,
    // pub span: Span,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorType{
    UnexpectedToken, //{ expected: TokenType, found: TokenType },
    UnexpectedEOF,
    InvalidAssignmentTarget,
    ExpectedExpression,
    InvalidVariableDeclaration,
    InvalidFunctionDeclaration,
    InvalidTypeAnnotation,
    ExpectVariableName,
    ExpectOperatorEqual,
    ExpectValue,
    ExpectColon,

    ExpectedTypeAnnotation,
    ExpectParameterName,
    ExpectFunctionName,
    ExpectIdentifier,
    ExpectedType,
    ExpectedDeclaration,

    MissingType,
    MissingParameter,

    TypeInferenceError,

    UnexpectedEndOfInput,

    ExpectedCloseParenthesis,
    ExpectedCommaOrCloseBrace,
    InvalidShapeValue,
    ExpectedParameterName,
    ValidationError,

}

#[allow(dead_code)]
impl Position {
    fn new() -> Self {
        Position { index: 0 }
    }
    fn advance(&mut self, ch: char) {
        self.index += ch.len_utf8();
    }
    fn move_left(&mut self) {
        self.index -= 1;
    }
}

//implement de l'affichage de la position

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Position index {}", self.index)
    }
}

//implement de l'affichage de l'erreur

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ParserError: {} at {}", self.message, self.position)
    }
}

impl Display for ParserErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorType::UnexpectedToken => write!(f, "Unexpected Token"),
            // ParserErrorType::UnexpectedToken{expected,found} => write!(f, "Expected Toke {:?}, but Found: {:?}", expected, found),
            ParserErrorType::UnexpectedEOF => write!(f, "Unexpected EOF"),
            ParserErrorType::InvalidAssignmentTarget => write!(f, "Invalid Assignment Target"),
            ParserErrorType::ExpectedExpression => write!(f, "Expected Expression"),
            ParserErrorType::InvalidVariableDeclaration => write!(f, "Invalid Variable Declaration"),
            ParserErrorType::InvalidFunctionDeclaration => write!(f, "Invalid Function Declaration"),
            ParserErrorType::InvalidTypeAnnotation => write!(f, "Invalid Type Annotation"),
            ParserErrorType::ExpectVariableName => write!(f, "Expect Variable Name"),
            ParserErrorType::ExpectOperatorEqual => write!(f, "Expect Operator Equal"),
            ParserErrorType::ExpectValue => write!(f, "Expect Value"),
            ParserErrorType::ExpectColon => write!(f, "Expect Colon"),
            ParserErrorType::ExpectedTypeAnnotation => write!(f, "Expected Type Annotation"),
            ParserErrorType::ExpectParameterName => write!(f, "Expect Parameter Name"),
            ParserErrorType::ExpectFunctionName => write!(f, "Expect Function Name"),
            ParserErrorType::ExpectIdentifier => write!(f, "Expect Identifier"),
            ParserErrorType::ExpectedType => write!(f, "Expected Type"),
            ParserErrorType::ExpectedDeclaration => write!(f, "Expected Declaration"),
            ParserErrorType::MissingType => write!(f, "Missing Type"),
            ParserErrorType::MissingParameter => write!(f, "Missing Parameter"),
            ParserErrorType::TypeInferenceError => write!(f, "Type Inference Error"),

            ParserErrorType::UnexpectedEndOfInput => write!(f, "Unexpected End Of Input"),
            ParserErrorType::ExpectedCloseParenthesis => write!(f, "Expected Close Parenthesis"),

            ParserErrorType::ExpectedCommaOrCloseBrace => write!(f, "Expected Comma Or Close Brace"),
            ParserErrorType::InvalidShapeValue => write!(f, "Invalid Shape Value"),
            ParserErrorType::ExpectedParameterName => write!(f, "Expected Parameter Name"),
            ParserErrorType::ValidationError => write!(f, "Validation Error"),



        }
    }

}

impl  ParserError{
    pub fn new(error: ParserErrorType, position: Position) -> Self {
        let message = match &error {
            ParserErrorType::UnexpectedToken => "Unexpected token".to_string(),
            // ParserErrorType::UnexpectedToken { expected, found } =>
            //     format!("Expected {:?}, but found {:?}", expected, found),
            ParserErrorType::UnexpectedEOF => "Unexpected end of file".to_string(),
            ParserErrorType::InvalidAssignmentTarget => "Invalid assignment target".to_string(),
            ParserErrorType::ExpectedExpression => "Expected expression".to_string(),
            ParserErrorType::InvalidVariableDeclaration => "Invalid variable declaration".to_string(),
            ParserErrorType::InvalidFunctionDeclaration => "Invalid function declaration".to_string(),
            ParserErrorType::InvalidTypeAnnotation => "Invalid type annotation".to_string(),
            ParserErrorType::ExpectVariableName => "Expect variable name".to_string(),
            ParserErrorType::ExpectOperatorEqual => "Expect operator equal".to_string(),
            ParserErrorType::ExpectValue => "Expect value".to_string(),
            ParserErrorType::ExpectColon => "Expect colon".to_string(),
            ParserErrorType::ExpectedTypeAnnotation => "Expected type annotation".to_string(),
            ParserErrorType::ExpectParameterName => "Expect parameter name".to_string(),
            ParserErrorType::ExpectFunctionName => "Expect function name".to_string(),
            ParserErrorType::ExpectIdentifier => "Expect identifier".to_string(),
            ParserErrorType::ExpectedType => "Expected type".to_string(),
            ParserErrorType::ExpectedDeclaration => "Expected declaration".to_string(),
            ParserErrorType::MissingType => "Missing type".to_string(),
            ParserErrorType::MissingParameter => "Missing parameter".to_string(),
            ParserErrorType::TypeInferenceError => "Type inference error".to_string(),

            ParserErrorType::UnexpectedEndOfInput => "Unexpected end of input".to_string(),

            ParserErrorType::ExpectedCloseParenthesis => "Expected close parenthesis".to_string(),

            ParserErrorType::ExpectedCommaOrCloseBrace => "Expected comma or close brace".to_string(),
            ParserErrorType::InvalidShapeValue => "Invalid shape value".to_string(),
            ParserErrorType::ExpectedParameterName => "Expected parameter name".to_string(),
            ParserErrorType::ValidationError => "Validation error".to_string(),


        };

        ParserError {
            error,
            message,
            position,
        }
    }
}

