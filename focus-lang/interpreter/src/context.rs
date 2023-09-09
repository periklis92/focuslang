use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::r#type::TypeId;

#[derive(Clone, Debug)]
pub struct Local {
    pub ty: TypeId,
    pub sp: Option<usize>,
}

#[derive(Default, Clone, Debug)]
pub struct Context {
    locals: HashMap<String, Local>,
    parent: Option<Rc<RefCell<Context>>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            locals: HashMap::default(),
            parent: None,
        }
    }

    pub fn with_parent(mut self, parent: Rc<RefCell<Context>>) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn take_parent(&mut self) -> Option<Rc<RefCell<Context>>> {
        std::mem::take(&mut self.parent)
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Context>>> {
        self.parent.clone()
    }

    pub fn add_local(&mut self, ident: &str, local: Local) {
        self.locals.insert(ident.to_string(), local);
    }

    pub fn get_local(&self, ident: &str) -> Option<Local> {
        self.locals.get(ident).cloned()
    }

    pub fn find_local(&self, ident: &str) -> Option<Local> {
        self.locals.get(ident).cloned().or_else(|| {
            self.parent
                .as_ref()
                .map(|p| p.borrow().get_local(ident))
                .flatten()
        })
    }

    pub fn is_in_parent(&self, ident: &str) -> bool {
        self.parent
            .as_ref()
            .is_some_and(|p| p.borrow().is_local(ident))
    }

    #[inline]
    pub fn is_local(&self, ident: &str) -> bool {
        self.get_local(ident).is_some()
    }

    pub fn capture(&self) -> Self {
        self.clone()
    }
}
