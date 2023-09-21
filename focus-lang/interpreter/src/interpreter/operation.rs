use parser::{
    op::{ArithmeticOperator, BooleanOperator, ComparisonOperator},
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
            Operator::Comparison(comparison) => match comparison {
                ComparisonOperator::Equal => Ok(Value::Boolean(lhs.are_equal(&rhs))),
                ComparisonOperator::NotEqual => Ok(Value::Boolean(!lhs.are_equal(&rhs))),
                ComparisonOperator::Greater => todo!(),
                ComparisonOperator::Less => todo!(),
                ComparisonOperator::GreaterEqual => todo!(),
                ComparisonOperator::LessEqual => todo!(),
            },
            Operator::Boolean(boolean) => match boolean {
                BooleanOperator::Or => match (lhs, rhs) {
                    (Value::Boolean(false), Value::Boolean(false)) => Ok(Value::Boolean(false)),
                    _ => Ok(Value::Boolean(true)),
                },
                BooleanOperator::And => match (lhs, rhs) {
                    (Value::Boolean(true), Value::Boolean(true)) => Ok(Value::Boolean(true)),
                    _ => Ok(Value::Boolean(false)),
                },
            },
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
