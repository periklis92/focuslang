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

    pub fn get_type_from_name(&self, name: &str) -> Option<Rc<Type>> {
        let type_id = self.get_type_id_from_name(name)?;
        self.get_type_from_id(type_id)
    }

    pub fn get_type_from_id(&self, type_id: TypeId) -> Option<Rc<Type>> {
        self.types.get(type_id).cloned()
    }

    pub fn get_type_from_expr(&mut self, ty: &parser::Type) -> Option<Rc<Type>> {
        match ty {
            parser::Type::Unit => self.get_type_from_id(0),
            parser::Type::Name(name) => self.get_type_from_name(&name),
            parser::Type::Function(function) => {
                let mut args = function
                    .args
                    .iter()
                    .map(|arg| self.get_type_from_expr(arg))
                    .collect::<Option<Vec<_>>>()?;

                if args.is_empty() {
                    args.push(self.get_type_from_name("unit")?);
                }

                let ret = self.get_type_from_expr(&function.ret)?;

                let ident = self.get_function_type_ident(args.as_slice(), &ret)?;

                if let Some(ty) = self.get_type_from_name(&ident) {
                    Some(ty)
                } else {
                    let type_id = self.type_ids.len();
                    let ty = Rc::new(Type {
                        ident,
                        type_id,
                        layout: TypeLayout::Function(FunctionType {
                            arg_types: args.iter().map(|t| t.type_id).collect(),
                            ret_type: ret.type_id,
                        }),
                        visibility: Visibility::Private,
                    });

                    self.type_ids.insert(ty.ident.clone(), type_id);
                    self.types.push(ty.clone());

                    Some(ty)
                }
            }
        }
    }

    fn get_function_type_ident<T: AsRef<Type>>(&self, args: &[T], ret: &T) -> Option<String> {
        let idents = args
            .iter()
            .map(|arg| arg.as_ref().ident.as_str())
            .chain([ret.as_ref().ident.as_str()])
            .collect::<Vec<_>>()
            .join(" -> ");

        Some(format!("( {idents} )"))
    }

    #[inline]
    pub fn get_type_id_from_name(&self, name: &str) -> Option<TypeId> {
        self.type_ids.get(name).cloned()
    }

    #[inline]
    pub fn get_struct_type_id_from_name(&self, name: &str) -> Option<TypeId> {
        let type_id = self.get_type_id_from_name(name)?;
        let ty = self.get_type_from_id(type_id)?;
        match &*ty {
            Type {
                layout: TypeLayout::Struct(_),
                ..
            } => Some(type_id),
            _ => None,
        }
    }

    pub fn are_types_equal(&self, t1: TypeId, t2: TypeId) -> Result<bool, String> {
        let t1 = self.resolve_type(
            self.get_type_from_id(t1)
                .ok_or(format!("Type with id {t1} not found"))?
                .as_ref(),
        );

        let t2 = self.resolve_type(
            self.get_type_from_id(t2)
                .ok_or(format!("Type with id {t2} not found"))?
                .as_ref(),
        );

        Ok(t1 == t2)
    }

    pub fn resolve_type(&self, ty: &Type) -> Rc<Type> {
        match &ty.layout {
            TypeLayout::Alias(type_id) => self
                .get_type_from_id(*type_id)
                .map(|t| self.resolve_type(&t))
                .expect("Unable to find registered type."),
            _ => self
                .get_type_from_name(&ty.ident)
                .expect("Unable to find registered type."),
        }
    }

    pub fn insert_or_get_array_type_for_type(&mut self, type_id: TypeId) -> Option<TypeId> {
        let ty = self.get_type_from_id(type_id)?;
        let ty_name = format!("array of {}", ty.ident);
        self.get_type_id_from_name(&ty_name).or_else(|| {
            let arr_type_id = self.types.len();
            self.types.push(
                Type {
                    ident: ty_name.clone(),
                    type_id,
                    layout: TypeLayout::Array(type_id),
                    visibility: Visibility::Public,
                }
                .into(),
            );
            self.type_ids.insert(ty_name, arr_type_id);
            Some(arr_type_id)
        })
    }

    pub fn insert_alias_type_from_item(
        &mut self,
        alias: AliasItem,
        visibility: Visibility,
    ) -> Option<TypeId> {
        let ty = self.get_type_from_name(&alias.path.unwrap_or("()".to_string()))?;
        let type_id = self.types.len();
        self.types.push(
            Type {
                ident: alias.ident.clone(),
                type_id,
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
        for f in struc.fields {
            let ty = self
                .get_type_from_expr(&f.ty)
                .map(|t| self.resolve_type(&t))?;
            fields.push(StructField {
                ident: f.ident.clone(),
                visibility: f.visibility,
                type_id: ty.type_id,
            });
        }
        let type_id = self.types.len();
        self.types.push(
            Type {
                ident: struc.ident.clone(),
                type_id,
                layout: TypeLayout::Struct(fields),
                visibility,
            }
            .into(),
        );
        self.type_ids.insert(struc.ident, type_id);

        Some(type_id)
    }

    fn insert_primitives(&mut self) {
        self.type_ids.insert(
            PrimitiveType::Unit.name().to_string(),
            PrimitiveType::Unit.type_id(),
        );
        self.types.push(PrimitiveType::Unit.to_type().into());
        self.type_ids.insert(
            PrimitiveType::Boolean.name().to_string(),
            PrimitiveType::Boolean.type_id(),
        );
        self.types.push(PrimitiveType::Boolean.to_type().into());
        self.type_ids.insert(
            PrimitiveType::Char.name().to_string(),
            PrimitiveType::Char.type_id(),
        );
        self.types.push(PrimitiveType::Char.to_type().into());
        self.type_ids.insert(
            PrimitiveType::Integer.name().to_string(),
            PrimitiveType::Integer.type_id(),
        );
        self.types.push(PrimitiveType::Integer.to_type().into());
        self.type_ids.insert(
            PrimitiveType::Float.name().to_string(),
            PrimitiveType::Float.type_id(),
        );
        self.types.push(PrimitiveType::Float.to_type().into());
        self.type_ids.insert(
            PrimitiveType::Object.name().to_string(),
            PrimitiveType::Object.type_id(),
        );
        self.types.push(PrimitiveType::Object.to_type().into());
    }
}

pub struct Type {
    pub ident: String,
    pub type_id: TypeId,
    pub layout: TypeLayout,
    pub visibility: Visibility,
}

impl Type {
    #[inline]
    pub fn is_unit(&self) -> bool {
        matches!(self.layout, TypeLayout::Unit)
    }

    #[inline]
    pub fn is_boolean(&self) -> bool {
        matches!(self.layout, TypeLayout::Boolean)
    }

    #[inline]
    pub fn is_char(&self) -> bool {
        matches!(self.layout, TypeLayout::Char)
    }

    #[inline]
    pub fn is_integer(&self) -> bool {
        matches!(self.layout, TypeLayout::Integer)
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self.layout, TypeLayout::Float)
    }

    #[inline]
    pub fn is_object(&self) -> bool {
        matches!(self.layout, TypeLayout::Object)
    }

    #[inline]
    pub fn is_function(&self) -> bool {
        matches!(self.layout, TypeLayout::Function(_))
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self.layout, TypeLayout::Array(_))
    }

    #[inline]
    pub fn is_alias(&self) -> bool {
        matches!(self.layout, TypeLayout::Alias(_))
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        matches!(self.layout, TypeLayout::Struct(_))
    }

    pub fn as_function(&self) -> Option<&FunctionType> {
        match self.layout {
            TypeLayout::Function(ref function) => Some(function),
            _ => None,
        }
    }

    pub fn as_struct(&self) -> Option<&[StructField]> {
        match self.layout {
            TypeLayout::Struct(ref fields) => Some(&fields),
            _ => None,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

pub enum TypeLayout {
    Unit,
    Boolean,
    Char,
    Integer,
    Float,
    Object,
    Function(FunctionType),
    Array(TypeId),
    Alias(TypeId),
    Struct(Vec<StructField>),
}

pub struct StructField {
    pub ident: String,
    pub visibility: Visibility,
    pub type_id: TypeId,
}

pub struct FunctionType {
    pub arg_types: Vec<TypeId>,
    pub ret_type: TypeId,
}

pub enum PrimitiveType {
    Unit,
    Boolean,
    Char,
    Integer,
    Float,
    Object,
}

impl PrimitiveType {
    pub fn name(&self) -> &str {
        match self {
            PrimitiveType::Unit => "()",
            PrimitiveType::Boolean => "bool",
            PrimitiveType::Char => "char",
            PrimitiveType::Integer => "int",
            PrimitiveType::Float => "float",
            PrimitiveType::Object => "object",
        }
    }

    pub fn type_id(&self) -> TypeId {
        match self {
            PrimitiveType::Unit => 0,
            PrimitiveType::Boolean => 1,
            PrimitiveType::Char => 2,
            PrimitiveType::Integer => 3,
            PrimitiveType::Float => 4,
            PrimitiveType::Object => 5,
        }
    }

    pub fn layout(&self) -> TypeLayout {
        match self {
            PrimitiveType::Unit => TypeLayout::Unit,
            PrimitiveType::Boolean => TypeLayout::Boolean,
            PrimitiveType::Char => TypeLayout::Char,
            PrimitiveType::Integer => TypeLayout::Integer,
            PrimitiveType::Float => TypeLayout::Float,
            PrimitiveType::Object => TypeLayout::Object,
        }
    }

    pub fn to_type(&self) -> Type {
        Type {
            ident: self.name().to_string(),
            type_id: self.type_id(),
            layout: self.layout(),
            visibility: Visibility::Public,
        }
    }
}
