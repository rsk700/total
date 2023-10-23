use std::collections::HashMap as _HashMap;
#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub struct Some<T: _ValueBase> {
    pub value: T,
}

impl<T: _ValueBase> Some<T> {
    pub fn new(value: T, ) -> Self {
        Self {
            value,
        }
    }
}

impl<T: _ValueBase> _ValueBase for Some<T> {
    const _TYPE: &'static str = "some";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
            _ValueType::new(T::_TYPE.to_owned(), T::_applied_args()),
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        ldm::Value::Struct(self._to_ldm_struct())
    }
}

impl<T: _ValueBase> _StructBase for Some<T> {
    fn _to_ldm_struct(&self) -> ldm::StructValue {
        let mut value = _HashMap::new();
        value.insert("value".to_owned(), self.value._to_ldm());
        ldm::StructValue{
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        }
    }
}