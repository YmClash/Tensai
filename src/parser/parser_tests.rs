#[cfg(test)]
mod tests {
    use crate::lexer::lex::Lexer;
    use super::*;

    use crate::parser::ast::{ASTNode, DataType, Shape, TensorDataType, TensorDeclaration};
    use crate::parser::ast::ASTNode::Expression;
    use crate::parser::parser::Parser;
    use crate::parser::parser_error::ParserError;

    fn parse_input(input: &str) -> Result<ASTNode, ParserError> {
        let mut lexer = Lexer::new(input);
        // let tokens = lexer.tokenize()?;
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);


        parser.parse_tensor_declaration()
    }

    #[test]
    fn test_simple_tensor_declaration() {
        let input = "tensor A:f32 @shape(2,2) = [1.0, 2.0; 3.0, 4.0]";
        let result = parse_input(input).unwrap();
        assert_eq!(result.name, "A");
        assert_eq!(result.dtype, DataType::F32);
        assert_eq!(result.shape, Some(Shape { shape: vec![2, 2] }));
    }

    #[test]
    fn test_nested_tensor_declaration() {
        let input = "tensor B:f32 @shape(2,2) = [[1.0, 2.0], [3.0, 4.0]]";
        let result = parse_input(input).unwrap();
        match result.value {
            Expression::Array(array) => {
                assert_eq!(array.dimensions, Some(vec![2, 2]));
                assert_eq!(array.elements.len(), 4);
            },
            _ => panic!("Expected array expression"),
        }
    }

    #[test]
    fn test_3d_tensor_flat() {
        let input = "tensor C:f32 @shape(2,2,2) = [1.0, 2.0; 3.0, 4.0; 5.0, 6.0; 7.0, 8.0]";
        let result = parse_input(input).unwrap();
        match result.value {
            Expression::Array(array) => {
                assert_eq!(array.dimensions, Some(vec![2, 2, 2]));
                assert_eq!(array.elements.len(), 8);
            },
            _ => panic!("Expected array expression"),
        }
    }

    #[test]
    fn test_3d_tensor_nested() {
        let input = "tensor D:f32 @shape(2,2,2) = [[[1.0, 2.0], [3.0, 4.0]], [[5.0, 6.0], [7.0, 8.0]]]";
        let result = parse_input(input).unwrap();
        match result.value {
            Expression::Array(array) => {
                assert_eq!(array.dimensions, Some(vec![2, 2, 2]));
                assert_eq!(array.elements.len(), 8);
            },
            _ => panic!("Expected array expression"),
        }
    }

    #[test]
    fn test_integer_tensor() {
        let input = "tensor E:i32 @shape(2,2) = [[1, 2], [3, 4]]";
        let result = parse_input(input).unwrap();
        assert_eq!(result.dtype, TensorDataType::I32);
        match result.value {
            Expression::Array(array) => {
                for element in array.elements.iter() {
                    match element {
                        Expression::Literal(Literal::Integer { .. }) => (),
                        _ => panic!("Expected integer literals"),
                    }
                }
            },
            _ => panic!("Expected array expression"),
        }
    }

    #[test]
    fn test_shape_dimension_mismatch() {
        let input = "tensor F:f32 @shape(2,3) = [[1.0, 2.0], [3.0, 4.0]]";
        let result = parse_input(input);
        assert!(result.is_err(), "Expected error for shape mismatch");
    }

    #[test]
    fn test_invalid_tensor_syntax() {
        let cases = vec![
            // Pas de type
            "tensor A @shape(2,2) = [1.0, 2.0]",
            // Pas de shape
            "tensor A:f32 = [1.0, 2.0]",
            // Tableau incomplet
            "tensor A:f32 @shape(2,2) = [1.0, 2.0, 3.0]",
            // Type invalide
            "tensor A:invalid @shape(2,2) = [1.0, 2.0]",
        ];

        for case in cases {
            let result = parse_input(case);
            assert!(result.is_err(), "Expected error for invalid syntax: {}", case);
        }
    }

    #[test]
    fn test_tensor_type_validation() {
        let input = "tensor G:i32 @shape(2,2) = [[1.5, 2.0], [3.0, 4.0]]";
        let result = parse_input(input);
        assert!(result.is_err(), "Expected error for float values in integer tensor");
    }

    #[test]
    fn test_tensor_without_dimensions() {
        let input = "tensor H:f32 @shape() = [1.0]";
        let result = parse_input(input).unwrap();
        match result.value {
            Expression::Array(array) => {
                assert_eq!(array.dimensions, Some(vec![1]));
                assert_eq!(array.elements.len(), 1);
            },
            _ => panic!("Expected array expression"),
        }
    }

    #[test]
    fn test_large_tensor() {
        let mut elements = Vec::new();
        for i in 0..100 {
            elements.push(i.to_string());
        }
        let input = format!(
            "tensor I:i32 @shape(10,10) = [{}]",
            elements.join(",")
        );
        let result = parse_input(&input).unwrap();
        match result.value {
            Expression::Array(array) => {
                assert_eq!(array.dimensions, Some(vec![10, 10]));
                assert_eq!(array.elements.len(), 100);
            },
            _ => panic!("Expected array expression"),
        }
    }
}