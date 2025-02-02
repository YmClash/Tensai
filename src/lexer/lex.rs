use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::tok::{Delimiters, Keywords, Operators, StringKind, TokenType};
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
            current_column: 0,
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
        keywords.insert("assert".to_string(),Keywords::ASSERT);
        keywords.insert("break".to_string(),Keywords::BREAK);
        keywords.insert("class".to_string(),Keywords::CLASS);
        keywords.insert("continue".to_string(),Keywords::CONTINUE);
        keywords.insert("cache".to_string(),Keywords::CACHE);
        keywords.insert("del".to_string(),Keywords::DEL);
        keywords.insert("elif".to_string(),Keywords::ELIF);
        keywords.insert("else".to_string(),Keywords::ELSE);
        keywords.insert("enum".to_string(),Keywords::ENUM);
        keywords.insert("except".to_string(),Keywords::EXCEPT);
        keywords.insert("False".to_string(),Keywords::FALSE);
        keywords.insert("finally".to_string(),Keywords::FINALLY);
        keywords.insert("for".to_string(),Keywords::FOR);
        keywords.insert("fn".to_string(),Keywords::FN);
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
        keywords.insert("tpu".to_string(),Keywords::TPU);
        keywords.insert("type".to_string(),Keywords::TYPE);
        keywords.insert("typeof".to_string(),Keywords::TYPEOF);
        keywords.insert("use".to_string(),Keywords::USE);
        keywords.insert("where".to_string(),Keywords::WHERE);

        keywords.insert("while".to_string(),Keywords::WHILE);

        keywords.insert("yield".to_string(),Keywords::YIELD);

        //TYPE KEYWORDS
        keywords.insert("optim".to_string(),Keywords::OPTIM);
        keywords.insert("matrix".to_string(),Keywords::MATRIX);
        keywords.insert("vector".to_string(),Keywords::VECTOR);
        keywords.insert("scalar".to_string(),Keywords::SCALAR);
        keywords.insert("nan".to_string(),Keywords::NAN);
        keywords.insert("inf".to_string(),Keywords::INF);
        keywords.insert("nanf".to_string(),Keywords::NANF);
        keywords.insert("inff".to_string(),Keywords::INFF);
        keywords.insert("device".to_string(),Keywords::DEVICE);
        keywords.insert("shape".to_string(),Keywords::SHAPE);

        keywords.insert("i32".to_string(),Keywords::I32);
        keywords.insert("i64".to_string(),Keywords::I64);
        keywords.insert("f32".to_string(),Keywords::F32);
        keywords.insert("f64".to_string(),Keywords::F64);


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
        operators.insert("'".to_string(), Operators::TRANSPOSE);
        operators.insert("#>".to_string(), Operators::CONTRACT);



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
        self.skip_whitespace();
        // self.skip_comments();

        match self.peek_char() {
            Some('\n') => {
                // self.advance(); // Consomme le '\n'
                self.next_char();
                self.at_line_start = true;
                return Some(TokenType::NEWLINE);
            }
            Some('0'..='9') => Some(self.lex_number()),
            Some('a'..='z') | Some('A'..='Z') | Some('_') => Some(self.lex_identifier_or_keyword()),
            Some('"') | Some('\'') => Some(self.lex_string()),
            Some('#') => Some(self.lex_operator().unwrap()),
            Some('/') => {
                if let Some(next_char) = self.peek_next_char() {
                    match next_char {
                        '/' | '*' => Some(self.lex_comment()),
                        _ => self.lex_operator(),
                    }
                } else {
                    self.lex_operator()
                }
            }
            Some(ch) if self.delimiters.contains_key(&ch.to_string()) => Some(self.lex_delimiter()),
            Some(ch) if !ch.is_alphanumeric() => self.lex_operator(),
            None => Some(TokenType::EOF),   //Ajouter nouvelement
            _ => Some(self.lex_unknown()),
        }


    }

    fn lex_number(&mut self) -> TokenType {
        self.current_token_text.clear();


        if self.peek_char() == Some('0') {
            if let Some(next_char) = self.peek_next_char() {
                if next_char == 'x' || next_char == 'X' {
                    // let ch1 = self.advance(); // '0'
                    // let ch2 = self.advance(); // 'x' ou 'X'
                    let ch1 = self.next_char().unwrap();
                    let ch2 = self.next_char().unwrap();
                    self.current_token_text.push(ch1);
                    self.current_token_text.push(ch2);
                    return self.lex_hexadecimal();
                }
            }
        }

        let mut number = String::new();
        let mut dot_count = 0;

        while let Some(&ch) = self.source.peek(){
            if ch.is_digit(10) {
                let digit = self.next_char().unwrap();
                number.push(digit);
                self.current_token_text.push(digit);
            } else if ch == '.' {
                // Vérifier si le '.' est suivi d'un chiffre pour un nombre flottant
                if dot_count == 0 {
                    if let Some(next_ch) = self.peek_next_char() {
                        if next_ch.is_digit(10) {
                            // let dot = self.advance();
                            let dot = self.next_char().unwrap();
                            number.push(dot);
                            self.current_token_text.push(dot);
                            dot_count += 1;
                        } else {
                            // Le '.' n'est pas suivi d'un chiffre, il pourrait faire partie d'un opérateur
                            break;
                        }
                    } else {
                        // Fin de l'entrée après '.', ce qui est une erreur pour un float
                        break;
                    }
                } else {
                    // Deuxième point trouvé, c'est une erreur pour un float
                    break;
                }
            } else {
                break;
            }
        }

        if number.is_empty() {
            return self.create_error(LexerErrorType::InvalidInteger(number));
        }

        if dot_count > 0 {
            match number.parse::<f64>() {
                Ok(value) => TokenType::FLOAT { value },
                Err(_) => self.create_error(LexerErrorType::InvalidFloat(number)),
            }
        } else {
            match number.parse::<i64>() {
                Ok(value) => TokenType::INTEGER {
                    value: value.into(),
                },
                Err(_) => self.create_error(LexerErrorType::InvalidInteger(number)),
            }
        }

    }


    fn lex_hexadecimal(&mut self) -> TokenType {
        let mut hex_number = self.current_token_text.clone(); // Déjà contient "0x" ou "0X"

        while let Some(&ch) = self.source.peek() {
            if Self::is_hex_digit(ch) {
                hex_number.push(self.next_char().unwrap());
            } else {
                break;
            }
        }

        self.current_token_text = hex_number.clone();

        if hex_number.len() == 2 {
            // Seulement "0x" ou "0X"
            self.create_error(LexerErrorType::InvalidHexadecimal(hex_number))
        } else {
            match u64::from_str_radix(&hex_number[2..], 16) {
                // Skip "0x"
                Ok(value) => TokenType::HEXADECIMAL { value },
                Err(_) => self.create_error(LexerErrorType::InvalidHexadecimal(hex_number)),
            }
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> TokenType {
        self.current_token_text.clear();
        while let Some(&ch) = self.source.peek(){
            if ch.is_alphanumeric() || ch =='_'{
                let ch = self.next_char().unwrap();
                self.current_token_text.push(ch);
            }else { break; }
        }
        if let Some(keyword) = self.keywords.get(&self.current_token_text) {
            TokenType::KEYWORD(keyword.clone()) // c'est un mot clé
        } else {
            TokenType::IDENTIFIER {
                name: self.current_token_text.clone(),
            } // sinon c'est un identifiant
        }
    }


    fn lex_string(&mut self) -> TokenType {
        self.current_token_text.clear();

        let quote = self.next_char().unwrap();
        let mut value = String::new();
        let mut is_escaped = false;

        while let Some(&ch) = self.source.peek(){
            self.next_char().unwrap();
            if is_escaped {
                match ch {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    '\n' => {
                        // Ignorer le saut de ligne après un backslash
                        while let Some(&next_ch) = self.source.peek() {
                            if next_ch.is_whitespace() && next_ch != '\n' {
                                // self.advance();
                                self.next_char();
                            } else {
                                break;
                            }
                        }
                    }
                    _ => value.push(ch),
                }
                is_escaped = false;
            } else if ch == '\\' {
                is_escaped = true;
            }

            else if ch == '\'' && value.len() == 1 {
                self.current_token_text = value.clone();
                return TokenType::CHAR {
                    value: value.chars().next().unwrap()
                }
            }

            else if ch == quote {
                self.current_token_text = value.clone();
                return TokenType::STRING {
                    value,
                    kind: StringKind::NORMAL,
                };
            } else {
                value.push(ch);
            }
        }

        // Si nous sortons de la boucle sans avoir trouvé de guillemet fermant
        self.create_error(LexerErrorType::UnterminatedString)
    }
    fn lex_operator(&mut self) -> Option<TokenType> {
        self.current_token_text.clear();

        // Regardez les deux prochains caractères pour vérifier les opérateurs composés
        let first_char = self.next_char().unwrap(); // self.advance();
        self.current_token_text.push(first_char);
        let mut op = self.current_token_text.clone();

        if let Some(&next_char) = self.source.peek() {
            op.push(next_char);
            if self.operators.contains_key(&op) {
                self.next_char().unwrap(); // self.advance();
                self.current_token_text.push(next_char);
                return Some(TokenType::OPERATOR(self.operators[&op].clone()));
            }
        }

        // Si ce n'est pas un opérateur composé, vérifiez si c'est un opérateur simple
        if let Some(operator) = self.operators.get(&self.current_token_text){
            return Some(TokenType::OPERATOR(operator.clone()));
        }
        // Si l'opérateur n'est pas reconnu, retournez une erreur
        Some(TokenType::ERROR(LexerError::invalid_token(
            &self.current_token_text,
            Position {
                line: self.current_line,
                column: self.current_column,
            },
        )))
    }

    fn lex_delimiter(&mut self) -> TokenType {
        self.current_token_text.clear();

        // let first_char = self.advance();
        let first_char = self.next_char().unwrap();
        self.current_token_text.push(first_char);

        if let Some(&next_char) = self.source.peek(){
            let mut combined = self.current_token_text.clone();
            combined.push(next_char);

            // verifie pour "::"
            if combined == "::"{
                // self.advance();
                self.next_char();
                self.current_token_text = combined;
                return TokenType::DELIMITER(Delimiters::DOUBLECOLON);
            }
            if first_char == '.' {
                if let Some(&next_char) = self.source.peek(){
                    if next_char == '.'{
                        // self.advance();
                        self.next_char();
                        self.current_token_text.push(next_char);

                        if let Some(&third_char) = self.source.peek(){
                            if third_char == '.'{
                                // self.advance();
                                self.next_char();
                                self.current_token_text.push(third_char);
                                return TokenType::DELIMITER(Delimiters::ELLIPSIS);
                            }else if third_char == '=' {
                                // self.advance();
                                self.next_char();
                                self.current_token_text.push(third_char);
                                return TokenType::OPERATOR(Operators::DOTDOTEQUAL);
                            }
                        }
                        return TokenType::OPERATOR(Operators::DOTDOT);
                    }else {
                        return TokenType::DELIMITER(Delimiters::DOT);
                    }
                }
            }

        }
        if let Some(delimiter) = self.delimiters.get(&self.current_token_text) {
            return TokenType::DELIMITER(delimiter.clone());
        } else {
            return TokenType::UNKNOWN;
        }

    }
    fn lex_comment(&mut self) -> TokenType {
        self.current_token_text.clear();
        let mut comment = String::new();

        // Skip the initial '#' or '/'
        self.next_char();

        match self.peek_char() {
            // Pour les commentaires sur une ligne
            // Some('/') /*| Some('#')*/ => {
            //     self.next_char(); // Skip second '/' or '#'
            //     while let Some(&ch) = self.source.peek() {
            //         if ch == '\n' { break; }
            //         comment.push(self.next_char().unwrap());
            //     }
            //     TokenType::COMMENT(comment)
            // },
            // Pour les commentaires multi-lignes
            Some('*') => {
                self.next_char(); // Skip '*'
                let mut ended = false;
                while let Some(&ch) = self.source.peek() {
                    if ch == '*' {
                        self.next_char();
                        if let Some('/') = self.peek_char() {
                            self.next_char();
                            ended = true;
                            break;
                        }
                        comment.push('*');
                    } else {
                        comment.push(self.next_char().unwrap());
                    }
                }
                if !ended {
                    self.create_error(LexerErrorType::UnterminatedComment)
                } else {
                    TokenType::COMMENT(comment)
                }
            },
            _ => TokenType::COMMENT(comment)
        }
        // todo!()
    }


    fn lex_unknown(&mut self) -> TokenType {
        // let ch = self.advance();
        let ch = self.next_char().unwrap();
        self.current_token_text = ch.to_string();
        TokenType::UNKNOWN
    }


    /// C'est la deuxième methode principal avec get_token() pour obtenir les tokens
    /// Son role c'est de tokeniser le code source
    /// appel la methode get_token pour obtenir les tokens.
    /// Elle crée objet Token pour chaque TokenType retourné par get_token()
    /// elle retourne un vecteur de tokens Vec<Token>
    /// methode pour tokeniser le code source
    pub fn tokenize(&mut self) -> Vec<Token>{
        let mut tokens = Vec::new();
        while let Some(token_type) = self.get_token() {
            let token = Token::new(
                self.current_token_text.clone(),
                token_type.clone(),
                self.current_line,
                self.current_column,
            );
            tokens.push(token);
            self.current_token_text.clear();
            if matches!(token_type, TokenType::EOF) {
                break;
            }
        }
        return tokens;
    }


    fn lex_complex_number(&mut self) -> TokenType {
        self.current_token_text.clear();
        let mut real_part = String::new();
        let mut imag_part = String::new();
        let mut has_imag = false;

        while let Some(&ch) = self.source.peek() {
            if ch.is_digit(10) || ch == '.' || ch == '+' || ch == '-' {
                real_part.push(self.next_char().unwrap());
            } else if ch == 'i' {
                has_imag = true;
                self.next_char();
                break;
            } else {
                break;
            }
        }

        if has_imag {
            while let Some(&ch) = self.source.peek() {
                if ch.is_digit(10) || ch == '.' {
                    imag_part.push(self.next_char().unwrap());
                } else {
                    break;
                }
            }
        }

        if real_part.is_empty() || imag_part.is_empty() {
            self.create_error(LexerErrorType::InvalidFloat(format!("{}+{}i", real_part, imag_part)))
        } else {
            TokenType::COMPLEX {
                real: real_part.parse::<f64>().unwrap(),
                imag: imag_part.parse::<f64>().unwrap(),
            }
        }
    }





    /////////////////////////////////by YmC///////////////////////////////////////////////


}