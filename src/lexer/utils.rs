use crate::lexer::lex::Lexer;
use crate::lexer::lexer_error::{LexerError, LexerErrorType, Position};
use crate::lexer::tok::TokenType;

impl<'a> Lexer<'a> {

    // 1er Methode pour avancer d'un character
    // fn advance(&mut self) -> char {
    //     let ch = self.source.next().unwrap();
    //     if ch == '\n' {
    //         self.current_line += 1; // Incrémenter le numéro de ligne
    //         self.current_column = 1; // Réinitialiser le numéro de colonne
    //     } else {
    //         self.current_column += 1; // sinon incrementer le numero de colonne
    //     }
    //     ch
    // }


    // Methode pour avancer d'un caractère
    #[allow(dead_code)]
    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.source.next()?;
        self.current_char = ch;
        if ch == '\n' {
            self.current_line += 1;
            self.current_column = 1;
            self.at_line_start = true;
        } else {
            self.current_column += 1;
            self.at_line_start = false;
        }
        Some(ch)
    }

    // Methode pour regarder le prochain caractere sans avancer
    #[allow(dead_code)]
    pub fn peek_char(&mut self) -> Option<char>{
        self.source.peek().copied()
    }


    ///2eme Methode pour avancer de 2eme caractères sans avancer
    #[allow(dead_code)]
    pub fn peek_next_char(&mut self) -> Option<char>{
        self.source.clone().nth(1)
    }


    #[allow(dead_code)]
    pub fn skip_whitespace(&mut self){
        while let Some(&ch) = self.source.peek() {
            if ch.is_whitespace() && ch != '\n'{
                // self.advance();
                self.next_char();
                self.at_line_start = false;
            }else { break; }
        }
    }

    #[allow(dead_code)]
    pub fn is_hex_digit(ch: char) -> bool {
        ch.is_digit(16)
    }

    #[allow(dead_code)]
    pub fn create_error(&self, error: LexerErrorType) -> TokenType {
        let position = Position {
            line: self.current_line,
            column: self.current_column,
        };
        TokenType::ERROR(LexerError::new(error.clone(), error.to_string(), position))
    }








}