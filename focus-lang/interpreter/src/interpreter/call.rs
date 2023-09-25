use std::{cell::RefCell, rc::Rc};

use parser::stmt::Call;

use crate::{
    context::{Context, Local},
    object::Value,
    r#type::FunctionType,
    Interpreter,
};

impl Interpreter {
    pub(super) fn interpret_call(&mut self, call: Call) -> Result<Value, String> {
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
        let parent_module = parent_context.borrow().module();
        let inner_context = Rc::new(RefCell::new(
            Context::new(parent_module).with_parent(parent_context),
        ));
        let previous_context = std::mem::replace(&mut self.context, inner_context);

        let resolved_ret_type = self.resolve_expr_type(&expr, Some(*ret_type))?;
        self.type_registry
            .are_types_equal(resolved_ret_type, *ret_type)?;

        let value = self.interpret_expression(expr)?;

        self.context = previous_context;
        self.stack.pop_frame();
        Ok(value)
    }
}
