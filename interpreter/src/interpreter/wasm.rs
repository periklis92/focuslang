use crate::{object::Value, Interpreter};

use js_sys::Object;
use wasm_bindgen::{convert::IntoWasmAbi, prelude::wasm_bindgen, JsValue};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Interpreter {
    pub fn interpret_str_web(&mut self, str: &str) -> Result<JsValue, String> {
        let value = self.interpret_str(str)?;
        self.to_js_value(value)
    }
}

impl Interpreter {
    pub fn to_js_value(&self, value: Value) -> Result<JsValue, String> {
        match value {
            Value::Unit => Ok(Object::new().into()),
            Value::Boolean(boolean) => {
                if boolean {
                    Ok(JsValue::TRUE)
                } else {
                    Ok(JsValue::FALSE)
                }
            }
            Value::Char(ch) => Ok(JsValue::from(ch.to_string())),
            Value::Integer(integer) => Ok(JsValue::from(integer)),
            Value::Float(float) => Ok(JsValue::from(float)),
            Value::Object(object) => {
                let ty = self
                    .type_registry
                    .get_type_from_id(object.type_id)
                    .ok_or("Type of object not found.".to_string())?;

                let fields = ty
                    .as_struct()
                    .ok_or("Invalid type of object.".to_string())?;

                let js_object = js_sys::Object::new();

                for (i, f) in fields.iter().enumerate() {
                    js_sys::Reflect::set(
                        &js_object,
                        &f.ident.clone().into(),
                        &self.to_js_value(
                            object.get_value(i).expect("Value not found in object."),
                        )?,
                    )
                    .or(Err("Unable to set property.".to_string()))?;
                }

                Ok(JsValue::from(js_object))
            }
            _ => Ok(Object::new().into()),
        }
    }
}
