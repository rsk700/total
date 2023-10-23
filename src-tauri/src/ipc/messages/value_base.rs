use super::leap_data_model as ldm;
use super::value_type::ValueType;
use std::collections::HashMap;

pub trait ValueBase {
    const _TYPE: &'static str;
    fn _type(&self) -> &str {
        Self::_TYPE
    }
    fn _applied_args() -> Vec<ValueType>;
    fn _value_type(&self) -> ValueType {
        ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args())
    }
    fn _to_ldm(&self) -> ldm::Value;
}

impl ValueBase for String {
    const _TYPE: &'static str = "str";

    fn _applied_args() -> Vec<ValueType> {
        vec![]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::String(ldm::StringValue(self.clone()))
    }
}

impl ValueBase for i64 {
    const _TYPE: &'static str = "int";

    fn _applied_args() -> Vec<ValueType> {
        vec![]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Integer(ldm::IntegerValue(*self))
    }
}

impl ValueBase for f64 {
    const _TYPE: &'static str = "float";

    fn _applied_args() -> Vec<ValueType> {
        vec![]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Float(ldm::FloatValue(*self))
    }
}

impl ValueBase for bool {
    const _TYPE: &'static str = "bool";

    fn _applied_args() -> Vec<ValueType> {
        vec![]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Boolean(ldm::BooleanValue(*self))
    }
}

impl<T: ValueBase> ValueBase for Vec<T> {
    const _TYPE: &'static str = "list";

    fn _applied_args() -> Vec<ValueType> {
        vec![ValueType::new(T::_TYPE.to_owned(), T::_applied_args())]
    }

    fn _to_ldm(&self) -> ldm::Value {
        let value = self.iter().map(|v| v._to_ldm()).collect();
        ldm::Value::List(ldm::ListValue {
            value,
            value_type: ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        })
    }
}

impl<T: ValueBase> ValueBase for Option<T> {
    const _TYPE: &'static str = "option";

    fn _applied_args() -> Vec<ValueType> {
        vec![ValueType::new(T::_TYPE.to_owned(), T::_applied_args())]
    }

    fn _to_ldm(&self) -> ldm::Value {
        if let Some(v) = self {
            let mut value = HashMap::new();
            value.insert("value".to_owned(), v._to_ldm());
            ldm::Value::Enum(ldm::EnumValue {
                variant: "some".to_owned(),
                value: ldm::StructValue {
                    value,
                    value_type: ValueType::new("some".to_owned(), Self::_applied_args()),
                },
                value_type: ValueType::new("option".to_owned(), Self::_applied_args()),
            })
        } else {
            ldm::Value::Enum(ldm::EnumValue {
                variant: "none".to_owned(),
                value: ldm::StructValue {
                    value: HashMap::new(),
                    value_type: ValueType::new("none".to_owned(), vec![]),
                },
                value_type: ValueType::new("option".to_owned(), Self::_applied_args()),
            })
        }
    }
}

impl<T: ValueBase, E: ValueBase> ValueBase for Result<T, E> {
    const _TYPE: &'static str = "result";

    fn _applied_args() -> Vec<ValueType> {
        vec![
            ValueType::new(T::_TYPE.to_owned(), T::_applied_args()),
            ValueType::new(E::_TYPE.to_owned(), E::_applied_args()),
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        let mut value = HashMap::new();
        match self {
            Ok(v) => {
                value.insert("value".to_owned(), v._to_ldm());
                ldm::Value::Enum(ldm::EnumValue {
                    variant: "ok".to_owned(),
                    value: ldm::StructValue {
                        value,
                        value_type: ValueType::new(
                            "ok".to_owned(),
                            vec![ValueType::new(T::_TYPE.to_owned(), T::_applied_args())],
                        ),
                    },
                    value_type: ValueType::new("result".to_owned(), Self::_applied_args()),
                })
            }
            Err(e) => {
                value.insert("value".to_owned(), e._to_ldm());
                ldm::Value::Enum(ldm::EnumValue {
                    variant: "err".to_owned(),
                    value: ldm::StructValue {
                        value,
                        value_type: ValueType::new(
                            "err".to_owned(),
                            vec![ValueType::new(E::_TYPE.to_owned(), E::_applied_args())],
                        ),
                    },
                    value_type: ValueType::new("result".to_owned(), Self::_applied_args()),
                })
            }
        }
    }
}

impl<T: ValueBase> ValueBase for Box<T> {
    const _TYPE: &'static str = T::_TYPE;

    fn _applied_args() -> Vec<ValueType> {
        T::_applied_args()
    }

    fn _to_ldm(&self) -> ldm::Value {
        T::_to_ldm(self)
    }
}
