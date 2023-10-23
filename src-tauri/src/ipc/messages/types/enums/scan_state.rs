#[allow(unused_imports)]
use super::super::super::types;
use super::super::super::value_base::ValueBase as _ValueBase;
use super::super::super::variant_value::VariantValue as _VariantValue;
use super::super::super::struct_base::StructBase as _StructBase;
use super::super::super::leap_data_model as ldm;
use super::super::super::value_type::ValueType as _ValueType;

#[derive(Clone, Debug)]
pub enum ScanState {
    Ready(types::structs::None),
    ScanProgress(types::structs::ScanProgress),
}

impl ScanState {
    const _READY: &'static str = "ready";
    const _SCAN_PROGRESS: &'static str = "scan-progress";
}

impl _ValueBase for ScanState {
    const _TYPE: &'static str = "scan-state";

    fn _applied_args() -> Vec<_ValueType> {
        vec![
        ]
    }

    fn _to_ldm(&self) -> ldm::Value {
        let value = match self {
            Self::Ready(v) => v._to_ldm_struct(),
            Self::ScanProgress(v) => v._to_ldm_struct(),
        };
        ldm::Value::Enum(ldm::EnumValue{
            variant: self._variant().to_owned(),
            value,
            value_type: _ValueType::new(Self::_TYPE.to_owned(), Self::_applied_args()),
        })
    }
}

impl _VariantValue for ScanState {
    fn _variant(&self) -> &str {
        match self {
            Self::Ready(_) => Self::_READY,
            Self::ScanProgress(_) => Self::_SCAN_PROGRESS,
        }
    }
}