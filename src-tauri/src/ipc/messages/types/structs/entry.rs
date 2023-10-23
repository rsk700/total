use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct Entry {
    pub id: i64,
    pub name: String,
    pub path: Vec<String>,
    pub self_size: i64,
    pub size: i64,
    pub self_file_count: i64,
    pub file_count: i64,
    pub self_dir_count: i64,
    pub dir_count: i64,
    pub entry_type: types::enums::EntryType,
    pub parent: Option<i64>,
    pub entries: Vec<i64>,
}

impl Entry {
    pub fn new(id: i64, name: String, path: Vec<String>, self_size: i64, size: i64, self_file_count: i64, file_count: i64, self_dir_count: i64, dir_count: i64, entry_type: types::enums::EntryType, parent: Option<i64>, entries: Vec<i64>, ) -> Self {
        Self {
            id,
            name,
            path,
            self_size,
            size,
            self_file_count,
            file_count,
            self_dir_count,
            dir_count,
            entry_type,
            parent,
            entries,
        }
    }
}

impl _ValueBase for Entry {
    const _TYPE: &'static str = "entry";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl _StructBase for Entry {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("id".to_owned(), self.id._to_ldm());
        value.insert("name".to_owned(), self.name._to_ldm());
        value.insert("path".to_owned(), self.path._to_ldm());
        value.insert("self-size".to_owned(), self.self_size._to_ldm());
        value.insert("size".to_owned(), self.size._to_ldm());
        value.insert("self-file-count".to_owned(), self.self_file_count._to_ldm());
        value.insert("file-count".to_owned(), self.file_count._to_ldm());
        value.insert("self-dir-count".to_owned(), self.self_dir_count._to_ldm());
        value.insert("dir-count".to_owned(), self.dir_count._to_ldm());
        value.insert("entry-type".to_owned(), self.entry_type._to_ldm());
        value.insert("parent".to_owned(), self.parent._to_ldm());
        value.insert("entries".to_owned(), self.entries._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}