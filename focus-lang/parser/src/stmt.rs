use std::fmt::Debug;

use crate::{
    op::{ArithmeticOperator, BooleanOperator, ComparisonOperator, CompoundAssignmentOperator},
    Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StmtDetails {
    pub file: Option<String>,
    pub defined_at: std::ops::Range<usize>,
    pub stmt: Stmt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Private,
    Module,
    Public,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Item(ItemStmt),
    Let(LetStmt),
    Expr(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemStmt {
    pub item: Item,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Alias(AliasItem),
    Struct(StructItem),
    ModuleDeclaration(String),
    UseDeclaration(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AliasItem {
    pub ident: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructItem {
    pub ident: String,
    pub fields: Vec<StructItemField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructItemField {
    pub ident: String,
    pub visibility: Visibility,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt {
    pub ident: String,
    pub visibility: Option<Visibility>,
    pub args: Vec<String>,
    pub expr: Option<Box<Expression>>,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Path(String),
    Operation(Operation),
    Call(Call),
    Struct(Struct),
    Range(Range),
    Array(Vec<Expression>),
    Index(Box<Expression>),
    IfElse(IfElse),
    Match(Match),
    For(For),
    Block(Vec<Stmt>),
    Closure(Closure),
}

impl Expression {
    pub fn name(&self) -> &str {
        match self {
            Expression::Literal(_) => "<Literal>",
            Expression::Path(_) => "<Path>",
            Expression::Operation(_) => "<Operation>",
            Expression::Call(_) => "<Call>",
            Expression::Struct(_) => "<Struct>",
            Expression::Range(_) => "<Range>",
            Expression::Array(_) => "<Array>",
            Expression::Index(_) => "<Index>",
            Expression::IfElse(_) => "<IfElse>",
            Expression::Match(_) => "<Match>",
            Expression::For(_) => "<For>",
            Expression::Block(_) => "<Block>",
            Expression::Closure(_) => "<Closure>",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Unit,
    Boolean(bool),
    Char(char),
    Integer(i64),
    Float(f64),
    String(Box<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub op: Operator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Comparison(ComparisonOperator),
    Boolean(BooleanOperator),
    Assignment,
    CompoundAssignment(CompoundAssignmentOperator),
}

impl Operator {
    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Assignment => 10,
            Operator::Comparison(_) => 20,
            Operator::Boolean(_) => 20,
            Operator::CompoundAssignment(_) => 20,
            Operator::Arithmetic(ArithmeticOperator::Add | ArithmeticOperator::Sub) => 30,
            Operator::Arithmetic(
                ArithmeticOperator::Mul | ArithmeticOperator::Div | ArithmeticOperator::Mod,
            ) => 40,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub path: String,
    pub params: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub path: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub ident: String,
    pub expr: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub from: Option<Box<Expression>>,
    pub to: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElse {
    pub condition: Box<Expression>,
    pub if_expr: Box<Expression>,
    pub else_expr: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub match_expr: Box<Expression>,
    pub branches: Vec<MatchBranch>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchBranch {
    pub match_expr: Box<Expression>,
    pub if_expr: Option<Box<Expression>>,
    pub expr: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub ident: String,
    pub in_expr: Box<Expression>,
    pub block: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub args: Vec<String>,
    pub block: Box<Expression>,
}
