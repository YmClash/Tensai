use num_bigint::BigInt;
use std::fmt;
use crate::parser::ast::{ArrayExpression, ArrayRepeatExpression, ArraySlice, BinaryOperation, Body, BreakStatement, CompoundAssignment, Conditional, ContinueStatement, DataType, ForStatement, FunctionCall, IfStatement, LambdaExpression, Literal, LoopStatement, MemberAccess, MethodCall, ModuleImportStatement, Mutability, RangeExpression, ReturnStatement, SpecificImportStatement, TypeCast, UnaryOperation, UseStatement, Visibility, WhileStatement};

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Declaration(Declaration),
    Expression(Expression),
    Statement(Statement),
    Error(ParserError),
    Body(Body),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Device {
    CPU,
    GPU(usize),
    TPU,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TensorLayout {
    Contiguous,
    Strided(Vec<usize>),
    Sparse,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TensorMetadata {
    pub dtype: TensorDataType,
    pub shape: Vec<usize>,
    pub tensor_type: TensorType,
    pub is_contiguous: bool,
    pub device: Device,
    pub layout: TensorLayout,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    TensorDeclaration(TensorDeclaration),
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    TypeAnnotation(TypeAnnotation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TensorType {
    Scalar,
    Vector,
    Matrix,
    Tensor(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TensorDataType {
    F32,
    F64,
    I32,
    I64,
    Complex32,
    Complex64,
    Bool,
    Generic(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TensorDimension {
    Fixed(Vec<usize>),
    Dynamic(Vec<Option<usize>>),
    Named(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TensorDeclaration {
    pub name: String,
    pub shape: TensorDimension,
    pub data_type: TensorDataType,
    pub layout: TensorLayout,
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub device: Device,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: String,
    pub data_type: DataType,
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: DataType,
    pub body: Body,
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Annotation {
    Parallel,
    GPU {
        device: Option<usize>,
        memory: Option<String>,
    },
    Cache {
        strategy: CacheStrategy,
    },
    Fusion {
        group: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum CacheStrategy {
    Memory,
    Register,
    Shared,
    Global,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub parameter_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    FunctionCall(FunctionCall),
    TensorOperation(TensorOperation),
    TensorFunction(TensorFunction),
    // IndexAccess(IndexAccess),
    ArraySlice(ArraySlice),
    MemberAccess(MemberAccess),
    LambdaExpression(LambdaExpression),
    TypeCast(TypeCast),
    Conditional(Conditional),
    Statement(Box<Statement>),
    MethodCall(MethodCall),
    CompoundAssignment(CompoundAssignment),
    RangeExpression(RangeExpression),
    Array(ArrayExpression),
    ArrayRepeat(ArrayRepeatExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TensorOperation {
    Contract {
        left: Box<Expression>,
        right: Box<Expression>,
        dims: Option<Vec<usize>>,
    },
    Broadcast {
        tensor: Box<Expression>,
        shape: Vec<usize>,
    },
    Reshape {
        tensor: Box<Expression>,
        shape: Vec<Expression>,
    },
    Slice {
        tensor: Box<Expression>,
        slices: Vec<SliceSpec>,
    },
    Transpose {
        tensor: Box<Expression>,
        dims: Option<Vec<usize>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TensorFunction {
    Zeros(Vec<Expression>),
    Ones(Vec<Expression>),
    Eye(Expression),
    Random {
        shape: Vec<Expression>,
        distribution: Distribution,
    },
    LinSpace {
        start: Box<Expression>,
        end: Box<Expression>,
        num: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Distribution {
    Uniform {
        low: Option<Box<Expression>>,
        high: Option<Box<Expression>>,
    },
    Normal {
        mean: Option<Box<Expression>>,
        std: Option<Box<Expression>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SliceSpec {
    Single(Expression),
    Range {
        start: Option<Expression>,
        end: Option<Expression>,
        step: Option<Expression>,
    },
    All,
}

// ... Le reste de vos structures existantes (CompoundAssignment, IndexAccess, etc.) ...

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    ReturnStatement(ReturnStatement),
    UseStatement(UseStatement),
    ModuleImportStatement(ModuleImportStatement),
    SpecificImportStatement(SpecificImportStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    LoopStatement(LoopStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    Break,
    Continue,
    DeclarationStatement(Declaration),
    Assignment(Expression, Expression),
    // TensorAssignment {
    //     target: Box<Expression>,
    //     value: Box<Expression>,
    //     device: Option<Device>,
    // },
    TensorAssignment(TensorAssignment),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TensorAssignment{
    pub target: Box<Expression>,
    pub value: Box<Expression>,
    pub device: Option<Device>,
}



// ... Le reste de vos énumérations et structures existantes ...

// Implémentation des traits utiles
impl ASTNode {
    pub fn is_tensor_operation(&self) -> bool {
        matches!(self, ASTNode::Expression(Expression::TensorOperation(_)))
    }

    pub fn get_type_info(&self) -> Option<TensorTypeInfo> {
        match self {
            ASTNode::Declaration(Declaration::TensorDeclaration(tensor)) => {
                Some(TensorTypeInfo {
                    dtype: tensor.data_type.clone(),
                    dims: tensor.shape.clone(),
                    layout: tensor.layout.clone(),
                })
            },
            _ => None,
        }
    }

    pub fn verify_tensor_operation(&self) -> Result<(), String> {
        match self {
            ASTNode::Expression(Expression::TensorOperation(op)) => {
                match op {
                    TensorOperation::Contract { dims, .. } => {
                        // Vérification des dimensions de contraction
                        // ...
                        Ok(())
                    },
                    _ => Ok(()),
                }
            },
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TensorTypeInfo {
    pub dtype: TensorDataType,
    pub dims: TensorDimension,
    pub layout: TensorLayout,
}