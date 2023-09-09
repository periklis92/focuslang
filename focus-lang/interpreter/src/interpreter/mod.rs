mod block;
mod closure;
mod literal;
mod operation;
mod resolve_type;
mod stmt;
mod r#struct;

#[cfg(test)]
mod tests;
#[cfg(target_arch = "wasm32")]
mod wasm;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::context::Context;
use crate::r#type::Type;
use crate::r#type::TypeRegistry;
use crate::stack::ValueStack;
use crate::{object::Value, r#type::FunctionType};
use parser::{
    stmt::{Expression, Stmt},
    Parser, ParserError, ParserErrorInfo,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{context::Local, object::Function};

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

    fn resolve_path<'a, T: Iterator<Item = &'a str>>(
        &mut self,
        mut iter: T,
        value: Value,
        ty: &Type,
    ) -> Result<Value, String> {
        if let Some(s) = iter.next() {
            let fields = ty.as_struct().ok_or("Invalid path.".to_string())?;

            let (offset, field) = fields
                .iter()
                .enumerate()
                .find(|f| f.1.ident == s)
                .ok_or(format!("Invalid field name {s}"))?;

            let field_type = self
                .type_registry
                .get_type_from_id(field.type_id)
                .expect(&format!("Type with id {} not found", field.type_id));

            match value {
                Value::Object(object) => {
                    self.resolve_path(iter, object.get_value(offset).unwrap(), &field_type)
                }
                _ => Err("Expected struct, found another type.".to_string()),
            }
        } else {
            return Ok(value);
        }
    }

    fn interpret_expression(&mut self, expr: Expression) -> Result<Value, String> {
        match expr {
            Expression::Literal(literal) => self.interpret_literal(literal),
            Expression::Path(path) => {
                let mut path_parts = path.split('.');
                let root = path_parts.next().ok_or("Invalid path.".to_string())?;

                let local = self
                    .context
                    .borrow()
                    .find_local(&root)
                    .ok_or(format!("Unknown path {path}."))?;

                let value = self.stack.get_value(local.sp.unwrap()).unwrap();
                let value_type = self.type_registry.get_type_from_id(local.ty).unwrap();

                self.resolve_path(path_parts, value, &value_type)
            }
            Expression::Operation(operation) => self.interpret_operation(operation),
            Expression::Call(call) => {
                let function_name = self.context.borrow().find_local(&call.path).ok_or(format!(
                    "Unable to get value from stack for path {}.",
                    call.path
                ))?;

                let value = self
                    .stack
                    .get_value(function_name.sp.unwrap())
                    .ok_or(format!(
                        "Unable to get value from stack for path {}.",
                        call.path
                    ))?;

                let func_ty = self.type_registry.get_type_from_id(function_name.ty);

                let FunctionType {
                    arg_types,
                    ret_type,
                } = func_ty
                    .as_deref()
                    .map(|ty| ty.as_function())
                    .flatten()
                    .ok_or("Unexpected type for function".to_string())?;

                self.stack.push_frame();
                let (parent_context, expr) = match value.deref_value() {
                    Value::Function(function) => {
                        let function = function.borrow();
                        let mut ctx_mut = function.context.borrow_mut();

                        if function.args.len() != call.params.len() {
                            return Err(format!("Invalid number of arguments in function call. Expected {} while {} were passed.",
                            function.args.len(),
                            call.params.len()));
                        }

                        for (i, arg) in function.args.iter().enumerate() {
                            let expr = call.params[i].clone();
                            let expr_type_id = self.resolve_expr_type(&expr, Some(arg_types[i]))?;
                            let value = self.interpret_expression(expr)?;
                            if !self
                                .type_registry
                                .are_types_equal(arg_types[i], expr_type_id)?
                            {
                                return Err(format!("Unexpected type for argument {i}"));
                            }
                            let sp = self.stack.push_value(value);
                            ctx_mut.add_local(
                                &arg,
                                Local {
                                    ty: expr_type_id,
                                    sp: Some(sp),
                                },
                            );
                        }

                        for name in &function.captured_names {
                            let sp = self.stack.push_value(Value::Ref(name.value.clone()));
                            ctx_mut.add_local(
                                &name.ident,
                                Local {
                                    sp: Some(sp),
                                    ty: name.type_id,
                                },
                            );
                        }
                        (function.context.clone(), function.expr.clone())
                    }
                    _ => return Err("Expected a function or closure for call.".to_string()),
                };
                let inner_context =
                    Rc::new(RefCell::new(Context::new().with_parent(parent_context)));
                let previous_context = std::mem::take(&mut self.context);
                self.context = inner_context;

                let resolved_ret_type = self.resolve_expr_type(&expr, Some(*ret_type))?;
                self.type_registry
                    .are_types_equal(resolved_ret_type, *ret_type)?;

                let value = self.interpret_expression(expr)?;

                self.context = previous_context;
                self.stack.pop_frame();
                Ok(value)
            }
            Expression::Struct(r#struct) => self.interpret_struct(r#struct),
            Expression::Range(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Index(_) => todo!(),
            Expression::IfElse(_) => todo!(),
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
