// use crate::parser::parser_error::Position;
//
// #[allow(dead_code)]
// #[derive(Debug, Clone, PartialEq)]
// pub struct Span {
//     pub start: Position,
//     pub end: Position,
// }
//
// #[allow(dead_code)]
// #[derive(Debug, Clone, PartialEq)]
// pub struct SemanticError {
//     pub error: SemanticErrorType,
//     pub message: String,
//     pub span: Span,
// }
//
//
// #[allow(dead_code)]
// #[derive(Debug, Clone, PartialEq)]
// pub enum SemanticErrorType{
//     UndefinedVariable,
//     UndefinedFunction,
//     UndefinedTensor,
//     UndefinedType,
//     UndefinedSymbol,
//     InvalidType,
//     InvalidShape,
//
// }