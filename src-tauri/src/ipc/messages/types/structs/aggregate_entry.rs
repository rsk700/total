use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct AggregateEntry {
    pub global_id: i64,
    pub local_id: i64,
    pub name: String,
    pub path: String,
    pub self_size: i64,
    pub size: i64,
    pub tail_size: i64,
    pub self_file_count: i64,
    pub file_count: i64,
    pub tail_file_count: i64,
    pub self_dir_count: i64,
    pub dir_count: i64,
    pub tail_dir_count: i64,
    pub is_file: bool,
    pub global_parent: Option<i64>,
    pub local_parent: Option<i64>,
    pub nested: Vec<i64>,
}

impl AggregateEntry {
    pub fn new(global_id: i64, local_id: i64, name: String, path: String, self_size: i64, size: i64, tail_size: i64, self_file_count: i64, file_count: i64, tail_file_count: i64, self_dir_count: i64, dir_count: i64, tail_dir_count: i64, is_file: bool, global_parent: Option<i64>, local_parent: Option<i64>, nested: Vec<i64>, ) -> Self {
        Self {
            global_id,
            local_id,
            name,
            path,
            self_size,
            size,
            tail_size,
            self_file_count,
            file_count,
            tail_file_count,
            self_dir_count,
            dir_count,
            tail_dir_count,
            is_file,
            global_parent,
            local_parent,
            nested,
        }
    }
}

impl _ValueBase for AggregateEntry {
    const _TYPE: &'static str = "aggregate-entry";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for AggregateEntry {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("global-id".to_owned(), self.global_id._to_ldm());
        value.insert("local-id".to_owned(), self.local_id._to_ldm());
        value.insert("name".to_owned(), self.name._to_ldm());
        value.insert("path".to_owned(), self.path._to_ldm());
        value.insert("self-size".to_owned(), self.self_size._to_ldm());
        value.insert("size".to_owned(), self.size._to_ldm());
        value.insert("tail-size".to_owned(), self.tail_size._to_ldm());
        value.insert("self-file-count".to_owned(), self.self_file_count._to_ldm());
        value.insert("file-count".to_owned(), self.file_count._to_ldm());
        value.insert("tail-file-count".to_owned(), self.tail_file_count._to_ldm());
        value.insert("self-dir-count".to_owned(), self.self_dir_count._to_ldm());
        value.insert("dir-count".to_owned(), self.dir_count._to_ldm());
        value.insert("tail-dir-count".to_owned(), self.tail_dir_count._to_ldm());
        value.insert("is-file".to_owned(), self.is_file._to_ldm());
        value.insert("global-parent".to_owned(), self.global_parent._to_ldm());
        value.insert("local-parent".to_owned(), self.local_parent._to_ldm());
        value.insert("nested".to_owned(), self.nested._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}