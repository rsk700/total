use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct PathAggregate {
    pub entries: Vec<types::structs::Entry>,
    pub tree: Vec<Vec<i64>>,
}

impl PathAggregate {
    pub fn new(entries: Vec<types::structs::Entry>, tree: Vec<Vec<i64>>, ) -> Self {
        Self {
            entries,
            tree,
        }
    }
}

impl _ValueBase for PathAggregate {
    const _TYPE: &'static str = "path-aggregate";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for PathAggregate {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("entries".to_owned(), self.entries._to_ldm());
        value.insert("tree".to_owned(), self.tree._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}