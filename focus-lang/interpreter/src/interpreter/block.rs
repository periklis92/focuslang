use std::{cell::RefCell, rc::Rc};

use parser::stmt::Stmt;

use crate::{
    object::{CapturedName, Value},
    Interpreter,
};

impl Interpreter {
    pub(super) fn interpret_block(&mut self, block: Vec<Stmt>) -> Result<Value, String> {
        let ret = block
            .into_iter()
            .map(|stmt| self.interpret_stmt(stmt))
            .last()
            .expect("No statements in block")?;

        // If the function escapes the current block it
        // captures all the local variables it uses and they escape to the heap.
        match ret {
            Value::Function(ref function) => {
                let names = self.find_names_to_capture(&function.borrow());
                let values = names
                    .into_iter()
                    .map(|n| {
                        let local = self
                            .context
                            .borrow()
                            .get_local(&n)
                            .ok_or(format!("Unknown local {n}"))?;
                        Ok(CapturedName {
                            ident: n.clone(),
                            value: Rc::new(RefCell::new(
                                self.stack
                                    .get_value(local.sp.expect("Uninitialized local."))
                                    .ok_or(format!(
                                        "Unable to get value from stack for local {n}"
                                    ))?,
                            )),
                            type_id: local.ty,
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                function.borrow_mut().captured_names = values;
            }
            _ => {}
        }
        Ok(ret)
    }
}
