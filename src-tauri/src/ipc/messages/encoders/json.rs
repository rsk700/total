use super::super::inspect;
use super::super::leap_data_model as ldm;
use super::super::value_base::ValueBase;
use super::super::value_type::{LeapEnum, LeapStruct, LeapVariant, ValueType};
use super::ldm::FromLdm;
use super::{format_error, VerificationResult, Verify};
use serde_json::map::Map as JsonMap;
use serde_json::ser::to_vec;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct FromJson<T> {
    json_value: Option<JsonValue>,
    value_type: ValueType,
    structs: HashMap<String, LeapStruct>,
    enums: HashMap<String, LeapEnum>,
    variants: HashMap<String, HashMap<String, LeapVariant>>,
    t: PhantomData<T>,
}

impl<T: ValueBase> FromJson<T>
where
    ldm::Value: FromLdm<T>,
{
    pub fn new(data: &[u8]) -> Self {
        let json_value = serde_json::from_slice(data).ok();
        let value_type = ValueType::new(T::_TYPE.to_owned(), T::_applied_args());
        Self {
            json_value,
            value_type,
            structs: inspect::get_leap_structs_map(),
            enums: inspect::get_leap_enums_map(),
            variants: inspect::get_leap_variants_map(),
            t: PhantomData,
        }
    }

    fn add_json_errors(
        &self,
        path: &[&str],
        json_value: &JsonValue,
        value_type: &ValueType,
        errors: &mut Vec<String>,
    ) {
        // note: reimplement using Generator, when Generator stabilized
        match value_type.name.as_str() {
            "str" => {
                if !json_value.is_string() {
                    errors.push(format_error(path, "expecting string"));
                }
            }
            "int" => {
                if !json_value.is_i64() {
                    let ok = if let Some(v) = json_value.as_f64() {
                        v.fract() == 0.0
                    } else {
                        false
                    };
                    if !ok {
                        errors.push(format_error(path, "expecting integer"));
                    }
                }
            }
            "float" => {
                FromJson::add_float_errors(path, json_value, errors);
            }
            "bool" => {
                if !json_value.is_boolean() {
                    errors.push(format_error(path, "expecting boolean"));
                }
            }
            "list" => {
                if let JsonValue::Array(v) = json_value {
                    self.add_list_errors(path, v, value_type, errors)
                } else {
                    errors.push(format_error(path, "expecting list"));
                }
            }
            "option" => {
                if value_type.args[0].name == "option" {
                    if let JsonValue::Object(obj) = json_value {
                        self.add_enum_errors(path, obj, value_type, errors);
                    } else {
                        errors.push(format_error(path, "expecting object"));
                    }
                } else {
                    if !json_value.is_null() {
                        self.add_json_errors(path, json_value, &value_type.args[0], errors);
                    }
                }
            }
            _ => {
                if let JsonValue::Object(obj) = json_value {
                    if self.structs.contains_key(&value_type.name) {
                        self.add_struct_errors(path, obj, value_type, errors);
                    } else if self.enums.contains_key(&value_type.name) {
                        self.add_enum_errors(path, obj, value_type, errors);
                    } else {
                        panic!("incorrect value type name")
                    }
                } else {
                    errors.push(format_error(path, "expecting object"));
                }
            }
        }
    }

    fn add_float_errors(path: &[&str], json_value: &JsonValue, errors: &mut Vec<String>) {
        if json_value.is_f64() || json_value.is_i64() {
            return;
        } else if let JsonValue::String(s) = json_value {
            if s != "+inf" && s != "-inf" && s != "nan" {
                errors.push(format_error(path, "expecting float"));
            }
        } else {
            errors.push(format_error(path, "expecting float"));
        }
    }

    fn add_list_errors(
        &self,
        path: &[&str],
        json_values: &[JsonValue],
        value_type: &ValueType,
        errors: &mut Vec<String>,
    ) {
        let item_type = &value_type.args[0];
        for (i, v) in json_values.iter().enumerate() {
            self.add_json_errors(&[path, &[&i.to_string()]].concat(), v, item_type, errors);
        }
    }

    fn add_struct_errors(
        &self,
        path: &[&str],
        json_obj: &JsonMap<String, JsonValue>,
        value_type: &ValueType,
        errors: &mut Vec<String>,
    ) {
        let struct_type = self
            .structs
            .get(&value_type.name)
            .unwrap()
            .apply_args(&value_type.args);
        for prop in struct_type.props {
            if let Some(v) = json_obj.get(&prop.name) {
                self.add_json_errors(&[path, &[&prop.name]].concat(), v, &prop.prop_type, errors);
            } else {
                errors.push(format_error(
                    &[path, &[&prop.name]].concat(),
                    "property is missing",
                ));
            }
        }
    }

    fn add_enum_errors(
        &self,
        path: &[&str],
        json_obj: &JsonMap<String, JsonValue>,
        value_type: &ValueType,
        errors: &mut Vec<String>,
    ) {
        let variant_name = if let Some(v) = json_obj.get("_t") {
            v
        } else {
            errors.push(format_error(
                &[path, &[&"_t"]].concat(),
                "variant name is missing",
            ));
            return;
        };
        let variant_name = if let JsonValue::String(s) = variant_name {
            s
        } else {
            errors.push(format_error(
                &[path, &[&"_t"]].concat(),
                "expecting string with variant name",
            ));
            return;
        };
        let variant_type = if let Some(v) = self
            .variants
            .get(&value_type.name)
            .unwrap()
            .get(variant_name)
        {
            v.apply_args(&value_type.args)
        } else {
            errors.push(format_error(
                &[path, &[&"_t"]].concat(),
                "unknown variant name",
            ));
            return;
        };
        self.add_struct_errors(path, json_obj, &variant_type.variant.prop_type, errors);
    }

    fn json_to_ldm(&self, json_value: &JsonValue, value_type: &ValueType) -> Option<ldm::Value> {
        match value_type.name.as_str() {
            "str" => {
                if let JsonValue::String(s) = json_value {
                    Some(ldm::Value::new_string(s.clone()))
                } else {
                    None
                }
            }
            "int" => {
                if let JsonValue::Number(n) = json_value {
                    let v = n.as_i64().map(|v| ldm::Value::new_integer(v));
                    if v.is_some() {
                        v
                    } else if let Some(v_float) = n.as_f64() {
                        if v_float.fract() == 0.0 {
                            Some(ldm::Value::new_integer(v_float as i64))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "float" => match json_value {
                JsonValue::Number(n) => n.as_f64().map(|v| ldm::Value::new_float(v)),
                JsonValue::String(s) => match s.as_ref() {
                    "+inf" => Some(ldm::Value::new_float(f64::INFINITY)),
                    "-inf" => Some(ldm::Value::new_float(f64::NEG_INFINITY)),
                    "nan" => Some(ldm::Value::new_float(f64::NAN)),
                    _ => None,
                },
                _ => None,
            },
            "bool" => {
                if let JsonValue::Bool(b) = json_value {
                    Some(ldm::Value::new_boolean(*b))
                } else {
                    None
                }
            }
            "list" => {
                if let JsonValue::Array(v) = json_value {
                    self.list_to_ldm(v, value_type)
                } else {
                    None
                }
            }
            "option" => {
                if value_type.args[0].name != "option" {
                    self.option_to_ldm(json_value, value_type)
                } else if let JsonValue::Object(obj) = json_value {
                    self.enum_to_ldm(obj, value_type)
                } else {
                    None
                }
            }
            _ => {
                if let JsonValue::Object(obj) = json_value {
                    if self.structs.contains_key(&value_type.name) {
                        Some(ldm::Value::Struct(self.struct_to_ldm(obj, value_type)?))
                    } else if self.enums.contains_key(&value_type.name) {
                        self.enum_to_ldm(obj, value_type)
                    } else {
                        panic!("incorrect value type name")
                    }
                } else {
                    None
                }
            }
        }
    }

    fn list_to_ldm(&self, json_values: &[JsonValue], value_type: &ValueType) -> Option<ldm::Value> {
        let mut items = vec![];
        let item_type = &value_type.args[0];
        for v in json_values {
            if let Some(item) = self.json_to_ldm(v, item_type) {
                items.push(item)
            } else {
                return None;
            }
        }
        Some(ldm::Value::new_list(items, value_type.clone()))
    }

    fn option_to_ldm(&self, json_value: &JsonValue, value_type: &ValueType) -> Option<ldm::Value> {
        match json_value {
            JsonValue::Null => Some(ldm::Value::new_enum(
                "none".to_owned(),
                ldm::StructValue {
                    value: HashMap::new(),
                    value_type: ValueType::new("none".to_owned(), vec![]),
                },
                value_type.clone(),
            )),
            _ => Some(ldm::Value::new_enum(
                "some".to_owned(),
                ldm::StructValue {
                    value: [(
                        "value".to_owned(),
                        self.json_to_ldm(json_value, &value_type.args[0])?,
                    )]
                    .into(),
                    value_type: ValueType::new("some".to_owned(), value_type.args.clone()),
                },
                value_type.clone(),
            )),
        }
    }

    fn struct_to_ldm(
        &self,
        json_obj: &JsonMap<String, JsonValue>,
        value_type: &ValueType,
    ) -> Option<ldm::StructValue> {
        let struct_type = self
            .structs
            .get(&value_type.name)
            .unwrap()
            .apply_args(&value_type.args);
        let mut props = HashMap::new();
        for prop in struct_type.props {
            props.insert(
                prop.name.clone(),
                self.json_to_ldm(json_obj.get(&prop.name)?, &prop.prop_type)?,
            );
        }
        Some(ldm::StructValue {
            value: props,
            value_type: value_type.clone(),
        })
    }

    fn enum_to_ldm(
        &self,
        json_obj: &JsonMap<String, JsonValue>,
        value_type: &ValueType,
    ) -> Option<ldm::Value> {
        let variant_name = if let JsonValue::String(s) = json_obj.get("_t")? {
            s
        } else {
            return None;
        };
        let variant_type = self
            .variants
            .get(&value_type.name)
            .unwrap()
            .get(variant_name)?
            .apply_args(&value_type.args);
        return Some(ldm::Value::new_enum(
            variant_name.clone(),
            self.struct_to_ldm(json_obj, &variant_type.variant.prop_type)?,
            value_type.clone(),
        ));
    }

    pub fn to_ldm(&self) -> Option<ldm::Value> {
        let json_value = self.json_value.as_ref()?;
        self.json_to_ldm(json_value, &self.value_type)
    }

    pub fn to_value(&self) -> Option<T> {
        Some(self.to_ldm()?.to_value())
    }
}

impl<T: ValueBase> Verify for FromJson<T>
where
    ldm::Value: FromLdm<T>,
{
    fn verify(&self, _up_to_first_error: bool) -> VerificationResult {
        // note: up_to_first_error is ignored until Generator stabilized https://doc.rust-lang.org/std/ops/trait.Generator.html
        let mut errors = vec![];
        if let Some(json_value) = self.json_value.as_ref() {
            self.add_json_errors(&[], json_value, &self.value_type, &mut errors);
        } else {
            errors.push("incorrect json data".to_owned());
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub struct ToJson;

impl ToJson {
    pub fn from_ldm(value: &ldm::Value) -> Vec<u8> {
        to_vec(&Self::ldm_to_json(value)).unwrap()
    }

    pub fn from_value<T>(value: &T) -> Vec<u8>
    where
        T: ValueBase,
    {
        Self::from_ldm(&value._to_ldm())
    }

    fn ldm_to_json(value: &ldm::Value) -> JsonValue {
        match value {
            ldm::Value::String(v) => JsonValue::String(v.0.clone()),
            ldm::Value::Integer(v) => JsonValue::Number(serde_json::Number::from(v.0)),
            ldm::Value::Float(v) => Self::float_to_json(v.0),
            ldm::Value::Boolean(v) => JsonValue::Bool(v.0),
            ldm::Value::List(v) => Self::list_to_json(v),
            ldm::Value::Struct(s) => Self::struct_to_json(s),
            ldm::Value::Enum(e) => Self::enum_to_json(e),
        }
    }

    fn float_to_json(value: f64) -> JsonValue {
        if value.is_finite() {
            JsonValue::Number(serde_json::Number::from_f64(value).unwrap())
        } else if value.is_nan() {
            JsonValue::String("nan".to_owned())
        } else if value == f64::INFINITY {
            JsonValue::String("+inf".to_owned())
        } else if value == f64::NEG_INFINITY {
            JsonValue::String("-inf".to_owned())
        } else {
            unreachable!()
        }
    }

    fn list_to_json(list_value: &ldm::ListValue) -> JsonValue {
        let json_values = list_value.value.iter().map(Self::ldm_to_json).collect();
        JsonValue::Array(json_values)
    }

    fn struct_to_json(struct_value: &ldm::StructValue) -> JsonValue {
        let mut obj = JsonMap::with_capacity(struct_value.value.len());
        for (name, value) in &struct_value.value {
            obj.insert(name.clone(), Self::ldm_to_json(value));
        }
        JsonValue::Object(obj)
    }

    fn enum_to_json(value: &ldm::EnumValue) -> JsonValue {
        if value.value_type.name == "option" && value.value_type.args[0].name != "option" {
            if value.variant == "none" {
                JsonValue::Null
            } else {
                Self::ldm_to_json(value.value.value.get("value").unwrap())
            }
        } else {
            let mut json_obj = Self::struct_to_json(&value.value);
            if let JsonValue::Object(ref mut obj) = json_obj {
                obj.insert("_t".to_owned(), JsonValue::String(value.variant.clone()));
            }
            json_obj
        }
    }
}
