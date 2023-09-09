#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BooleanOperator {
    Or,
    And,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompoundAssignmentOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
