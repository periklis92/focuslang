use parser::{
    op::ArithmeticOperator,
    stmt::{Expression, Operation, Operator},
};

use crate::{context::Local, object::Value, Interpreter};

impl Interpreter {
    pub(super) fn interpret_operation(&mut self, operation: Operation) -> Result<Value, String> {
        let mut lhs = self.interpret_expression(*operation.lhs.clone())?;
        let rhs = self.interpret_expression(*operation.rhs)?;

        match operation.op {
            Operator::Arithmetic(operator) => match operator {
                ArithmeticOperator::Add => {
                    Ok(lhs.add(rhs).ok_or("Unable to add values.".to_string())?)
                }
                ArithmeticOperator::Sub => {
                    Ok(lhs.sub(rhs).ok_or("Unable to sub values.".to_string())?)
                }
                ArithmeticOperator::Mul => todo!(),
                ArithmeticOperator::Div => todo!(),
                ArithmeticOperator::Mod => todo!(),
            },
            Operator::Comparison(_) => todo!(),
            Operator::Boolean(_) => todo!(),
            Operator::Assignment => {
                let Expression::Path(path) = *operation.lhs else {
                    return Err(format!(
                        "Invalid left hand side expression {}.",
                        operation.lhs.name()
                    ));
                };

                if lhs.is_ref() {
                    lhs.set_inner_value(rhs);
                } else {
                    let Some(Local { sp: Some(sp), .. }) = self.context.borrow().find_local(&path)
                    else {
                        return Err(format!("Unable to find local {path}."));
                    };
                    self.stack.set_value(sp, rhs);
                }

                Ok(Value::Unit)
            }
            Operator::CompoundAssignment(_) => todo!(),
        }
    }
}
