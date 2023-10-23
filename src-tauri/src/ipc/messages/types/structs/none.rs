use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct None {
}

impl None {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl _ValueBase for None {
    const _TYPE: &'static str = "none";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for None {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let value = _HashMap::new();
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}