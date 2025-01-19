use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::tok::{Delimiters, Keywords, Operators, TokenType};
use crate::lexer::lexer_error::{LexerError, LexerErrorType, Position};

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

// #[allow(dead_code)]
// pub enum LexerState{
//     NORMAL,
//     IN_STRING,
//     IN_COMMENT,
//     IN_TENSOR_DIM,
//     IN_ANNOTATION,clear

// }

#[allow(dead_code)]
pub struct Lexer<'a>{
    pub source: Peekable<Chars<'a>>,
    pub current_char: char,
    pub keywords: HashMap<String, Keywords>,
    pub operators: HashMap<String, Operators>,
    pub delimiters: HashMap<String, Delimiters>,
    pub current_line: usize,
    pub current_column: usize,
    pub current_token_text: String,
    pub at_line_start: bool,
}

impl<'a> Lexer<'a>  {
    pub fn new(code_source: &'a str) -> Self {
        let lexer = Lexer{
            source:code_source.chars().peekable(),
            current_char: '\0',
            keywords: Self::keywords(),
            operators: Self::operators(),
            delimiters: Self::delimiters(),
            current_line: 1,
            current_column: 1,
            current_token_text: String::new(),
            at_line_start: true,
        };
        lexer
    }

    #[allow(dead_code)]
    fn keywords() -> HashMap<String,Keywords>{
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(),Keywords::AND);
        keywords.insert("as".to_string(),Keywords::AS);
        keywords.insert("break".to_string(),Keywords::BREAK);
        keywords.insert("class".to_string(),Keywords::CLASS);
        keywords.insert("continue".to_string(),Keywords::CONTINUE);
        keywords.insert("del".to_string(),Keywords::DEL);
        keywords.insert("elif".to_string(),Keywords::ELIF);
        keywords.insert("else".to_string(),Keywords::ELSE);
        keywords.insert("enum".to_string(),Keywords::ENUM);
        keywords.insert("except".to_string(),Keywords::EXCEPT);
        keywords.insert("False".to_string(),Keywords::FALSE);
        keywords.insert("finally".to_string(),Keywords::FINALLY);
        keywords.insert("for".to_string(),Keywords::FOR);
        keywords.insert("gpu".to_string(),Keywords::GPU);
        keywords.insert("if".to_string(),Keywords::IF);
        keywords.insert("in".to_string(),Keywords::IN);
        keywords.insert("is".to_string(),Keywords::IS);
        keywords.insert("lambda".to_string(),Keywords::LAMBDA);
        keywords.insert("let".to_string(),Keywords::LET);
        keywords.insert("loop".to_string(),Keywords::LOOP);
        keywords.insert("match".to_string(),Keywords::MATCH);
        keywords.insert("mod".to_string(),Keywords::MOD);
        keywords.insert("None".to_string(),Keywords::NONE);

        keywords.insert("not".to_string(),Keywords::NOT);
        keywords.insert("on".to_string(),Keywords::ON);
        keywords.insert("or".to_string(),Keywords::OR);
        keywords.insert("parallel".to_string(),Keywords::PARALLEL);
        keywords.insert("pub".to_string(),Keywords::PUB);
        keywords.insert("pass".to_string(),Keywords::PASS);

        keywords.insert("return".to_string(),Keywords::RETURN);
        keywords.insert("self".to_string(),Keywords::SELF);
        keywords.insert("struct".to_string(),Keywords::STRUCT);
        keywords.insert("tensor".to_string(),Keywords::TENSOR);
        keywords.insert("True".to_string(),Keywords::TRUE);
        keywords.insert("type".to_string(),Keywords::TYPE);
        keywords.insert("typeof".to_string(),Keywords::TYPEOF);
        keywords.insert("use".to_string(),Keywords::USE);
        keywords.insert("where".to_string(),Keywords::WHERE);

        keywords.insert("while".to_string(),Keywords::WHILE);

        keywords.insert("yield".to_string(),Keywords::YIELD);

