use crate::parser::ast::TensorType;
use crate::semantic::semantic::SemanticAnalyzer;

impl SemanticAnalyzer{
    // Exemple de vérification dans l'analyseur sémantique
    // fn verify_type(&self, expected: Type, got: Type) -> Result<(), TypeError> {
    //     if expected != got {
    //         Err(TypeError::TypeMismatch {
    //             expected,
    //             got,
    //             span: /* position dans le code */
    //         })
    //     } else {
    //         Ok(())
    //     }
    // }
    //
    // fn verify_binary_op(&self, op: BinaryOp, lhs: Type, rhs: Type) -> Result<Type, TypeError> {
    //     match op {
    //         BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
    //             if lhs == rhs {
    //                 Ok(lhs)
    //             } else {
    //                 Err(TypeError::IncompatibleOperands { op, lhs, rhs })
    //             }
    //         }
    //         // ... autres opérateurs
    //     }
    // }


}