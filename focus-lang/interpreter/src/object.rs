use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

use parser::stmt::Expression;

use crate::{context::Context, r#type::TypeId};

pub enum ValueRef {
    StackRef {
        sp: usize,
        type_id: TypeId,
    },
    ObjectRef {
        object: Rc<RefCell<Object>>,
        index: usize,
        type_id: TypeId,
    },
}

pub struct CapturedName {
    pub ident: String,
    pub value: Rc<RefCell<Value>>,
    pub type_id: TypeId,
}

pub struct Function {
    pub context: Rc<RefCell<Context>>,
    pub expr: Expression,
    pub args: Vec<String>,
    pub captured_names: Vec<CapturedName>,
}

#[derive(Clone)]
pub enum Value {
    Unit,
    Boolean(bool),
    Char(char),
    Integer(i64),
    Float(f64),
    Ref(Rc<RefCell<Value>>),
    Object(Rc<RefCell<Object>>),
    Function(Rc<RefCell<Function>>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Ref(l0), Self::Ref(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => Rc::ptr_eq(l0, r0),
            (Self::Function(l0), Self::Function(r0)) => Rc::ptr_eq(l0, r0),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Value {
    pub fn deref_value(self) -> Value {
        match self {
            Value::Unit
            | Value::Boolean(_)
            | Value::Char(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Object(_)
            | Value::Function(_) => self,
            Value::Ref(value) => value.borrow().deref().clone(),
        }
    }

    #[inline]
    pub fn is_ref(&self) -> bool {
        match self {
            Value::Ref(_) => true,
            _ => false,
        }
    }

    pub fn set_inner_value(&mut self, value: Value) {
        match self {
            Value::Ref(v) => v.borrow_mut().set_inner_value(value),
            this => *this = value,
        }
    }

    pub fn add(self, other: Value) -> Option<Value> {
        match (self.deref_value(), other.deref_value()) {
            (Value::Float(l), Value::Float(r)) => Some(Value::Float(l + r)),
            (Value::Integer(l), Value::Integer(r)) => Some(Value::Integer(l + r)),
            _ => None,
        }
    }

    pub fn sub(self, other: Value) -> Option<Value> {
        match (self.deref_value(), other.deref_value()) {
            (Value::Float(l), Value::Float(r)) => Some(Value::Float(l - r)),
            (Value::Integer(l), Value::Integer(r)) => Some(Value::Integer(l - r)),
            _ => None,
        }
    }

    pub fn mul(self, other: Value) -> Option<Value> {
        match (self.deref_value(), other.deref_value()) {
            (Value::Float(l), Value::Float(r)) => Some(Value::Float(l * r)),
            (Value::Integer(l), Value::Integer(r)) => Some(Value::Integer(l * r)),
            _ => None,
        }
    }

    pub fn div(self, other: Value) -> Option<Value> {
        match (self.deref_value(), other.deref_value()) {
            (Value::Float(l), Value::Float(r)) => Some(Value::Float(l / r)),
            (Value::Integer(l), Value::Integer(r)) => Some(Value::Integer(l / r)),
            _ => None,
        }
    }

    pub fn are_equal(&self, other: &Value) -> bool {
        match (self.clone().deref_value(), other.clone().deref_value()) {
            (Value::Unit, Value::Unit) => true,
            (Value::Boolean(v1), Value::Boolean(v2)) => v1 == v2,
            (Value::Char(v1), Value::Char(v2)) => v1 == v2,
            (Value::Integer(v1), Value::Integer(v2)) => v1 == v2,
            (Value::Float(v1), Value::Float(v2)) => v1 == v2,
            (Value::Ref(v1), Value::Ref(v2)) => v1.borrow().are_equal(&v2.borrow()),
            (Value::Object(v1), Value::Object(v2)) => v1.borrow().equals(&v2.borrow()),
            (Value::Function(v1), Value::Function(v2)) => Rc::ptr_eq(&v1, &v2),
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, other: Value) {
        match (self, other) {
            (Value::Unit, Value::Unit) => {}
            (Value::Boolean(v1), Value::Boolean(v2)) => *v1 = v2,
            (Value::Char(v1), Value::Char(v2)) => *v1 = v2,
            (Value::Integer(v1), Value::Integer(v2)) => *v1 = v2,
            (Value::Float(v1), Value::Float(v2)) => *v1 = v2,
            (Value::Ref(v1), Value::Ref(v2)) => *v1 = v2,
            (Value::Ref(v1), v2) => v1.borrow_mut().set(v2),
            (Value::Object(v1), Value::Object(v2)) => *v1 = v2,
            _ => unreachable!(),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "Unit"),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::Char(arg0) => f.debug_tuple("Char").field(arg0).finish(),
            Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::Ref(arg0) => f.debug_tuple("Ref").field(arg0).finish(),
            Self::Object(arg0) => f
                .debug_tuple("Object")
                .field(&arg0.borrow().type_id)
                .finish(),
            Self::Function(_) => f.debug_tuple("Function").finish(),
        }
    }
}

pub struct Object {
    pub values: Vec<Value>,
    pub type_id: TypeId,
}

impl Object {
    #[inline]
    pub fn get_value(&self, index: usize) -> Option<Value> {
        self.values.get(index).cloned()
    }

    #[inline]
    pub fn get_value_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.values.get_mut(index)
    }

    pub fn set_value(&mut self, index: usize, value: Value) -> Option<Value> {
        let val = self.values.get_mut(index)?;
        val.set(value.clone());
        Some(value)
    }

    pub fn copy_from_object(&mut self, object: &Object) {
        for (i, v) in self.values.iter_mut().enumerate() {
            v.set(object.get_value(i).unwrap())
        }
    }

    #[inline]
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }

    pub fn equals(&self, other: &Object) -> bool {
        self.type_id == other.type_id
            && self
                .values
                .iter()
                .zip(other.values.iter())
                .all(|(v1, v2)| v1.are_equal(v2))
    }
}
