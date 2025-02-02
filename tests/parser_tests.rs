// #[cfg(test)]
// mod tests {
//     use tensai::lexer::lex::Lexer;
//     use tensai::parser::parser::Parser;
//     use tensai::parser::ast::{
//         Expression, Literal, ASTNode, TensorDataType, TensorDeclaration, Shape, Visibility
//     };
//     use tensai::parser::parser_error::ParserError;
//     use tensai::parser::ParserError;
//
//     fn parse_input(input: &str) -> Result<TensorDeclaration, ParserError> {
//         let mut lexer = Lexer::new(input);
//         let tokens = lexer.tokenize()?;
//         let mut parser = Parser::new(tokens);
//         if let ASTNode::TensorDecl(tensor_decl) = parser.parse_tensor_declaration(Visibility::Public)? {
//             Ok(tensor_decl)
//         } else {
//             panic!("Expected tensor declaration")
//         }
//     }
//
//     #[test]
//     fn test_simple_tensor_declaration() -> Result<(), ParserError> {
//         let input = "tensor A:f32 @shape(2,2) = [1.0, 2.0; 3.0, 4.0]";
//         let result = parse_input(input)?;
//         assert_eq!(result.identifier, "A");
//         assert_eq!(result.tensor_type, TensorDataType::F32);
//         assert_eq!(result.shape, Some(Shape { dimensions: vec![2, 2] }));
//         Ok(())
//     }
//
//     #[test]
//     fn test_nested_tensor_declaration() -> Result<(), ParserError> {
//         let input = "tensor B:f32 @shape(2,2) = [[1.0, 2.0], [3.0, 4.0]]";
//         let result = parse_input(input)?;
//         if let Expression::Array { elements, dimensions } = result.initializer {
//             assert_eq!(dimensions, Some(vec![2, 2]));
//             assert_eq!(elements.len(), 4);
//         } else {
//             panic!("Expected array expression");
//         }
//         Ok(())
//     }
//
//     #[test]
//     fn test_3d_tensor_flat() -> Result<(), ParserError> {
//         let input = "tensor C:f32 @shape(2,2,2) = [1.0, 2.0; 3.0, 4.0; 5.0, 6.0; 7.0, 8.0]";
//         let result = parse_input(input)?;
//         if let Expression::Array { elements, dimensions } = result.initializer {
//             assert_eq!(dimensions, Some(vec![2, 2, 2]));
//             assert_eq!(elements.len(), 8);
//         } else {
//             panic!("Expected array expression");
//         }
//         Ok(())
//     }
//
//     #[test]
//     fn test_integer_tensor() -> Result<(), ParserError> {
//         let input = "tensor E:i32 @shape(2,2) = [[1, 2], [3, 4]]";
//         let result = parse_input(input)?;
//         assert_eq!(result.tensor_type, TensorDataType::I32);
//         if let Expression::Array { elements, .. } = result.initializer {
//             for element in elements.iter() {
//                 match element {
//                     Expression::Literal(Literal::Integer(_)) => (),
//                     _ => panic!("Expected integer literals"),
//                 }
//             }
//         } else {
//             panic!("Expected array expression");
//         }
//         Ok(())
//     }
//
//     #[test]
//     fn test_shape_dimension_mismatch() {
//         let input = "tensor F:f32 @shape(2,3) = [[1.0, 2.0], [3.0, 4.0]]";
//         assert!(parse_input(input).is_err(), "Expected error for shape mismatch");
//     }
//
//     #[test]
//     fn test_invalid_tensor_syntax() {
//         let cases = vec![
//             "tensor A @shape(2,2) = [1.0, 2.0]",
//             "tensor A:f32 = [1.0, 2.0]",
//             "tensor A:f32 @shape(2,2) = [1.0, 2.0, 3.0]",
//             "tensor A:invalid @shape(2,2) = [1.0, 2.0]",
//         ];
//
//         for case in cases {
//             assert!(parse_input(case).is_err(), "Expected error for invalid syntax: {}", case);
//         }
//     }
//
//     #[test]
//     fn test_tensor_type_validation() {
//         let input = "tensor G:i32 @shape(2,2) = [[1.5, 2.0], [3.0, 4.0]]";
//         assert!(parse_input(input).is_err(), "Expected error for float values in integer tensor");
//     }
//
//     #[test]
//     fn test_tensor_without_dimensions() -> Result<(), ParserError> {
//         let input = "tensor H:f32 @shape() = [1.0]";
//         let result = parse_input(input)?;
//         if let Expression::Array { elements, dimensions } = result.initializer {
//             assert_eq!(dimensions, Some(vec![1]));
//             assert_eq!(elements.len(), 1);
//         } else {
//             panic!("Expected array expression");
//         }
//         Ok(())
//     }
// }