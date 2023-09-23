use std::{cell::RefCell, rc::Rc};

use parser::stmt::Expression;

use crate::{
    object::{Object, Value, ValueRef},
    r#type::{PrimitiveType, Type, TypeLayout},
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
            Expression::Array(arr) => {
                let type_id = self.resolve_expr_type(&arr[0], None)?;
                let arr_type_id = self
                    .type_registry
                    .insert_or_get_array_type_for_type(type_id)
                    .unwrap();
                let values = arr
                    .into_iter()
                    .map(|el| self.interpret_expression(el))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::Object(Rc::new(RefCell::new(Object {
                    values,
                    type_id: arr_type_id,
                }))))
            }
            Expression::Index(index) => {
                let value = self.interpret_expression(*index.index)?;
                let i = match value {
                    Value::Integer(i) => i,
                    _ => return Err("Unexpected value for indexing operation.".to_string()),
                };
                let value = self.interpret_expression(*index.value)?;
                match value {
                    Value::Object(object) => {
                        let ty = self
                            .type_registry
                            .get_type_from_id(object.borrow().type_id)
                            .unwrap();

                        let Type {
                            layout: TypeLayout::Array(_),
                            ..
                        } = *ty
                        else {
                            return Err("You can only index arrays currently.".to_string());
                        };

                        object
                            .borrow()
                            .get_value(i as usize)
                            .ok_or("Index out of range.".to_string())
                    }
                    _ => Err("Value cannot be indexed.".to_string()),
                }
            }
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
