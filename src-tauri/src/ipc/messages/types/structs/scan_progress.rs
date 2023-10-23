use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct ScanProgress {
    pub done_count: i64,
}

impl ScanProgress {
    pub fn new(done_count: i64, ) -> Self {
        Self {
            done_count,
        }
    }
}

impl _ValueBase for ScanProgress {
    const _TYPE: &'static str = "scan-progress";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for ScanProgress {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("done-count".to_owned(), self.done_count._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}