use alloc::string::ToString;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use core::fmt;
use core::marker::PhantomData;
use serde::{de, forward_to_deserialize_any};

use super::DeserializerError;

pub struct TimeVisitor;

impl<'de> de::Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("any numeric value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(datetime) = v.parse() {
            Ok(Time::DateTime(datetime))
        } else if let Ok(date) = v.parse() {
            Ok(Time::Date(date))
        } else if let Ok(time) = v.parse() {
            Ok(Time::Time(time))
        } else {
            panic!()
        }
    }

    fn visit_string<E>(self, v: alloc::string::String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }
}

pub enum Time {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Time(NaiveTime),
}

pub struct TimeDeserializer<E> {
    value: Time,
    error: PhantomData<fn() -> E>,
}

impl<E> TimeDeserializer<E> {
    pub fn new(value: Time) -> Self {
        TimeDeserializer {
            value,
            error: Default::default(),
        }
    }

    // pub fn into_time(self) -> Time {
    //     self.value
    // }
}

impl<'de, E> de::Deserializer<'de> for TimeDeserializer<E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Time::Time(time) => visitor.visit_string(time.to_string()),
            Time::Date(m) => visitor.visit_string(m.to_string()),
            Time::DateTime(m) => visitor.visit_string(m.to_string()),
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit
        seq bytes byte_buf map unit_struct option
        tuple_struct struct tuple ignored_any identifier newtype_struct enum
    }
}

impl<'de, E> de::IntoDeserializer<'de, E> for TimeDeserializer<E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> de::Deserializer<'de> for Time {
    type Error = DeserializerError;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_any(visitor)
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_option(visitor)
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_enum(name, variants, visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        TimeDeserializer::new(self).deserialize_newtype_struct(name, visitor)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit
        seq bytes byte_buf map unit_struct
        tuple_struct struct tuple ignored_any identifier
    }
}
