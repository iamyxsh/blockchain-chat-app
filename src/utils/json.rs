use serde::Serialize;

pub fn ser_json<T>(value: &T) -> String
where
    T: Serialize,
{
    serde_json::to_string(value).unwrap_or_default()
}
