use crate::{bytes::Bytes, number::Number, List, Map, String, Type};

use super::value::Value;
use alloc::{
    string::{String as StdString, ToString},
    vec,
    vec::Vec,
};
use core::fmt;
use serde::ser;
use serde::ser::StdError;

#[derive(Debug)]
pub enum SerializerError {
    Custom(StdString),
}

impl fmt::Display for SerializerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerializerError::Custom(ref s) => fmt.write_str(s),
        }
    }
}

impl StdError for SerializerError {}

impl ser::Error for SerializerError {
    fn custom<T: fmt::Display>(msg: T) -> SerializerError {
        SerializerError::Custom(msg.to_string())
    }
}

impl ser::Serialize for Value {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match *self {
            Value::Bool(v) => s.serialize_bool(v),
            Value::Char(v) => s.serialize_char(v),
            Value::String(ref v) => s.serialize_str(v),
            // Value::Json(ref j) => j.serialize(s),
            Value::Number(n) => n.serialize(s),
            // Value::Option(Some(ref v)) => s.serialize_some(v),
            Value::Null => s.serialize_none(),
            Value::List(ref v) => v.serialize(s),
            Value::Map(ref v) => v.serialize(s),
            Value::Bytes(ref v) => s.serialize_bytes(v.as_ref()),
            Value::Time(ref m) => m.serialize(s),
            Value::Date(ref m) => m.serialize(s),
            Value::DateTime(ref m) => m.serialize(s),
            Value::Uuid(ref m) => m.serialize(s),
        }
    }
}

// impl ser::Serialize for JsonValue {
//     fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
//         match self {
//             Self::Bool(b) => b.serialize(s),
//             Self::List(l) => l.serialize(s),
//             Self::Null => s.serialize_none(),
//             Self::Number(n) => n.serialize(s),
//             Self::Object(m) => m.serialize(s),
//             Self::String(m) => m.serialize(s),
//         }
//     }
// }

impl ser::Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl ser::Serialize for Number {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Number::U8(v) => s.serialize_u8(v),
            Number::U16(v) => s.serialize_u16(v),
            Number::U32(v) => s.serialize_u32(v),
            Number::U64(v) => s.serialize_u64(v),
            Number::I8(v) => s.serialize_i8(v),
            Number::I16(v) => s.serialize_i16(v),
            Number::I32(v) => s.serialize_i32(v),
            Number::I64(v) => s.serialize_i64(v),
            Number::F32(v) => s.serialize_f32(v),
            Number::F64(v) => s.serialize_f64(v),
        }
    }
}

pub fn to_value<T: ser::Serialize>(value: T) -> Result<Value, SerializerError> {
    value.serialize(Serializer)
}

struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = Value;
    type Error = SerializerError;
    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.into()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.into()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bytes(v.to_vec().into()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(Serializer)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(variant.into()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(Serializer)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(Serializer).map(|v| {
            let mut map = Map::with_capacity(1);
            map.insert(variant.to_string(), v);
            map.into()
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq(vec![]))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple(vec![]))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct(vec![]))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant(
            variant.to_string(),
            Vec::with_capacity(len),
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: Map::with_capacity(len.unwrap_or_default()),
            key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct(Map::with_capacity(len)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant(
            variant.into(),
            Map::with_capacity(len),
        ))
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: fmt::Display,
    {
        Ok(Value::String(value.to_string().into()))
    }
}

struct SerializeSeq(Vec<Value>);

impl ser::SerializeSeq for SerializeSeq {
    type Ok = Value;
    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::List(self.0.into()))
    }
}

struct SerializeTuple(Vec<Value>);

impl ser::SerializeTuple for SerializeTuple {
    type Ok = Value;
    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::List(self.0.into()))
    }
}

struct SerializeTupleStruct(Vec<Value>);

impl ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = Value;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::List(self.0.into()))
    }
}

struct SerializeTupleVariant(StdString, Vec<Value>);

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.1.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = Map::<Value>::default();
        map.insert(self.0, Value::List(self.1.into()));
        Ok(Value::Map(map))
    }
}

struct SerializeMap {
    map: Map<Value>,
    key: Option<String>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = SerializerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = key.serialize(Serializer)?;

        let key_str = crate::convert::convert(key, Type::String)
            .map_err(|err| SerializerError::Custom(err.to_string()))?;
        // FIXME: Dont unwrap
        self.key = match key_str {
            Value::String(str) => Some(str),
            _ => return Err(SerializerError::Custom("invalid key".into())),
        };

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.map.insert(self.key.take().unwrap(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map.into())
    }
}

struct SerializeStruct(Map);

impl ser::SerializeStruct for SerializeStruct {
    type Ok = Value;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = key.to_string();
        let value = value.serialize(Serializer)?;
        self.0.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.0))
    }
}

struct SerializeStructVariant(String, Map);

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = key.to_string();
        let value = value.serialize(Serializer)?;
        self.1.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = Map::<Value>::with_capacity(1);
        map.insert(self.0, Value::Map(self.1));
        Ok(Value::Map(map))
    }
}

impl<V: ser::Serialize> ser::Serialize for Map<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in &self.inner {
            map.serialize_entry(&**k, v)?;
        }
        map.end()
    }
}

impl<V: serde::Serialize> ser::Serialize for List<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use ser::SerializeSeq;
        let mut map = serializer.serialize_seq(Some(self.len()))?;
        for v in &self.v {
            map.serialize_element(v)?;
        }
        map.end()
    }
}

// impl ser::Serialize for Type {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(&self.to_string())
//     }
// }
