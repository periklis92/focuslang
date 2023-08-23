use std::{collections::HashMap, rc::Rc};

use parser::stmt::{AliasItem, StructItem, Visibility};

pub type TypeId = usize;

pub struct TypeRegistry {
    pub type_ids: HashMap<String, TypeId>,
    pub types: Vec<Rc<Type>>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        let mut tr = TypeRegistry {
            type_ids: Default::default(),
            types: Default::default(),
        };
        tr.insert_primitives();
        tr
    }

    pub fn get_type_from_name(&mut self, name: &str) -> Option<Rc<Type>> {
        let type_id = self.type_ids.get(name)?;
        self.get_type_from_id(*type_id)
    }

    pub fn get_type_from_id(&mut self, type_id: TypeId) -> Option<Rc<Type>> {
        self.types.get(type_id).cloned()
    }

    pub fn get_type_from_expr(&mut self, ty: parser::Type) -> Option<Rc<Type>> {
        match ty {
            parser::Type::Unit => self.get_type_from_id(0),
            parser::Type::Name(name) => self.get_type_from_name(&name),
            parser::Type::Function(_) => todo!(),
        }
    }

    pub fn resolve_type(&mut self, ty: &Type) -> Option<Rc<Type>> {
        match &ty.layout {
            TypeLayout::Object(type_id) => self
                .get_type_from_id(*type_id)
                .map(|t| self.resolve_type(&t))
                .flatten(),
            TypeLayout::Alias(type_id) => self
                .get_type_from_id(*type_id)
                .map(|t| self.resolve_type(&t))
                .flatten(),
            _ => self.get_type_from_name(&ty.ident),
        }
    }

    pub fn insert_alias_type_from_item(
        &mut self,
        alias: AliasItem,
        visibility: Visibility,
    ) -> Option<TypeId> {
        let ty = self.get_type_from_name(&alias.path.unwrap_or("unit".to_string()))?;
        let type_id = self.types.len();
        self.types.push(
            Type {
                ident: alias.ident.clone(),
                type_id,
                size: ty.size,
                layout: TypeLayout::Alias(ty.type_id),
                visibility,
            }
            .into(),
        );
        self.type_ids.insert(alias.ident, type_id);
        Some(type_id)
    }

    pub fn insert_struct_type_from_item(
        &mut self,
        struc: StructItem,
        visibility: Visibility,
    ) -> Option<TypeId> {
        let mut fields = Vec::new();
        let mut offset = 0;
        for f in struc.fields {
            let ty = self
                .get_type_from_expr(f.ty)
                .map(|t| self.resolve_type(&t))
                .flatten()?;
            fields.push(StructField {
                ident: f.ident.clone(),
                offset,
                visibility: f.visibility,
                type_id: ty.type_id,
            });
            offset += ty.size;
        }
        let type_id = self.types.len();
        self.types.push(
            Type {
                ident: struc.ident.clone(),
                type_id,
                size: offset,
                layout: TypeLayout::Struct(fields),
                visibility,
            }
            .into(),
        );
        self.type_ids.insert(struc.ident, type_id);

        Some(type_id)
    }

    fn insert_primitives(&mut self) {
        self.type_ids.insert("unit".to_string(), 0);
        self.types.push(
            Type {
                ident: "unit".to_string(),
                type_id: 0,
                size: 0,
                layout: TypeLayout::Unit,
                visibility: Visibility::Public,
            }
            .into(),
        );
        self.type_ids.insert("boolean".to_string(), 1);
        self.types.push(
            Type {
                ident: "boolean".to_string(),
                type_id: 1,
                size: 1,
                layout: TypeLayout::Unit,
                visibility: Visibility::Public,
            }
            .into(),
        );
        self.type_ids.insert("char".to_string(), 2);
        self.types.push(
            Type {
                ident: "char".to_string(),
                type_id: 2,
                size: 4,
                layout: TypeLayout::Unit,
                visibility: Visibility::Public,
            }
            .into(),
        );
        self.type_ids.insert("int".to_string(), 3);
        self.types.push(
            Type {
                ident: "int".to_string(),
                type_id: 3,
                size: 8,
                layout: TypeLayout::Unit,
                visibility: Visibility::Public,
            }
            .into(),
        );
        self.type_ids.insert("float".to_string(), 4);
        self.types.push(
            Type {
                ident: "float".to_string(),
                type_id: 4,
                size: 8,
                layout: TypeLayout::Unit,
                visibility: Visibility::Public,
            }
            .into(),
        );
        self.type_ids.insert("object".to_string(), 5);
        self.types.push(
            Type {
                ident: "object".to_string(),
                type_id: 5,
                size: 0,
                layout: TypeLayout::Unit,
                visibility: Visibility::Public,
            }
            .into(),
        );
    }
}

pub struct Type {
    pub ident: String,
    pub type_id: TypeId,
    pub size: usize,
    pub layout: TypeLayout,
    pub visibility: Visibility,
}

pub enum TypeLayout {
    Unit,
    Boolean,
    Char,
    Integer,
    Float,
    Object(TypeId),
    Alias(TypeId),
    Struct(Vec<StructField>),
}

pub struct StructField {
    pub ident: String,
    pub offset: usize,
    pub visibility: Visibility,
    pub type_id: TypeId,
}
