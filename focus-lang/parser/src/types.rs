#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Name(String),
    Function(FunctionType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub args: Vec<Type>,
    pub ret: Box<Type>,
}
