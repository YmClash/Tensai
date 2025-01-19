use num_bigint::BigInt;
use std::fmt;
use std::str::pattern::Pattern;
use crate::lexer::tok::Keywords;

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum ASTNode{
    Programm(Vec<ASTNode>),
    Declaration(Declaration),
    Expression(Expression),
    Statement(Statement),
    Error(ParserError),
    Body(Body),

}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Device {
    CPU,
    GPU(usize),  // usize pour l'index du GPU
    TPU,
    Custom(String),
}


//# Et une structure pour les métadonnées des tenseurs :
#[derive(Debug, PartialEq, Clone)]
pub struct TensorMetadata {
    pub dtype: Keywords,  // Type de données (F32, F64, etc.)
    pub shape: Vec<usize>,
    pub tensor_type: TensorType,
    pub is_contiguous: bool,
    pub device: Device,  // CPU, GPU, etc.
}


#[allow(dead_code)]
#[derive(Debug, Clone, , PartialEq)]
pub struct ParserError {

    pub message: String,
    pub position: Position,

}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[allow(dead_code)]
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
    Tensor(usize),  // usize représente le rang du tenseur
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct TensorDeclaration {
    pub name: String,
    pub shape: Vec<usize>,
    pub data_type: DataType,
    pub visibility: Visibility,
    pub mutability: Mutability,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: String,
    pub data_type: DataType,
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub value: Option<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters:     Vec<Parameter>,
    pub return_type: DataType,
    pub body: Body,
    pub visibility: Visibility,
    pub mutability: Mutability,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub name: String,
    pub data_type: DataType,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter{
    pub name: String,
    pub parameter_type: DataType,

}






#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expression{
    Literal(Literal),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    FunctionCall(FunctionCall),

    ArrayAccess(ArrayAccess), // transfere dans IndexAccess
    ArraySlice(ArraySlice), // pas encore completement implementé

    MemberAccess(MemberAccess),
    LambdaExpression(LambdaExpression),
    // MatchExpression(MatchExpression),
    // MatchArm(Box<MatchArm>),
    TypeCast(TypeCast),
    Conditional(Conditional),
    // Assignment(Assignment),
    // Borrow(Borrow),
    Statement(Box<Statement>),
    MethodCall(MethodCall),
    IndexAccess(IndexAccess), // Aka ArrayAccess
    CompoundAssignment(CompoundAssignment),
    // DestructuringAssignment(DestructuringAssignment),
    RangeExpression(RangeExpression),
    Array(ArrayExpression),
    ArrayRepeat(ArrayRepeatExpression),

}
// #[allow(dead_code)]
// #[derive(Debug, Clone, PartialEq)]
// pub struct MatchArm {
//     pub pattern: Pattern,
//     pub guard: Option<Box<Expression>>,
//     //pub expression: Box<Expression>
//     pub body: Body,
// }

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundAssignment{
    pub target: Box<Expression>,
    pub operator: CompoundOperator,
    pub value: Box<Expression>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct IndexAccess{
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpression{
    pub elements: Vec<Expression>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayRepeatExpression{
    pub value: Box<Expression>,
    pub size: Box<Expression>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall{
    pub object: Box<Expression>,
    pub method: String,
    pub arguments: Vec<Expression>,
}



#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Box<Expression>,
    pub arguments: Vec<Expression>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayAccess {
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySlice{
    pub array: Box<Expression>,
    pub start: Option<Box<Expression>>,
    pub end: Option<Box<Expression>>,
    pub step: Option<Box<Expression>>,
    pub inclusive: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MemberAccess {
    pub object: Box<Expression>,
    pub member: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeCast {
    pub expression: Box<Expression>,
    pub target_type: DataType,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct YieldStatement {
    pub value: Option<Expression>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub target: Expression,
    pub value: Expression,
}

// #[allow(dead_code)]
// #[derive(Debug, Clone)]
// pub struct Function {
//     pub declaration: FunctionDeclaration,
//     pub body: Body,
// }

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaExpression {
    pub parameters: Vec<Parameter>,
    pub return_type: Option<DataType>,
    //pub body: Box<Expression>,
    pub body: Vec<ASTNode>,
    //pub body: Body,
}

// #[allow(dead_code)]
// #[derive(Debug, Clone, PartialEq)]
// pub struct MatchExpression {
//     pub expression: Box<Expression>,
//     pub arms: Vec<MatchArm>,
// }

// #[allow(dead_code)]
// #[derive(Debug, Clone)]
// pub struct RangeExpression {
//     pub start: Box<Expression>,
//     pub end: Box<Expression>,
//     pub inclusive: bool,
// }
//

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct RangeExpression {
    pub left: Option<Box<Expression>>,
    pub operator: Operator,
    pub right: Option<Box<Expression>>,
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub then_block: Box<Expression>,
    pub else_block: Box<Expression>,
}



#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement{
    Expression(Expression),
    ReturnStatement(ReturnStatement),

    UseStatement(UseStatement),

    ModuleImportStatement(ModuleImportStatement),
    SpecificImportStatement(SpecificImportStatement),

    //
    // RaiseStatement(RaiseStatement),
    // DelStatement(DelStatement),
    IfStatement(IfStatement),
    //ElifStatement(ElifStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    LoopStatement(LoopStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),

    Break,
    Continue,


    // YieldStatement(YieldStatement),

    DeclarationStatement(Declaration),
    Assignment(Expression, Expression),
    // MatchStatement(MatchStatement),

}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct UseStatement {
    pub module:String,
    pub alias: Option<String>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleImportStatement{
    pub keyword: ImportKeyword,
    //pub module_path: ModulePath,
    pub module_path: Vec<String>,
    pub alias: Option<String>,
    // pub items: Option<Vec<ImportItem>>,
    // pub relative_level: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct SpecificImportStatement{
    pub keyword: ImportKeyword,
    pub module_path: Vec<String>,
    pub alias: Option<String>,
    pub imports : Vec<(String,Option<String>)>
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum ImportKeyword{
    Use,
    Import,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ModulePath{
    //pub segments: Vec<String>,
    pub path: Vec<String>,
    pub alias: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ImportItem{
    pub name: String,
    pub alias: Option<String>,
    pub sub_path: Option<Vec<String>>
}

// #[allow(dead_code)]
// #[derive(Clone, Debug)]
// pub struct MatchStatement{
//     pub expression: Expression,
//     pub arms: Vec<MatchArm>,
// }

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    // pub value: Expression
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub elif_block: Vec<ASTNode>,
    pub else_block: Option<Vec<ASTNode>>,
}


#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Body,
    //pub body: Body,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub iterator: String,
    pub iterable: Expression,
    pub body: Body,
    //pub body: Body,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement {
    pub label: Option<String>,
    pub body: Body,
    //pub body: Body,
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement {
    pub label: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStatement {
    pub label: Option<String>,
}



#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub struct Body{
    pub statements: Vec<ASTNode>,
}

#[allow(dead_code)]
#[derive(Debug,Clone,PartialEq)]
pub enum Visibility {
    Private,     // default mode
    Public   // keyword PUB
}


#[allow(dead_code)]
#[derive(Debug, Clone,PartialEq)]
pub enum Mutability {
    Immutable, // default mode
    Mutable,   // keyword MUT
}

#[allow(dead_code)]
#[derive(Debug, Clone,PartialEq)]
pub enum Operator {
    Addition,       // +
    Substraction,   // -
    Multiplication, // *
    Division,       // /
    Modulo,     // %
    Equal,  // ==
    NotEqual,   // !=
    LessThan,   // <
    GreaterThan,   // >
    And, // &&
    Or, // ||
    LesshanOrEqual, // <=
    GreaterThanOrEqual, // >=
    Range, // ..
    RangeInclusive, // ..=
    TENSORPROD, // @
    CONTRACTDIM, // #[0-9]+>
    CONTRACT, // #>
    TRANSPOSE, // '

}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,     // -
    Not,      // !
    Increment,      // ++
    Decrement,      // --
    Reference,      // &
    ReferenceMutable,       // &mut
    Dereference,        // *
    BitwiseNot,     // ~
    LogicalNot,     // !
    Positive,       // +
    Negative,       // -
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CompoundOperator{
    AddAssign,      // +=
    SubAssign,      // -=
    MulAssign,      // *=
    DivAssign,      // /=
    ModAssign,      // %=
    BitwiseAndAssign,   // &=
    BitwiseOrAssign,    // |=
    BitwiseXorAssign,   // ^=
    LeftShiftAssign,    // <<=
    RightShiftAssign,   // >>=
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer { value: BigInt },
    Float { value: f64 },
    String(String),
    Boolean(bool),
    Array(Vec<Expression>),
    Char(char),
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    Char,
    Array(Box<DataType>),
    Tensor(Box<DataType>),
    Tuple(Vec<DataType>),
    Custom(String),
    Any,
    None,
    Infer, // Type inféré déduire par le compilateur (Type Inference)

    //Trait(String), // pour Type Bounds
    Named(String),

    SelfType,
}


