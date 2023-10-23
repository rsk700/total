use super::value_type::ValueType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    String(StringValue),
    Integer(IntegerValue),
    Float(FloatValue),
    Boolean(BooleanValue),
    List(ListValue),
    Struct(StructValue),
    Enum(EnumValue),
}

impl Value {
    pub fn new_string(value: String) -> Self {
        Value::String(StringValue(value))
    }

    pub fn new_integer(value: i64) -> Self {
        Value::Integer(IntegerValue(value))
    }

    pub fn new_float(value: f64) -> Self {
        Value::Float(FloatValue(value))
    }

    pub fn new_boolean(value: bool) -> Self {
        Value::Boolean(BooleanValue(value))
    }

    pub fn new_list(value: Vec<Value>, value_type: ValueType) -> Self {
        Value::List(ListValue { value, value_type })
    }

    pub fn new_struct(value: HashMap<String, Value>, value_type: ValueType) -> Self {
        Value::Struct(StructValue { value, value_type })
    }

    pub fn new_enum(variant: String, value: StructValue, value_type: ValueType) -> Self {
        Value::Enum(EnumValue {
            variant,
            value,
            value_type,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StringValue(pub String);

#[derive(Debug, Clone)]
pub struct IntegerValue(pub i64);

#[derive(Debug, Clone)]
pub struct FloatValue(pub f64);

#[derive(Debug, Clone)]
pub struct BooleanValue(pub bool);

#[derive(Debug, Clone)]
pub struct ListValue {
    pub value: Vec<Value>,
    pub value_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct StructValue {
    pub value: HashMap<String, Value>,
    pub value_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub variant: String,
    pub value: StructValue,
    pub value_type: ValueType,
}
