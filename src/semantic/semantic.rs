// use std::collections::HashMap;
// use crate::parser::ast::{ASTNode, DataType, Declaration, Expression, Mutability, Parameter, Statement, TensorDataType};
// use crate::semantic::semantic_error::SemanticError;
//
// pub struct SemanticAnalyzer {
//     symbol_table: SymbolTable,
//     current_scope: usize,
// }
//
// pub struct SymbolTable {
//     scopes: Vec<Scope>,
// }
//
// pub struct Scope {
//     symbols: HashMap<String, Symbol>,
//     parent: Option<usize>,
// }
//
// #[derive(Clone, Debug)]
// pub enum Symbol {
//     Tensor {
//         dtype: TensorDataType,
//         shape: Option<Vec<usize>>,
//         mutable: bool,
//     },
//     Function {
//         parameters: Vec<Parameter>,
//         return_type: Option<DataType>,
//     },
//     Variable {
//         dtype: DataType,
//         mutable: bool,
//     },
// }
//
// impl SemanticAnalyzer {
//     pub fn new() -> Self {
//         let mut symbol_table = SymbolTable { scopes: Vec::new() };
//         // Créer le scope global
//         symbol_table.scopes.push(Scope {
//             symbols: HashMap::new(),
//             parent: None,
//         });
//
//         Self {
//             symbol_table,
//             current_scope: 0,
//         }
//     }
//
//     pub fn analyze(&mut self, ast: &ASTNode) -> Result<(), SemanticError> {
//         match ast {
//             ASTNode::Program(nodes) => {
//                 for node in nodes {
//                     self.analyze(node)?;
//                 }
//             },
//             ASTNode::Declaration(decl) => self.analyze_declaration(decl)?,
//             ASTNode::Statement(stmt) => self.analyze_statement(stmt)?,
//             ASTNode::Expression(expr) => {
//                 let _ = self.analyze_expression(expr)?;
//             },
//         }
//         Ok(())
//     }
//
//     fn analyze_declaration(&mut self, decl: &Declaration) -> Result<(), SemanticError> {
//         match decl {
//             Declaration::TensorDeclaration(tensor) => {
//                 // Vérifier que le nom n'est pas déjà utilisé dans le scope courant
//                 if self.symbol_exists(&tensor.name) {
//                     return Err(SemanticError::DuplicateSymbol(tensor.name.clone()));
//                 }
//
//                 // Vérifier et inférer le type du tenseur
//                 let inferred_type = self.check_tensor_type(&tensor.value)?;
//
//                 // Vérifier la correspondance entre la shape déclarée et la valeur
//                 if let Some(shape) = &tensor.shape {
//                     self.verify_tensor_shape(shape, &tensor.value)?;
//                 }
//
//                 // Ajouter le tenseur à la table des symboles
//                 self.add_symbol(tensor.name.clone(), Symbol::Tensor {
//                     dtype: tensor.dtype.clone(),
//                     shape: tensor.shape.as_ref().map(|s| s.dimensions.clone()),
//                     mutable: tensor.mutability == Mutability::Mutable,
//                 });
//             },
//             Declaration::FunctionDeclaration(func) => {
//                 // Créer un nouveau scope pour la fonction
//                 let function_scope = self.enter_scope();
//
//                 // Ajouter les paramètres au scope de la fonction
//                 for param in &func.parameters {
//                     self.add_symbol(param.name.clone(), Symbol::Variable {
//                         dtype: param.parameter_type.clone(),
//                         mutable: false,
//                     });
//                 }
//
//                 // Analyser le corps de la fonction
//                 for stmt in &func.body {
//                     self.analyze(stmt)?;
//                 }
//
//                 // Vérifier le type de retour
//                 if let Some(return_type) = &func.return_type {
//                     self.verify_return_type(&func.body, return_type)?;
//                 }
//
//                 // Quitter le scope de la fonction
//                 self.exit_scope();
//             },
//         }
//         Ok(())
//     }
//
//     fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), SemanticError> {
//         match stmt {
//             Statement::IfStatement(if_stmt) => {
//                 // Vérifier que la condition est un booléen
//                 let condition_type = self.analyze_expression(&if_stmt.condition)?;
//                 if !self.is_boolean_type(&condition_type) {
//                     return Err(SemanticError::TypeMismatch {
//                         expected: "boolean".to_string(),
//                         found: format!("{:?}", condition_type),
//                     });
//                 }
//
//                 // Analyser les blocs then/elif/else
//                 self.enter_scope();
//                 for stmt in &if_stmt.then_block {
//                     self.analyze(stmt)?;
//                 }
//                 self.exit_scope();
//
//                 for elif in &if_stmt.elif_block {
//                     let elif_condition_type = self.analyze_expression(&elif.condition)?;
//                     if !self.is_boolean_type(&elif_condition_type) {
//                         return Err(SemanticError::TypeMismatch {
//                             expected: "boolean".to_string(),
//                             found: format!("{:?}", elif_condition_type),
//                         });
//                     }
//
//                     self.enter_scope();
//                     for stmt in &elif.block {
//                         self.analyze(stmt)?;
//                     }
//                     self.exit_scope();
//                 }
//
//                 if let Some(else_block) = &if_stmt.else_block {
//                     self.enter_scope();
//                     for stmt in else_block {
//                         self.analyze(stmt)?;
//                     }
//                     self.exit_scope();
//                 }
//             },
//             // Ajouter d'autres statements...
//         }
//         Ok(())
//     }
//
//     fn analyze_expression(&mut self, expr: &Expression) -> Result<DataType, SemanticError> {
//         match expr {
//             Expression::BinaryOperation(bin_op) => {
//                 let left_type = self.analyze_expression(&bin_op.left)?;
//                 let right_type = self.analyze_expression(&bin_op.right)?;
//                 self.check_binary_operation(&bin_op.operator, &left_type, &right_type)
//             },
//             Expression::FunctionCall(call) => {
//                 // Vérifier que la fonction existe
//                 let func_symbol = self.lookup_symbol(&call.name)?;
//                 if let Symbol::Function { parameters, return_type } = func_symbol {
//                     // Vérifier les arguments
//                     if parameters.len() != call.arguments.len() {
//                         return Err(SemanticError::ArgumentCount {
//                             expected: parameters.len(),
//                             found: call.arguments.len(),
//                         });
//                     }
//
//                     // Vérifier les types des arguments
//                     for (param, arg) in parameters.iter().zip(&call.arguments) {
//                         let arg_type = self.analyze_expression(arg)?;
//                         if param.parameter_type != arg_type {
//                             return Err(SemanticError::TypeMismatch {
//                                 expected: format!("{:?}", param.parameter_type),
//                                 found: format!("{:?}", arg_type),
//                             });
//                         }
//                     }
//
//                     // Retourner le type de retour de la fonction
//                     Ok(return_type.unwrap_or(DataType::Void))
//                 } else {
//                     Err(SemanticError::NotAFunction(call.name.clone()))
//                 }
//             },
//             // Ajouter d'autres expressions...
//         }
//     }
// }