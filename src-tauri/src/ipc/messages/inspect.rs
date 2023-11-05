use std::collections::HashMap;
use super::value_type;

pub fn get_leap_structs_map() -> HashMap<String, value_type::LeapStruct> {
    [
        ("aggregate-entry".to_owned(), value_type::LeapStruct::new("aggregate-entry".to_owned(), vec![], vec![value_type::Property::new("global-id".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("local-id".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("name".to_owned(), value_type::ValueType::new("str".to_owned(), vec![])), value_type::Property::new("name-hash".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("path".to_owned(), value_type::ValueType::new("str".to_owned(), vec![])), value_type::Property::new("self-size".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("size".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("tail-size".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("self-file-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("file-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("tail-file-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("self-dir-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("dir-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("tail-dir-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![])), value_type::Property::new("is-file".to_owned(), value_type::ValueType::new("bool".to_owned(), vec![])), value_type::Property::new("global-parent".to_owned(), value_type::ValueType::new("option".to_owned(), vec![value_type::ValueType::new("int".to_owned(), vec![])])), value_type::Property::new("local-parent".to_owned(), value_type::ValueType::new("option".to_owned(), vec![value_type::ValueType::new("int".to_owned(), vec![])])), value_type::Property::new("nested".to_owned(), value_type::ValueType::new("list".to_owned(), vec![value_type::ValueType::new("int".to_owned(), vec![])]))])),
        ("path-component".to_owned(), value_type::LeapStruct::new("path-component".to_owned(), vec![], vec![value_type::Property::new("path".to_owned(), value_type::ValueType::new("str".to_owned(), vec![])), value_type::Property::new("name".to_owned(), value_type::ValueType::new("str".to_owned(), vec![]))])),
        ("aggregate-data".to_owned(), value_type::LeapStruct::new("aggregate-data".to_owned(), vec![], vec![value_type::Property::new("path".to_owned(), value_type::ValueType::new("str".to_owned(), vec![])), value_type::Property::new("path-top".to_owned(), value_type::ValueType::new("str".to_owned(), vec![])), value_type::Property::new("path-components".to_owned(), value_type::ValueType::new("list".to_owned(), vec![value_type::ValueType::new("path-component".to_owned(), vec![])])), value_type::Property::new("path-separator".to_owned(), value_type::ValueType::new("str".to_owned(), vec![])), value_type::Property::new("entries".to_owned(), value_type::ValueType::new("list".to_owned(), vec![value_type::ValueType::new("aggregate-entry".to_owned(), vec![])]))])),
        ("scan-progress".to_owned(), value_type::LeapStruct::new("scan-progress".to_owned(), vec![], vec![value_type::Property::new("done-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![]))])),
        ("navigation".to_owned(), value_type::LeapStruct::new("navigation".to_owned(), vec![], vec![value_type::Property::new("global-id".to_owned(), value_type::ValueType::new("option".to_owned(), vec![value_type::ValueType::new("int".to_owned(), vec![])])), value_type::Property::new("path".to_owned(), value_type::ValueType::new("str".to_owned(), vec![]))])),
        ("about-app".to_owned(), value_type::LeapStruct::new("about-app".to_owned(), vec![], vec![value_type::Property::new("version".to_owned(), value_type::ValueType::new("str".to_owned(), vec![]))])),
        ("none".to_owned(), value_type::LeapStruct::new("none".to_owned(), vec![], vec![])),
        ("some".to_owned(), value_type::LeapStruct::new("some".to_owned(), vec!["t".to_owned()], vec![value_type::Property::new("value".to_owned(), value_type::ValueType::new("t".to_owned(), vec![]))])),
    ].into()
}

pub fn get_leap_enums_map() -> HashMap<String, value_type::LeapEnum> {
    [
        ("scan-state".to_owned(), value_type::LeapEnum::new("scan-state".to_owned(), vec![], vec![value_type::Property::new("ready".to_owned(), value_type::ValueType::new("none".to_owned(), vec![])), value_type::Property::new("scan-progress".to_owned(), value_type::ValueType::new("scan-progress".to_owned(), vec![]))])),
        ("option".to_owned(), value_type::LeapEnum::new("option".to_owned(), vec!["t".to_owned()], vec![value_type::Property::new("none".to_owned(), value_type::ValueType::new("none".to_owned(), vec![])), value_type::Property::new("some".to_owned(), value_type::ValueType::new("some".to_owned(), vec![value_type::ValueType::new("t".to_owned(), vec![])]))])),
        ("result".to_owned(), value_type::LeapEnum::new("result".to_owned(), vec!["t".to_owned(), "e".to_owned()], vec![value_type::Property::new("ok".to_owned(), value_type::ValueType::new("some".to_owned(), vec![value_type::ValueType::new("t".to_owned(), vec![])])), value_type::Property::new("err".to_owned(), value_type::ValueType::new("some".to_owned(), vec![value_type::ValueType::new("e".to_owned(), vec![])]))])),
    ].into()
}

pub fn get_leap_variants_map() -> HashMap<String, HashMap<String, value_type::LeapVariant>> {
    [
        (
            "scan-state".to_owned(),
            [
                ("ready".to_owned(), value_type::LeapVariant::new("scan-state".to_owned(), value_type::Property::new("ready".to_owned(), value_type::ValueType::new("none".to_owned(), vec![])), vec![], vec![])),
                ("scan-progress".to_owned(), value_type::LeapVariant::new("scan-state".to_owned(), value_type::Property::new("scan-progress".to_owned(), value_type::ValueType::new("scan-progress".to_owned(), vec![])), vec![], vec![value_type::Property::new("done-count".to_owned(), value_type::ValueType::new("int".to_owned(), vec![]))])),
            ].into()
        ),
        (
            "option".to_owned(),
            [
                ("none".to_owned(), value_type::LeapVariant::new("option".to_owned(), value_type::Property::new("none".to_owned(), value_type::ValueType::new("none".to_owned(), vec![])), vec!["t".to_owned()], vec![])),
                ("some".to_owned(), value_type::LeapVariant::new("option".to_owned(), value_type::Property::new("some".to_owned(), value_type::ValueType::new("some".to_owned(), vec![value_type::ValueType::new("t".to_owned(), vec![])])), vec!["t".to_owned()], vec![value_type::Property::new("value".to_owned(), value_type::ValueType::new("t".to_owned(), vec![]))])),
            ].into()
        ),
        (
            "result".to_owned(),
            [
                ("ok".to_owned(), value_type::LeapVariant::new("result".to_owned(), value_type::Property::new("ok".to_owned(), value_type::ValueType::new("some".to_owned(), vec![value_type::ValueType::new("t".to_owned(), vec![])])), vec!["t".to_owned(), "e".to_owned()], vec![value_type::Property::new("value".to_owned(), value_type::ValueType::new("t".to_owned(), vec![]))])),
                ("err".to_owned(), value_type::LeapVariant::new("result".to_owned(), value_type::Property::new("err".to_owned(), value_type::ValueType::new("some".to_owned(), vec![value_type::ValueType::new("e".to_owned(), vec![])])), vec!["t".to_owned(), "e".to_owned()], vec![value_type::Property::new("value".to_owned(), value_type::ValueType::new("e".to_owned(), vec![]))])),
            ].into()
        ),
    ].into()
}
