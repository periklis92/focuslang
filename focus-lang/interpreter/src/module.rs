use std::{collections::HashMap, rc::Rc};

use crate::{object::Value, r#type::TypeId};

pub enum ModuleDefinition {
    Value(Rc<Value>),
    Type(TypeId),
    Module(Rc<Module>),
}

pub struct Module {
    name: String,
    names_defined: HashMap<String, Rc<Value>>,
    types_defined: HashMap<String, TypeId>,
    modules_defined: HashMap<String, Rc<Module>>,
    parent_module: Option<Rc<Module>>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            names_defined: Default::default(),
            types_defined: Default::default(),
            modules_defined: Default::default(),
            parent_module: None,
        }
    }

    pub fn new_with_parent(name: &str, parent: Rc<Module>) -> Self {
        Self {
            name: name.to_string(),
            names_defined: Default::default(),
            types_defined: Default::default(),
            modules_defined: Default::default(),
            parent_module: Some(parent),
        }
    }

    pub fn insert_name(&mut self, name: &str, value: Value) {
        self.names_defined.insert(name.to_string(), Rc::new(value));
    }

    pub fn insert_type(&mut self, name: &str, type_id: TypeId) {
        self.types_defined.insert(name.to_string(), type_id);
    }

    pub fn insert_module(&mut self, name: &str, module: Module) {
        self.modules_defined
            .insert(name.to_string(), Rc::new(module));
    }

    pub fn get_definition_from_partial_path<I: AsRef<str>, T: Iterator<Item = I>>(
        &self,
        path: &mut T,
    ) -> Option<ModuleDefinition> {
        if let Some(p) = path.next() {
            if let Some(module) = self.modules_defined.get(p.as_ref()) {
                return Some(ModuleDefinition::Module(module.clone()));
            } else if let Some(value) = self.names_defined.get(p.as_ref()) {
                return Some(ModuleDefinition::Value(value.clone()));
            } else if let Some(type_id) = self.types_defined.get(p.as_ref()) {
                return Some(ModuleDefinition::Type(*type_id));
            } else {
                return None;
            }
        } else {
            None
        }
    }

    pub fn get_fully_qualified_name(&self, name: &str) -> String {
        let mut buf = self.get_fully_qualified_name_internal(String::new());
        buf.push_str(name);
        buf
    }

    fn get_fully_qualified_name_internal(&self, mut buf: String) -> String {
        if let Some(parent) = &self.parent_module {
            buf = parent.get_fully_qualified_name_internal(buf);
        }
        buf.push_str(&self.name);
        buf.push('.');
        buf
    }
}
