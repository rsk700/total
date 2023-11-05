use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct AggregateData {
    pub path: String,
    pub path_top: String,
    pub path_components: Vec<types::structs::PathComponent>,
    pub path_separator: String,
    pub entries: Vec<types::structs::AggregateEntry>,
}

impl AggregateData {
    pub fn new(path: String, path_top: String, path_components: Vec<types::structs::PathComponent>, path_separator: String, entries: Vec<types::structs::AggregateEntry>, ) -> Self {
        Self {
            path,
            path_top,
            path_components,
            path_separator,
            entries,
        }
    }
}

impl _ValueBase for AggregateData {
    const _TYPE: &'static str = "aggregate-data";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for AggregateData {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("path".to_owned(), self.path._to_ldm());
        value.insert("path-top".to_owned(), self.path_top._to_ldm());
        value.insert("path-components".to_owned(), self.path_components._to_ldm());
        value.insert("path-separator".to_owned(), self.path_separator._to_ldm());
        value.insert("entries".to_owned(), self.entries._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}