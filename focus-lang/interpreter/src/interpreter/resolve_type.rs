use parser::stmt::{Expression, Literal, Stmt};

use crate::{
    object::ValueRef,
    r#type::{FunctionType, PrimitiveType, Type, TypeId, TypeLayout},
    Interpreter,
};

impl Interpreter {
    pub(super) fn resolve_stmt_type(
        &mut self,
        stmt: &Stmt,
        expected_type: Option<TypeId>,
    ) -> Result<TypeId, String> {
        match stmt {
            Stmt::Item(_) => Err("An item declaration cannot have a type.".to_string()),
            Stmt::Let(_) => {
                return Ok(PrimitiveType::Unit.type_id());
            }
            Stmt::Expr(expr) => self.resolve_expr_type(expr, expected_type),
        }
    }

    pub(super) fn resolve_expr_type(
        &mut self,
        expr: &Expression,
        expected_type: Option<TypeId>,
    ) -> Result<TypeId, String> {
        let type_id = match expr {
            Expression::Literal(literal) => self.resolve_literal_type(literal)?,
            Expression::Path(path) => {
                let value_ref = self.resolve_path(path.clone())?;

                match value_ref {
                    ValueRef::StackRef { type_id, .. } => type_id,
                    ValueRef::ObjectRef { type_id, .. } => type_id,
                }
            }
            Expression::Operation(operation) => {
                let lhs_type_id = self.resolve_expr_type(&operation.lhs, expected_type)?;
                let rhs_type_id = self.resolve_expr_type(&operation.rhs, expected_type)?;
                if !self
                    .type_registry
                    .are_types_equal(lhs_type_id, rhs_type_id)?
                {
                    return Err("Not matching types in operation.".to_string());
                }
                match operation.op {
                    parser::stmt::Operator::Arithmetic(_) => lhs_type_id,
                    parser::stmt::Operator::Comparison(_) => {
                        if !self
                            .type_registry
                            .are_types_equal(lhs_type_id, PrimitiveType::Boolean.type_id())?
                        {
                            return Err("Boolean type expected.".to_string());
                        } else {
                            PrimitiveType::Boolean.type_id()
                        }
                    }
                    parser::stmt::Operator::Boolean(_) => {
                        if !self
                            .type_registry
                            .are_types_equal(lhs_type_id, PrimitiveType::Boolean.type_id())?
                        {
                            return Err("Boolean type expected.".to_string());
                        } else {
                            PrimitiveType::Boolean.type_id()
                        }
                    }
                    parser::stmt::Operator::Assignment => PrimitiveType::Unit.type_id(),
                    parser::stmt::Operator::CompoundAssignment(_) => lhs_type_id,
                }
            }
            Expression::Call(call) => {
                let type_id = self
                    .context
                    .borrow()
                    .find_local(&call.path)
                    .map(|local| local.ty)
                    .ok_or(format!("Unable to find name {}.", call.path))?;

                let ty = self
                    .type_registry
                    .get_type_from_id(type_id)
                    .ok_or(format!("Type with id {type_id} not found."))?;

                let FunctionType { ret_type, .. } = ty
                    .as_function()
                    .ok_or("Expected function type.".to_string())?;

                *ret_type
            }
            Expression::Struct(struc) => self
                .type_registry
                .get_struct_type_id_from_name(&struc.path)
                .ok_or(format!("Unknown type {} or not a struct.", struc.path))?,
            Expression::Range(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::IfElse(if_else) => {
                let if_type_id = self.resolve_expr_type(&if_else.if_expr, expected_type)?;

                if let Some(else_expr) = &if_else.else_expr {
                    let else_type_id = self.resolve_expr_type(else_expr, expected_type)?;
                    if !self
                        .type_registry
                        .are_types_equal(if_type_id, else_type_id)?
                    {
                        return Err("If/Else types don't match.".to_string());
                    } else {
                        if_type_id
                    }
                } else {
                    if self
                        .type_registry
                        .are_types_equal(if_type_id, PrimitiveType::Unit.type_id())?
                    {
                        PrimitiveType::Unit.type_id()
                    } else {
                        return Err("Expected unit type.".to_string());
                    }
                }
            }
            Expression::Match(_) => todo!(),
            Expression::For(_) => todo!(),
            Expression::Block(block) => {
                let last = block.last().expect("Block is empty.");
                self.resolve_stmt_type(last, expected_type)?
            }
            Expression::Closure(_) => {
                let expected_type = self.type_registry.get_type_from_id(expected_type.ok_or(
                    "Unable to evaluate closure's type with no type provided.".to_string(),
                )?);

                let Some(Type {
                    type_id,
                    layout: TypeLayout::Function(FunctionType { .. }),
                    ..
                }) = expected_type.as_deref()
                else {
                    return Err(format!(
                        "Expected a function type but found {}.",
                        expected_type.map_or("None".to_string(), |t| t.ident.clone())
                    ));
                };

                *type_id
            }
        };

        if expected_type.is_none()
            || self
                .type_registry
                .are_types_equal(type_id, expected_type.unwrap())?
        {
            Ok(type_id)
        } else {
            Err(format!("Mismatched types."))
        }
    }

    pub(super) fn resolve_literal_type(&self, literal: &Literal) -> Result<TypeId, String> {
        match literal {
            Literal::Unit => self
                .type_registry
                .get_type_id_from_name(PrimitiveType::Unit.name())
                .ok_or("Unable to find unit type.".to_string()),
            Literal::Boolean(_) => self
                .type_registry
                .get_type_id_from_name(PrimitiveType::Boolean.name())
                .ok_or("Unable to find bool type.".to_string()),
            Literal::Char(_) => self
                .type_registry
                .get_type_id_from_name(&PrimitiveType::Char.name())
                .ok_or("Unable to find char type.".to_string()),
            Literal::Integer(_) => self
                .type_registry
                .get_type_id_from_name(&PrimitiveType::Integer.name())
                .ok_or("Unable to find integer type.".to_string()),
            Literal::Float(_) => self
                .type_registry
                .get_type_id_from_name(&PrimitiveType::Float.name())
                .ok_or("Unable to find float type.".to_string()),
            Literal::String(_) => todo!(),
        }
    }
}
