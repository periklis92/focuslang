use parser::stmt::Expression;

use crate::{
    object::{Value, ValueRef},
    r#type::PrimitiveType,
    Interpreter,
};

impl Interpreter {
    pub(super) fn interpret_expression(&mut self, expr: Expression) -> Result<Value, String> {
        match expr {
            Expression::Literal(literal) => self.interpret_literal(literal),
            Expression::Path(path) => {
                let value_ref = self.resolve_path(path)?;

                match value_ref {
                    ValueRef::StackRef { sp, .. } => Ok(self
                        .stack
                        .get_value(sp)
                        .expect("Unable to find registered local.")),
                    ValueRef::ObjectRef { object, index, .. } => Ok(object
                        .borrow()
                        .get_value(index)
                        .expect("Unable to find field in struct.")),
                }
            }
            Expression::Operation(operation) => self.interpret_operation(operation),
            Expression::Call(call) => self.interpret_call(call),
            Expression::Struct(r#struct) => self.interpret_struct(r#struct),
            Expression::Range(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::IfElse(if_else) => {
                let condition_type_id = self.resolve_expr_type(&if_else.condition, None)?;
                let condition_type = self
                    .type_registry
                    .get_type_from_id(condition_type_id)
                    .expect("Condition type not found.");

                if !condition_type.is_boolean() {
                    return Err(format!(
                        "Unexpected type for condition {}",
                        condition_type.ident
                    ));
                }

                let condition_value = self.interpret_expression(*if_else.condition)?;

                let if_type_id = self.resolve_expr_type(&if_else.if_expr, None)?;

                if let Some(else_expr) = &if_else.else_expr {
                    let else_type_id = self.resolve_expr_type(else_expr, None)?;
                    if !self
                        .type_registry
                        .are_types_equal(if_type_id, else_type_id)?
                    {
                        return Err("If/Else expression return types don't match.".to_string());
                    }
                } else if !self
                    .type_registry
                    .are_types_equal(if_type_id, PrimitiveType::Unit.type_id())?
                {
                    return Err("Invalid type for If expression. Unit was expected.".to_string());
                }

                match condition_value {
                    Value::Boolean(true) => self.interpret_expression(*if_else.if_expr),
                    Value::Boolean(false) if matches!(if_else.else_expr, Some(_)) => {
                        self.interpret_expression(*if_else.else_expr.unwrap())
                    }
                    Value::Boolean(false) => Ok(Value::Unit),
                    _ => unreachable!(),
                }
            }
            Expression::Match(_) => todo!(),
            Expression::For(_) => todo!(),
            Expression::Block(block) => self.interpret_block(block),
            Expression::Closure(closure) => self.interpret_closure(closure),
        }
    }
}
