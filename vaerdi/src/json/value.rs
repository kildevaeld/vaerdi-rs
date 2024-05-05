use alloc::string::ToString;
use base64::Engine;

use crate::{List, Map, Number, String, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonValue {
    Object(Map<JsonValue>),
    List(List<JsonValue>),
    String(String),
    Bool(bool),
    Number(Number),
    Null,
}

impl From<Value> for JsonValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => JsonValue::Bool(b),
            Value::String(s) => JsonValue::String(s),
            Value::Map(m) => {
                let obj = m
                    .into_iter()
                    .map(|(k, v)| {
                        let v: JsonValue = v.into();
                        (k, v)
                    })
                    .collect::<Map<JsonValue>>();

                JsonValue::Object(obj)
            }
            Value::List(l) => {
                let list = l.into_iter().map(Into::into).collect::<List<JsonValue>>();
                JsonValue::List(list)
            }
            Value::Bytes(s) => {
                JsonValue::String(base64::engine::general_purpose::STANDARD.encode(s).into())
            }
            Value::Date(d) => JsonValue::String(d.to_string().into()),
            Value::DateTime(d) => JsonValue::String(d.and_utc().to_rfc3339().into()),
            Value::Time(t) => JsonValue::String(t.to_string().into()),
            Value::Uuid(u) => JsonValue::String(u.as_hyphenated().to_string().into()),
            Value::Number(n) => JsonValue::Number(n),
            Value::Char(c) => JsonValue::String(c.to_string().into()),
            // Value::Json(v) => v,
            Value::Null => JsonValue::Null,
        }
    }
}

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Object(o) => Value::Map(o.into()),
            JsonValue::List(l) => Value::List(l.into()),
            JsonValue::String(s) => {
                if s.len() == 36 {
                    if let Ok(id) = uuid::Uuid::parse_str(&s) {
                        return Value::Uuid(id);
                    }
                }
                Value::String(s)
            }
            JsonValue::Bool(b) => Value::Bool(b),
            JsonValue::Number(n) => Value::Number(n),
            JsonValue::Null => Value::Null,
        }
    }
}

impl From<List<JsonValue>> for List<Value> {
    fn from(value: List<JsonValue>) -> Self {
        value.into_iter().map(Into::into).collect()
    }
}

impl From<List<Value>> for List<JsonValue> {
    fn from(value: List<Value>) -> Self {
        value.into_iter().map(Into::into).collect()
    }
}

impl From<Map<JsonValue>> for Map<Value> {
    fn from(value: Map<JsonValue>) -> Self {
        value.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
}

impl From<Map<Value>> for Map<JsonValue> {
    fn from(value: Map<Value>) -> Self {
        value.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
}
