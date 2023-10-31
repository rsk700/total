use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct Navigation {
    pub global_id: i64,
    pub path: String,
}

impl Navigation {
    pub fn new(global_id: i64, path: String, ) -> Self {
        Self {
            global_id,
            path,
        }
    }
}

impl _ValueBase for Navigation {
    const _TYPE: &'static str = "navigation";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for Navigation {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("global-id".to_owned(), self.global_id._to_ldm());
        value.insert("path".to_owned(), self.path._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}