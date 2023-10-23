use super::leap_data_model as ldm;

pub trait StructBase {
    fn _to_ldm_struct(&self) -> ldm::StructValue;
}