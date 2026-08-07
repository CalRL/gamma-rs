use crate::save::beta::StorageType;

pub mod custom_struct;

pub(crate) fn get_enum_number(enum_str: &str) -> Option<i32> {
    enum_str
        .to_string()
        .split("::")
        .last()
        .and_then(|part| part.strip_prefix("NewEnumerator"))
        .and_then(|x| x.parse::<i32>().ok())
}

pub(crate) fn property_key(storage_type: StorageType, key: &str) -> String {
    match storage_type {
        StorageType::PARTY => format!("Party{}", key),
        StorageType::BOXES(num) => format!("Box{}{}", num, key),
    }
}
