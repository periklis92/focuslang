use std::{cell::RefCell, rc::Rc};

use parser::stmt::{Expression, Item, ItemStmt, Stmt};

use crate::{
    context::Local,
    object::{Function, Value},
    r#type::{FunctionType, PrimitiveType, Type, TypeLayout},
    Interpreter,
};

impl Interpreter {
    pub fn interpret_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        let value = match stmt {
            Stmt::Item(item) => self.interpret_item(item).map(|_| Value::Unit)?,
            Stmt::Let(l) => match l.expr.map(|b| *b) {
                Some(Expression::Block(mut b))
                    if b.len() < 2
                        && l.args.is_empty()
                        && !matches!(l.ty, Some(parser::Type::Function(_))) =>
                {
                    if b.len() > 0 {
                        let stmt = b.swap_remove(0);
                        let ty = self.resolve_stmt_type(&stmt, None)?;
                        let value = self.interpret_stmt(stmt)?;
                        let sp = self.stack.push_value(value.clone());
                        self.context
                            .borrow_mut()
                            .add_local(&l.ident, Local { ty, sp: Some(sp) });
                        value
                    } else {
                        unreachable!("Empty block of statements.")
                    }
                }
                Some(Expression::Block(b)) if b.len() > 0 => {
                    let ty =
                        l.ty.ok_or("You need to declare the type of the function.".to_string())?;

                    let ty = self
                        .type_registry
                        .get_type_from_expr(&ty)
                        .ok_or("Invalid function type.".to_string())?;

                    let Type {
                        type_id,
                        layout: TypeLayout::Function(FunctionType { arg_types, .. }),
                        ..
                    } = ty.as_ref()
                    else {
                        return Err("Invalid function type.".to_string());
                    };

                    let args = if l.args.is_empty() {
                        if arg_types[0] == PrimitiveType::Unit.type_id() {
                            vec!["()".to_string()]
                        } else {
                            return Err("Invalid number of arguments in function.".to_string());
                        }
                    } else {
                        l.args
                    };

                    if args.len() != arg_types.len() {
                        return Err("Invalid number of arguments in function.".to_string());
                    }

                    let expr = Expression::Block(b);

                    let function = Rc::new(RefCell::new(Function {
                        context: self.context.clone(),
                        expr,
                        args,
                        captured_names: Vec::new(),
                    }));
                    let sp = self.stack.push_value(Value::Function(function.clone()));
                    self.context.borrow_mut().add_local(
                        &l.ident,
                        Local {
                            ty: *type_id,
                            sp: Some(sp),
                        },
                    );
                    Value::Function(function)
                }
                Some(_) => return Err("Invalid let declaration.".to_string()),
                None => unimplemented!("Uninitialized local"),
            },
            Stmt::Expr(expr) => self.interpret_expression(expr)?,
        };
        Ok(value)
    }

    fn interpret_item(&mut self, item: ItemStmt) -> Result<(), String> {
        match item.item {
            Item::Alias(alias) => self
                .type_registry
                .insert_alias_type_from_item(alias, item.visibility),
            Item::Struct(struc) => self
                .type_registry
                .insert_struct_type_from_item(struc, item.visibility),
            Item::ModuleDeclaration(_) => todo!(),
            Item::UseDeclaration(_) => todo!(),
        };
        Ok(())
    }
}