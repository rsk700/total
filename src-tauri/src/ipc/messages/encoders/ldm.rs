use super::super::leap_data_model as ldm;
use super::super::types;
use super::super::value_base::ValueBase as _ValueBase;

pub trait FromLdm<T> {
    fn to_value(&self) -> T;
}

impl FromLdm<String> for ldm::Value {
    fn to_value(&self) -> String {
        if let ldm::Value::String(v) = self {
            v.0.clone()
        } else {
            panic!("unexpected value type");
        }
    }
}

impl FromLdm<i64> for ldm::Value {
    fn to_value(&self) -> i64 {
        if let ldm::Value::Integer(v) = self {
            v.0
        } else {
            panic!("unexpected value type");
        }
    }
}

impl FromLdm<f64> for ldm::Value {
    fn to_value(&self) -> f64 {
        if let ldm::Value::Float(v) = self {
            v.0
        } else {
            panic!("unexpected value type");
        }
    }
}

impl FromLdm<bool> for ldm::Value {
    fn to_value(&self) -> bool {
        if let ldm::Value::Boolean(v) = self {
            v.0
        } else {
            panic!("unexpected value type");
        }
    }
}

impl<T: _ValueBase> FromLdm<Vec<T>> for ldm::Value
where
    ldm::Value: FromLdm<T>,
{
    fn to_value(&self) -> Vec<T> {
        if let ldm::Value::List(v) = self {
            v.value.iter().map(|e| e.to_value()).collect()
        } else {
            panic!("unexpected value type");
        }
    }
}

impl<T: _ValueBase> FromLdm<Option<T>> for ldm::Value
where
    ldm::Value: FromLdm<T>,
{
    fn to_value(&self) -> Option<T> {
        if let ldm::Value::Enum(e) = self {
            if e.value_type.name == "option" {
                match e.variant.as_ref() {
                    "some" => {
                        if let Some(v) = e.value.value.get("value") {
                            Some(v.to_value())
                        } else {
                            panic!("unexpected value type");
                        }
                    }
                    "none" => None,
                    _ => panic!("unexpected value type"),
                }
            } else {
                panic!("unexpected value type");
            }
        } else {
            panic!("unexpected value type");
        }
    }
}

impl<T: _ValueBase, E: _ValueBase> FromLdm<Result<T, E>> for ldm::Value
where
    ldm::Value: FromLdm<T> + FromLdm<E>,
{
    fn to_value(&self) -> Result<T, E> {
        if let ldm::Value::Enum(e) = self {
            if e.value_type.name == "result" {
                match e.variant.as_ref() {
                    "ok" => {
                        if let Some(v) = e.value.value.get("value") {
                            Ok(v.to_value())
                        } else {
                            panic!("unexpected value type");
                        }
                    }
                    "err" => {
                        if let Some(v) = e.value.value.get("value") {
                            Err(v.to_value())
                        } else {
                            panic!("unexpected value type");
                        }
                    }
                    _ => panic!("unexpected value type"),
                }
            } else {
                panic!("unexpected value type");
            }
        } else {
            panic!("unexpected value type");
        }
    }
}

