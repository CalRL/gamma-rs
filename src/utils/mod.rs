pub mod custom_struct;

pub fn get_enum_number(enum_str: &str) -> Option<i32> {
    enum_str
        .to_string()
        .split("::")
        .last()
        .and_then(|part| part.strip_prefix("NewEnumerator"))
        .and_then(|x| x.parse::<i32>().ok())
}
