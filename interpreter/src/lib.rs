mod stack;
mod r#type;

use std::collections::HashMap;

use stack::ValueStack;

pub struct Interpreter {
    stack: ValueStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Default::default(),
        }
    }
}

pub struct Context {
    locals: HashMap<String, Local>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            locals: HashMap::default(),
        }
    }

    pub fn add_local(&mut self, ident: &str, local: Local) {
        self.locals.insert(ident.to_string(), local);
    }

    pub fn get_local(&self, ident: &str) -> Option<&Local> {
        self.locals.get(ident)
    }
}

pub struct Local {
    pub ty: usize,
    pub sp: Option<usize>,
}