impl<T: _ValueBase> FromLdm<Box<T>> for ldm::Value
where
    ldm::Value: FromLdm<T>
{
    fn to_value(&self) -> Box<T> {
        Box::new(FromLdm::<T>::to_value(self))
    }
}
impl FromLdm<types::structs::AggregateEntry> for ldm::StructValue
where
{
    fn to_value(&self) -> types::structs::AggregateEntry {
        types::structs::AggregateEntry {
            id: FromLdm::<i64>::to_value(self.value.get("id").expect("unexpected value type")),
            name: FromLdm::<String>::to_value(self.value.get("name").expect("unexpected value type")),
            path: FromLdm::<String>::to_value(self.value.get("path").expect("unexpected value type")),
            self_size: FromLdm::<i64>::to_value(self.value.get("self-size").expect("unexpected value type")),
            size: FromLdm::<i64>::to_value(self.value.get("size").expect("unexpected value type")),
            tail_size: FromLdm::<i64>::to_value(self.value.get("tail-size").expect("unexpected value type")),
            self_file_count: FromLdm::<i64>::to_value(self.value.get("self-file-count").expect("unexpected value type")),
            file_count: FromLdm::<i64>::to_value(self.value.get("file-count").expect("unexpected value type")),
            tail_file_count: FromLdm::<i64>::to_value(self.value.get("tail-file-count").expect("unexpected value type")),
            self_dir_count: FromLdm::<i64>::to_value(self.value.get("self-dir-count").expect("unexpected value type")),
            dir_count: FromLdm::<i64>::to_value(self.value.get("dir-count").expect("unexpected value type")),
            tail_dir_count: FromLdm::<i64>::to_value(self.value.get("tail-dir-count").expect("unexpected value type")),
            is_file: FromLdm::<bool>::to_value(self.value.get("is-file").expect("unexpected value type")),
            parent: FromLdm::<Option<i64>>::to_value(self.value.get("parent").expect("unexpected value type")),
            nested: FromLdm::<Vec<i64>>::to_value(self.value.get("nested").expect("unexpected value type")),
        }
    }
}
impl FromLdm<types::structs::ScanProgress> for ldm::StructValue
where
{
    fn to_value(&self) -> types::structs::ScanProgress {
        types::structs::ScanProgress {
            done_count: FromLdm::<i64>::to_value(self.value.get("done-count").expect("unexpected value type")),
        }
    }
}
impl FromLdm<types::structs::None> for ldm::StructValue
where
{
    fn to_value(&self) -> types::structs::None {
        types::structs::None {
        }
    }
}
impl<T: _ValueBase> FromLdm<types::structs::Some<T>> for ldm::StructValue
where
    ldm::Value: FromLdm<T>,
{
    fn to_value(&self) -> types::structs::Some<T> {
        types::structs::Some {
            value: FromLdm::<T>::to_value(self.value.get("value").expect("unexpected value type")),
        }
    }
}
impl FromLdm<types::structs::AggregateEntry> for ldm::Value
where
{
    fn to_value(&self) -> types::structs::AggregateEntry {
        if let ldm::Value::Struct(s) = self {
            s.to_value()
        } else {
            panic!("unexpected value type");
        }
    }
}
impl FromLdm<types::structs::ScanProgress> for ldm::Value
where
{
    fn to_value(&self) -> types::structs::ScanProgress {
        if let ldm::Value::Struct(s) = self {
            s.to_value()
        } else {
            panic!("unexpected value type");
        }
    }
}
impl FromLdm<types::structs::None> for ldm::Value
where
{
    fn to_value(&self) -> types::structs::None {
        if let ldm::Value::Struct(s) = self {
            s.to_value()
        } else {
            panic!("unexpected value type");
        }
    }
}
impl<T: _ValueBase> FromLdm<types::structs::Some<T>> for ldm::Value
where
    ldm::Value: FromLdm<T>,
{
    fn to_value(&self) -> types::structs::Some<T> {
        if let ldm::Value::Struct(s) = self {
            s.to_value()
        } else {
            panic!("unexpected value type");
        }
    }
}
impl FromLdm<types::enums::ScanState> for ldm::Value
where
{
    fn to_value(&self) -> types::enums::ScanState {
        if let ldm::Value::Enum(e) = self {
            match e.variant.as_ref() {
                "ready" => types::enums::ScanState::Ready(FromLdm::<types::structs::None>::to_value(&e.value)),
                "scan-progress" => types::enums::ScanState::ScanProgress(FromLdm::<types::structs::ScanProgress>::to_value(&e.value)),
                _ => panic!("unexpected value type")
            }
        }
        else {
            panic!("unexpected value type");
        }
    }
}