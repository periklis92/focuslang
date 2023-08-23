#[derive(Debug, PartialEq)]
pub enum Type {
    Unit,
    Name(String),
    Function(FunctionType),
}

#[derive(Debug, PartialEq)]
pub struct FunctionType {
    pub args: Vec<String>,
    pub ret: String,
}
