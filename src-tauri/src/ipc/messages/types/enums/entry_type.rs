#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::variant_value::VariantValue as _VariantValue;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub enum EntryType {
    Directory(types::structs::None),
    File(types::structs::None),
}

impl EntryType {
    const _DIRECTORY: &'static str = "directory";
    const _FILE: &'static str = "file";
}

impl _ValueBase for EntryType {
    const _TYPE: &'static str = "entry-type";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        let value = match self {
            Self::Directory(v) => v._to_ldm_struct(),
            Self::File(v) => v._to_ldm_struct(),
        };
        ldm::Value::Enum(ldm::EnumValue{
            variant: self._variant().to_owned(),
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        })
    }
}

impl _VariantValue for EntryType {
    fn _variant(&self) -> &str {
        match self {
            Self::Directory(_) => Self::_DIRECTORY,
            Self::File(_) => Self::_FILE,
        }
    }
}