mod block;
mod closure;
mod literal;
mod operation;
mod resolve_type;
mod stmt;
mod r#struct;

mod call;
#[cfg(test)]
mod tests;
#[cfg(target_arch = "wasm32")]
mod wasm;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::context::Context;
use crate::object::Value;
use crate::object::ValueRef;
use crate::r#type::TypeRegistry;
use crate::stack::ValueStack;
use parser::{
    stmt::{Expression, Stmt},
    Parser, ParserError, ParserErrorInfo,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::object::Function;

pub enum InterpreterError {
    ParserError(ParserError),
}

impl From<ParserError> for InterpreterError {
    fn from(value: ParserError) -> Self {
        InterpreterError::ParserError(value)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Interpreter {
    stack: ValueStack,
    context: Rc<RefCell<Context>>,
    type_registry: TypeRegistry,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Default::default(),
            context: Default::default(),
            type_registry: TypeRegistry::new(),
        }
    }
}

impl Interpreter {
    pub fn interpret_str(&mut self, code: &str) -> Result<Value, String> {
        let mut parser = Parser::new(code, None);
        let mut stmt;
        let mut value = Ok(Value::Unit);
        loop {
            stmt = parser.parse_error_details();
            match stmt {
                Ok(stmt) => value = self.interpret_stmt(stmt.stmt),
                Err(ParserError {
                    info: ParserErrorInfo::Eof,
                    ..
                }) => break,
                Err(e) => return Err(e.to_string()),
            }
        }
        value
    }

    fn resolve_path(&mut self, path: String) -> Result<ValueRef, String> {
        let mut path_parts = path.split('.');
        let root = path_parts.next().ok_or("Invalid path.".to_string())?;

        let local = self
            .context
            .borrow()
            .find_local(&root)
            .ok_or(format!("Unknown path {path}."))?;

        let mut value = self.stack.get_value(local.sp.unwrap()).unwrap();
        let mut value_type = self.type_registry.get_type_from_id(local.ty).unwrap();
        let mut value_ref = ValueRef::StackRef {
            sp: local.sp.unwrap(),
            type_id: local.ty,
        };

        for p in path_parts {
            let fields = value_type.as_struct().ok_or("Invalid path.".to_string())?;

            let (offset, field) = fields
                .iter()
                .enumerate()
                .find(|f| f.1.ident == p)
                .ok_or(format!("Invalid field name {p}"))?;

            let field_type = self
                .type_registry
                .get_type_from_id(field.type_id)
                .expect(&format!("Type with id {} not found", field.type_id));

            match value {
                Value::Object(object) => {
                    value = object.borrow().get_value(offset).unwrap();
                    value_type = field_type;
                    value_ref = ValueRef::ObjectRef {
                        object: object.clone(),
                        index: offset,
                        type_id: value_type.type_id,
                    }
                }
                _ => return Err("Expected struct, found another type.".to_string()),
            }
        }

        Ok(value_ref)
    }

    fn interpret_expression(&mut self, expr: Expression) -> Result<Value, String> {
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
                }

                match condition_value {
                    Value::Boolean(true) => self.interpret_expression(*if_else.if_expr),
                    Value::Boolean(false) if matches!(if_else.else_expr, Some(_)) => {
                        self.interpret_expression(*if_else.else_expr.unwrap())
                    }
                    _ => unreachable!(),
                }
            }
            Expression::Match(_) => todo!(),
            Expression::For(_) => todo!(),
            Expression::Block(block) => self.interpret_block(block),
            Expression::Closure(closure) => self.interpret_closure(closure),
        }
    }

    fn find_names_to_capture(&mut self, closure: &Function) -> Vec<String> {
        let mut names_referenced = Vec::new();
        let mut names_defined = closure.args.iter().cloned().collect::<HashSet<_>>();
        self.get_referenced_names_in_expr(&closure.expr, &mut names_referenced, &mut names_defined);
        names_referenced
    }

    fn get_referenced_names_in_stmt(
        &self,
        stmt: &Stmt,
        names: &mut Vec<String>,
        defined: &mut HashSet<String>,
    ) {
        match stmt {
            Stmt::Item(_) => {}
            Stmt::Let(l) => {
                defined.insert(l.ident.clone());
                l.expr
                    .as_ref()
                    .map(|v| self.get_referenced_names_in_expr(&v, names, defined));
            }
            Stmt::Expr(expr) => self.get_referenced_names_in_expr(expr, names, defined),
        }
    }

    fn get_referenced_names_in_expr(
        &self,
        expr: &Expression,
        names: &mut Vec<String>,
        defined: &mut HashSet<String>,
    ) {
        match expr {
            Expression::Literal(_) => {}
            Expression::Path(path) => {
                if !defined.contains(path) && self.context.borrow().is_local(path) {
                    names.push(path.clone())
                }
            }
            Expression::Operation(operation) => {
                self.get_referenced_names_in_expr(&operation.lhs, names, defined);
                self.get_referenced_names_in_expr(&operation.rhs, names, defined);
            }
            Expression::Call(_) => {}
            Expression::Struct(_) => {}
            Expression::Range(range) => {
                range
                    .from
                    .as_ref()
                    .map(|v| self.get_referenced_names_in_expr(&v, names, defined));
                range
                    .to
                    .as_ref()
                    .map(|v| self.get_referenced_names_in_expr(&v, names, defined));
            }
            Expression::Array(array) => {
                for v in array {
                    self.get_referenced_names_in_expr(&v, names, defined);
                }
            }
            Expression::Index(index) => {
                self.get_referenced_names_in_expr(&index, names, defined);
            }
            Expression::IfElse(if_else) => {
                self.get_referenced_names_in_expr(&if_else.condition, names, defined);
                self.get_referenced_names_in_expr(&if_else.if_expr, names, defined);
                if_else
                    .else_expr
                    .as_ref()
                    .map(|v| self.get_referenced_names_in_expr(&v, names, defined));
            }
            Expression::Match(m) => {
                self.get_referenced_names_in_expr(&m.match_expr, names, defined);
                for m in &m.branches {
                    self.get_referenced_names_in_expr(&m.expr, names, defined);
                    m.if_expr
                        .as_ref()
                        .map(|v| self.get_referenced_names_in_expr(&v, names, defined));
                    self.get_referenced_names_in_expr(&m.match_expr, names, defined);
                }
            }
            Expression::For(f) => {
                self.get_referenced_names_in_expr(&f.in_expr, names, defined);
                self.get_referenced_names_in_expr(&f.block, names, defined);
            }
            Expression::Block(block) => {
                for stmt in block {
                    self.get_referenced_names_in_stmt(stmt, names, defined);
                }
            }
            Expression::Closure(_) => {}
        }
    }
}