        return keywords;

    }

    #[allow(dead_code)]
    fn operators() -> HashMap<String, Operators> {
        let mut operators = HashMap::new();
        operators.insert("+".to_string(), Operators::PLUS);
        operators.insert("-".to_string(), Operators::MINUS);
        operators.insert("*".to_string(), Operators::STAR);
        operators.insert("/".to_string(), Operators::SLASH);
        operators.insert("%".to_string(), Operators::PERCENT);
        operators.insert("==".to_string(), Operators::EQEQUAL);
        operators.insert("!=".to_string(), Operators::NOTEQUAL);
        operators.insert("<".to_string(), Operators::LESS);
        operators.insert(">".to_string(), Operators::GREATER);
        operators.insert("<=".to_string(), Operators::LESSEQUAL);
        operators.insert(">=".to_string(), Operators::GREATEREQUAL);
        operators.insert("=".to_string(), Operators::EQUAL);
        operators.insert("++".to_string(), Operators::PLUSEQUAL);
        operators.insert("--".to_string(), Operators::MINEQUAL);
        operators.insert("**".to_string(), Operators::DOUBLESTAR);
        //operators.insert("//".to_string(), Operators::DOUBLESLASH);
        operators.insert("&&".to_string(), Operators::AND);
        operators.insert("||".to_string(), Operators::OR);
        operators.insert("!".to_string(), Operators::EXCLAMATION);
        operators.insert("&".to_string(), Operators::AMPER);
        operators.insert("|".to_string(), Operators::VBAR);
        operators.insert("^".to_string(), Operators::CIRCUMFLEX);
        operators.insert("<<".to_string(), Operators::LEFTSHIFT);
        operators.insert(">>".to_string(), Operators::RIGHTSHIFT);
        operators.insert("~".to_string(), Operators::TILDE);
        operators.insert("+=".to_string(), Operators::PLUSEQUAL);
        operators.insert("-=".to_string(), Operators::MINEQUAL);
        operators.insert("*=".to_string(), Operators::STAREQUAL);
        operators.insert("/=".to_string(), Operators::SLASHEQUAL);
        operators.insert("%=".to_string(), Operators::PERCENTEQUAL);
        operators.insert("&=".to_string(), Operators::AMPEREQUAL);
        operators.insert("|=".to_string(), Operators::VBAREQUAL);
        operators.insert("^=".to_string(), Operators::CIRCUMFLEXEQUAL);
        operators.insert("<<=".to_string(), Operators::LEFTSHIFTEQUAL);
        operators.insert(">>=".to_string(), Operators::RIGHTSHIFTEQUAL);
        operators.insert("**=".to_string(), Operators::DOUBLESTAREQUAL);
        operators.insert("//=".to_string(), Operators::DOUBLESLASHEQUAL);
        operators.insert("@".to_string(), Operators::AT);
        operators.insert("@=".to_string(), Operators::ATEQUAL);
        operators.insert("->".to_string(), Operators::RARROW);
        operators.insert(":=".to_string(), Operators::COLONEQUAL);
        // operators.insert("*/".to_string(), Operators::STARSLASH);
        // operators.insert("/*".to_string(), Operators::SLASHSTAR);
        operators.insert("#".to_string(), Operators::DIESE);
        operators.insert("?".to_string(), Operators::INTERROGATION);

        operators.insert("_".to_string(), Operators::UNDERSCORE);
        operators.insert("=>".to_string(), Operators::FATARROW);

        operators.insert("..".to_string(), Operators::DOTDOT);
        operators.insert("..=".to_string(), Operators::DOTDOTEQUAL);

        return operators;
    }

    #[allow(dead_code)]
    fn delimiters() -> HashMap<String, Delimiters> {
        let mut delimiters = HashMap::new();
        delimiters.insert("(".to_string(), Delimiters::LPAR);
        delimiters.insert(")".to_string(), Delimiters::RPAR);
        delimiters.insert("{".to_string(), Delimiters::LCURBRACE);
        delimiters.insert("}".to_string(), Delimiters::RCURBRACE);
        delimiters.insert("]".to_string(), Delimiters::RSBRACKET);
        delimiters.insert("[".to_string(), Delimiters::LSBRACKET);
        delimiters.insert(";".to_string(), Delimiters::SEMICOLON);
        delimiters.insert(":".to_string(), Delimiters::COLON);
        delimiters.insert(",".to_string(), Delimiters::COMMA);
        delimiters.insert(".".to_string(), Delimiters::DOT);
        delimiters.insert("...".to_string(), Delimiters::ELLIPSIS);
        delimiters.insert("::".to_string(), Delimiters::DOUBLECOLON);

        return delimiters;
    }

    pub fn get_token(&mut self) -> Option<TokenType>{
        todo!()
    }

    fn lex_number(&mut self) -> TokenType {
        todo!()
    }

    fn lex_hexadecimal(&mut self) -> TokenType {
        todo!()
    }

    fn lex_indentifier_or_keyword(&mut self) -> TokenType {
        todo!()
    }

    fn lex_string(&mut self) -> TokenType {
        todo!()
    }
    fn lex_operator(&mut self) -> TokenType {
        todo!()
    }
    fn lex_delimiter(&mut self) -> TokenType {
        todo!()
    }
    fn lex_comment(&mut self) -> TokenType {
        todo!()
    }
    pub fn tokenize(&mut self) -> Vec<Token>{
        todo!()
    }







}