#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Name(String),
    Array(Box<Type>),
    Function(FunctionType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub args: Vec<Type>,
    pub ret: Box<Type>,
}
