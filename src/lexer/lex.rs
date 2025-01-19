use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::tok::TokenType;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

#[allow(dead_code)]
impl Token{
    pub fn new(text:String,token_type: TokenType,line:usize,column:usize) -> Self{
        Token{
            text,
            token_type,
            line,
            column,
        }
    }
}

#[allow(dead_code)]
pub struct Lexer<'a>{
    source: Peekable<Chars<'a>>,
    current_char: char,
    keywords: HashMap<String,TokenType>,
    operators: HashMap<String,TokenType>,
    delimiters: HashMap<String,TokenType>,
    current_line: usize,
    current_column: usize,
    current_token_text: String,
    at_line_start: bool,
}

impl <'a> Lexer<'a> {
    pub fn new(code_source: &'a str) -> Self{
        let lexer = Lexer{
            source:code_source.chars().peekable(),
            current_char: '\0',
            keywords: Self::Keywords(),
            operators: Self::Operators(),
            delimiters: Self::Delimiters(),
            current_line: 1,
            current_column: 1,
            current_token_text: String::new(),
            at_line_start: true,
        };
        lexer
    }



}