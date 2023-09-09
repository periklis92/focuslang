use std::{collections::HashMap, rc::Rc};

use parser::stmt::Struct;

use crate::{
    object::{Object, Value},
    r#type::{Type, TypeLayout},
    Interpreter,
};

impl Interpreter {
    pub(super) fn interpret_struct(&mut self, r#struct: Struct) -> Result<Value, String> {
        let ty = self.type_registry.get_type_from_name(&r#struct.path);

        let Some(Type {
            type_id,
            layout: TypeLayout::Struct(fields),
            ..
        }) = ty.as_deref()
        else {
            return Err(format!("No struct type with name {} found.", r#struct.path));
        };

        let mut field_values = r#struct
            .fields
            .into_iter()
            .map(|m| (m.ident, m.expr))
            .collect::<HashMap<_, _>>();

        let mut values = Vec::new();

        for f in fields.iter() {
            let expr = field_values
                .remove(f.ident.as_str())
                .ok_or(format!("Missing field {}.", f.ident))?;

            let expr_type = self.resolve_expr_type(expr.as_ref(), None)?;
            if !self.type_registry.are_types_equal(f.type_id, expr_type)? {
                return Err(format!("Unexpected type for field {}.", f.ident));
            }

            let value = self.interpret_expression(*expr)?;
            values.push(value);
        }

        if !field_values.is_empty() {
            let fields = field_values.keys().fold(String::new(), |mut a, b| {
                if !a.is_empty() {
                    a.push(',');
                }
                a.push_str(&b);
                a
            });
            return Err(format!("Unknown field(s): {fields}"));
        }

        Ok(Value::Object(Rc::new(Object {
            values,
            type_id: *type_id,
        })))
    }
}
