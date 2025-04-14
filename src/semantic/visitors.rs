use crate::parser::ast::ASTNode;
use crate::semantic::semantic::SemanticAnalyzer;
use crate::semantic::semantic_error::{Position, SemanticError};
use crate::semantic::semantic_error::SemanticErrorType::RootError;

impl SemanticAnalyzer{

    // ici  on vas utilise un visiteur  pour parcourir l'AST

    pub fn analyse_program(&mut self, program:&ASTNode) -> Result<(),SemanticError>{

        // match program {
        //     ASTNode::Program(nodes) =>{
        //         for node in nodes {
        //             self.analyse_program(node)?;
        //         }
        //         Ok(())
        //     }
        //     _ => Err(SemanticError::new(RootError)),
        // }
        todo!()
    }

    fn analyze_node(&mut self, node: &ASTNode) -> Result<(), SemanticError> {
        todo!()
    }






}