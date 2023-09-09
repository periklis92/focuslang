use std::{cell::RefCell, rc::Rc};

use parser::stmt::Closure;

use crate::{
    object::{Function, Value},
    Interpreter,
};

impl Interpreter {
    pub(super) fn interpret_closure(&mut self, closure: Closure) -> Result<Value, String> {
        let args = if closure.args.is_empty() {
            vec!["()".to_string()]
        } else {
            closure.args
        };

        Ok(Value::Function(Rc::new(RefCell::new(Function {
            context: self.context.clone(),
            expr: *closure.block,
            captured_names: Vec::new(),
            args,
        }))))
    }
}
