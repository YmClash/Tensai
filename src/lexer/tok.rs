use crate::lexer::lexer_error::LexerError;
use num_bigint::BigInt;

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum TokenType{
    IDENTIFIER{name: String},
    INTEGER{value: BigInt},
    FLOAT{value: f64},


    // INTEGER(IntegerType),
    // FLOAT(FloatType),

    // COMPLEX(ComplexType),

    HEXADECIMAL {value: u64},
    STRING {value: String,kind: StringKind},
    CHAR {value: char},
    EOF,
    NEWLINE,
    OPERATOR(Operators),
    KEYWORD(Keywords),
    DELIMITER(Delimiters),
    COMMENT(String),
    ERROR(LexerError),
    UNKNOWN,
    DOCSTRING {value: String},



}

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum Operators{
    PLUS,            // '+' PLUS / PLUS SIGN
    MINUS,           // '-' MOINS / MINUS SIGN
    STAR,            // '*' ETOILE / STAR
    SLASH,           // '/' SLASH / SLASH
    VBAR,            // '|' BARRE VERTICALE / VERTICAL BAR
    AMPER,           // '&' ET COMMERCIAL / AMPERSAND
    LESS,            // '<' INFERIEUR / LESS-THAN SIGN
    GREATER,         // '>' SUPERIEUR / GREATER-THAN SIGN
    EQUAL,           // '=' EGAL / EQUALS SIGN
    PERCENT,         // '%' POURCENTAGE / PERCENT
    EQEQUAL,         // '==' EGAL EGAL / EQUALS EQUALS
    NOTEQUAL,        // '!=' DIFFERENT / NOT EQUAL
    LESSEQUAL,       // '<=' INFERIEUR EGAL / LESS-THAN EQUAL
    FATARROW,        // '=>' IMPLIQUE / IMPLIES
    GREATEREQUAL,    // '>=' SUPERIEUR EGAL / GREATER-THAN EQUAL
    TILDE,           // '~' TILDE / TILDE
    CIRCUMFLEX,      // '^' CIRCONFLEXE / CIRCUMFLEX
    LEFTSHIFT,       // '<<' DECALAGE GAUCHE / LEFT SHIFT
    RIGHTSHIFT,      // '>>' DECALAGE DROITE / RIGHT SHIFT
    DOUBLESTAR,      // '**' DOUBLE ÉTOILE / DOUBLE STAR
    PLUSEQUAL,       // '+=' PLUS EGAL / PLUS EQUAL
    MINEQUAL,        // '-=' MOINS EGAL / MINUS EQUAL
    STAREQUAL,       // '*=' ETOILE EGAL / STAR EQUAL
    SLASHEQUAL,      // '/=' SLASH EGAL / SLASH EQUAL
    PERCENTEQUAL,    // '%=' POURCENTAGE EGAL / PERCENT EQUAL
    AMPEREQUAL,      // '&=' ET COMMERCIAL EGAL / AMPERSAND EQUAL
    VBAREQUAL,       // '|=' BARRE VERTICALE EGAL / VERTICAL BAR EQUAL
    CIRCUMFLEXEQUAL, // '^=' CIRCONFLEXE EGAL / CIRCUMFLEX EQUAL
    LEFTSHIFTEQUAL,  // '<<=' DECALAGE GAUCHE EGAL / LEFT SHIFT EQUAL
    RIGHTSHIFTEQUAL, // '>>=' DECALAGE DROITE EGAL / RIGHT SHIFT EQUAL
    DOUBLESTAREQUAL, // '**=' DOUBLE ETOILE EGAL / DOUBLE STAR EQUAL
    AND,             // '&&' ET ET / AND
    OR,              // '||' OU OU / OR


    //DOUBLESLASH, // '//' DOUBLE SLASH / DOUBLE SLASH
    DOUBLESLASHEQUAL, // '//=' DOUBLE SLASH EGAL / DOUBLE SLASH EQUAL

    ATEQUAL,          // '@=' AROBASE EGAL / AT EQUAL
    RARROW,           // '->' FLECHE DROITE / RIGHT ARROW
    //   ELLIPSIS, // '...' POINTS DE SUSPENSION / ELLIPSIS
    COLONEQUAL,    // ':=' DEUX POINT EGAL / COLON EQUAL
    // STARSLASH,     // '*/' ETOILE SLASH / STAR SLASH
    // SLASHSTAR,     // '/*' SLASH ETOILE / SLASH STAR
    DIESE,         // '#' DIESE / HASH
    EXCLAMATION,   // '!' POINT D'EXCLAMATION / EXCLAMATION POINT
    INTERROGATION, // '?'
    UNDERSCORE,    // '_'

    DOTDOT,      // '..' DEUX POINTS / DOUBLE DOT  // Range Operator
    DOTDOTEQUAL, // '..=' DEUX POINTS EGAL / DOUBLE DOT EQUAL RangeInclusive Operator


    AT,               // '@' AROBASE / AT// TENSORPROD,               // '@' AROBASE / AT
    TRANSPOSE,   // "'" TRANSPOSE / TRANSPOSE ou  "'"
    CONTRACT,    //  "#>"          // Contraction complète
    CONTRACTDIM, //  "#[0-9]+>"    // Contraction sur dimensions spécifiques

}

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum Keywords {
    AND,AS,ASSERT,BREAK,BLOCK, CONST, CLASS, CONTINUE, CACHE, CPU,DEL, ELIF, ELSE, ENUM, EXCEPT,
    EXPAND, FALSE, FINALLY, FN, FOR, GPU, IF, IMPL, IN, IS, LAMBDA, LET, LOOP, MATCH, MOD, MUT,NONE,
    NOT, ON, OR, PARALLEL, PUB, PASS,RETURN, SELF, STRUCT, TENSOR, TRUE, TYPE, TYPEOF, USE, WHERE, WHILE, YIELD,

    //TYPE KEYWORDS
    I32, I64, F32, F64, C32, C64, STR, CHAR, BOOL,

    //
    INT,FLOAT,COMPLEX,STRING, //TYPE KEYWORDS

    //gestion de memoire
    USING,ALLOC,FREE,

    //new keywords
    OPTIM,   //optimisation
    MATRIX,  //matrice
    VECTOR,  //vecteur
    SCALAR,  //scalaire
    NAN,     //not a number
    INF,     //infini
    NANF,    //not a number float
    INFF,    //infini float
    DEVICE,  //device




}

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum Delimiters{
    //DELIMITERS,
    LPAR,        // '(' PARANTHESE GAUCHE / LEFT PARENTHESIS
    RPAR,        // ')' PARANTHESE DROITE / RIGHT PARENTHESIS
    LSBRACKET,   // '[' CROCHET GAUCHE / LEFT SQUARE BRACKET
    RSBRACKET,   // ']' CROCHET DROIT / RIGHT SQUARE BRACKET
    COLON,       // ':' DEUX POINT / COLON
    COMMA,       // ',' VIRGULE / COMMA
    SEMICOLON,   // ';' POINT VIRGULE / SEMICOLON
    DOT,         // '.' POINT / DOT
    LCURBRACE,   // '{' ACCOLADE GAUCHE / LEFT CURLY BRACKET
    RCURBRACE,   // '}' ACCOLADE DROITE / RIGHT CURLY BRACKET
    ELLIPSIS,    // '...' POINTS DE SUSPENSION / ELLIPSIS
    DOUBLECOLON, // '::' DEUX POINTS / DOUBLE COLON

}

// #[allow(dead_code)]
// #[derive(Debug, PartialEq, Clone)]
// pub enum TensorType {
//     Scalar,
//     Vector,
//     Matrix,
//     Tensor(usize),  // usize représente le rang du tenseur
// }

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub struct Identifier{
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum  FloatType{
    Float32(f32),
    Float64(f64),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum IntegerType{
    Int32(i32),
    Int64(i64),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum ComplexType{
    Complex32{real: f32, imag: f32},
    Complex64{real: f64, imag: f64},
}

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum StringKind{
    NORMAL,
    FORMATTED,
    UNICODE,
}

//////////////////////////TOKEN//////////TENSAI/////////////////////////////////
////////////////////////////////by:YmC////////////////////////////////////////