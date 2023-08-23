use scanner::Token;

#[derive(Debug, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, PartialEq)]
pub enum BooleanOperator {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
pub enum CompoundAssignmentOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Lt,
    Set,

    BinAnd,
    BinOr,
    BinXor,

    And,
    Or,
    Eq,
    NotEq,
    GreaterEq,
    LesserEq,
    Pow,
    Mod,
}

impl BinOp {
    pub fn precedence(&self) -> i32 {
        match self {
            BinOp::And
            | BinOp::Or
            | BinOp::Eq
            | BinOp::NotEq
            | BinOp::GreaterEq
            | BinOp::LesserEq => 10,
            BinOp::Add | BinOp::Sub => 20,
            BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::Pow => 30,
            BinOp::Set => 40,
            BinOp::Gt | BinOp::Lt => 50,
            BinOp::BinAnd | BinOp::BinOr | BinOp::BinXor => 60,
        }
    }
}

impl TryFrom<Token> for BinOp {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Pipe => Ok(BinOp::BinOr),
            Token::Ampersand => Ok(BinOp::BinAnd),
            Token::Hat => Ok(BinOp::BinXor),
            Token::Plus => Ok(BinOp::Add),
            Token::Dash => Ok(BinOp::Sub),
            Token::Star => Ok(BinOp::Mul),
            Token::Slash => Ok(BinOp::Div),
            Token::Greater => Ok(BinOp::Gt),
            Token::Less => Ok(BinOp::Lt),
            Token::Assign => Ok(BinOp::Set),
            Token::Or => Ok(BinOp::Or),
            Token::And => Ok(BinOp::And),
            Token::NotEqual => Ok(BinOp::NotEq),
            Token::Equal => Ok(BinOp::Eq),
            Token::GreaterEqual => Ok(BinOp::GreaterEq),
            Token::LessEqual => Ok(BinOp::LesserEq),
            _ => Err(()),
        }
    }
}

pub fn token_to_bin_op(token: &Token) -> Option<BinOp> {
    match token {
        Token::Pipe => Some(BinOp::BinOr),
        Token::Ampersand => Some(BinOp::BinAnd),
        Token::Hat => Some(BinOp::BinXor),
        Token::Plus => Some(BinOp::Add),
        Token::Dash => Some(BinOp::Sub),
        Token::Star => Some(BinOp::Mul),
        Token::Slash => Some(BinOp::Div),
        Token::Greater => Some(BinOp::Gt),
        Token::Less => Some(BinOp::Lt),
        Token::Assign => Some(BinOp::Set),
        Token::Percent => Some(BinOp::Mod),
        Token::Or => Some(BinOp::Or),
        Token::And => Some(BinOp::And),
        Token::NotEqual => Some(BinOp::NotEq),
        Token::Equal => Some(BinOp::Eq),
        Token::GreaterEqual => Some(BinOp::GreaterEq),
        Token::LessEqual => Some(BinOp::LesserEq),
        _ => None,
    }
}
