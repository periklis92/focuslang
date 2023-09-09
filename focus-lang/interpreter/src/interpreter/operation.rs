use parser::{
    op::ArithmeticOperator,
    stmt::{Expression, Operation, Operator},
};

use crate::{
    object::{Value, ValueRef},
    Interpreter,
};

impl Interpreter {
    pub(super) fn interpret_operation(&mut self, operation: Operation) -> Result<Value, String> {
        let t1 = self.resolve_expr_type(&operation.lhs, None)?;
        let lhs = self.interpret_expression(*operation.lhs.clone())?;
        let t2 = self.resolve_expr_type(&operation.rhs, None)?;
        let rhs = self.interpret_expression(*operation.rhs)?;

        if !self.type_registry.are_types_equal(t1, t2)? {
            return Err("Invalid type in operation".to_string());
        }

        match operation.op {
            Operator::Arithmetic(operator) => match operator {
                ArithmeticOperator::Add => {
                    Ok(lhs.add(rhs).ok_or("Unable to add values.".to_string())?)
                }
                ArithmeticOperator::Sub => Ok(lhs
                    .sub(rhs)
                    .ok_or("Unable to subtract values.".to_string())?),
                ArithmeticOperator::Mul => Ok(lhs
                    .mul(rhs)
                    .ok_or("Unable to multiply values.".to_string())?),
                ArithmeticOperator::Div => {
                    Ok(lhs.div(rhs).ok_or("Unable to divide values.".to_string())?)
                }
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

                let value_ref = self.resolve_path(path)?;

                match value_ref {
                    ValueRef::StackRef { sp, .. } => {
                        self.stack.set_value(sp, rhs);
                    }
                    ValueRef::ObjectRef { object, index, .. } => {
                        object.borrow_mut().set_value(index, rhs);
                    }
                }

                Ok(Value::Unit)
            }
            Operator::CompoundAssignment(_) => todo!(),
        }
    }
}
