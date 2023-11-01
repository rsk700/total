use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct AboutApp {
    pub version: String,
}

impl AboutApp {
    pub fn new(version: String, ) -> Self {
        Self {
            version,
        }
    }
}

impl _ValueBase for AboutApp {
    const _TYPE: &'static str = "about-app";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for AboutApp {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("version".to_owned(), self.version._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}