use super::{number, ValueDeserializer};
use crate::{json::JsonValue, Value};
use core::marker::PhantomData;
use serde::{de, forward_to_deserialize_any};

pub struct JsonDeserializer<E> {
    value: JsonValue,
    error: PhantomData<fn() -> E>,
}

impl<E> JsonDeserializer<E> {
    pub fn new(value: JsonValue) -> Self {
        JsonDeserializer {
            value,
            error: Default::default(),
        }
    }
}

impl<'de, E> de::Deserializer<'de> for JsonDeserializer<E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            JsonValue::Bool(b) => visitor.visit_bool(b),
            JsonValue::List(l) => visitor.visit_seq(de::value::SeqDeserializer::new(
                l.into_iter().map(JsonDeserializer::new),
            )),
            JsonValue::Null => visitor.visit_none(),
            JsonValue::Number(n) => number::NumberDeserializer::new(n).deserialize_any(visitor),
            JsonValue::Object(o) => visitor.visit_map(de::value::MapDeserializer::new(
                o.into_iter().map(|(k, v)| {
                    (
                        ValueDeserializer::new(Value::String(k)),
                        JsonDeserializer::new(v),
                    )
                }),
            )),
            JsonValue::String(s) => visitor.visit_str(&s),
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit
        seq bytes byte_buf map unit_struct option
        tuple_struct struct tuple ignored_any identifier newtype_struct enum
    }
}

impl<'de, E> de::IntoDeserializer<'de, E> for JsonDeserializer<E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
