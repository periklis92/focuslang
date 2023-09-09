use parser::stmt::Literal;

use crate::{object::Value, Interpreter};

impl Interpreter {
    pub(super) fn interpret_literal(&mut self, literal: Literal) -> Result<Value, String> {
        match literal {
            Literal::Unit => Ok(Value::Unit),
            Literal::Boolean(bool) => Ok(Value::Boolean(bool)),
            Literal::Char(char) => Ok(Value::Char(char)),
            Literal::Integer(integer) => Ok(Value::Integer(integer)),
            Literal::Float(float) => Ok(Value::Float(float)),
            Literal::String(_) => todo!(),
        }
    }
}
