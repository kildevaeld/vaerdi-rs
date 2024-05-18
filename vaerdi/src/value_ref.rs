use core::fmt;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;

use crate::{bytes::Bytes, List, Map, Number, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueRef<'a> {
    Bool(bool),
    String(&'a str),
    Map(&'a Map),
    List(&'a List),
    Bytes(&'a Bytes),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Time(NaiveTime),
    Uuid(Uuid),
    Number(Number),
    Char(char),
    // Json(&'a JsonValue),
    Null,
}

impl<'a> From<&'a Value> for ValueRef<'a> {
    fn from(value: &'a Value) -> Self {
        match value {
            Value::Bool(i) => ValueRef::Bool(*i),
            Value::String(a) => ValueRef::String(a),
            Value::Map(m) => ValueRef::Map(m),
            Value::List(m) => ValueRef::List(m),
            Value::Bytes(b) => ValueRef::Bytes(b),
            Value::Date(m) => ValueRef::Date(*m),
            Value::DateTime(m) => ValueRef::DateTime(*m),
            Value::Time(m) => ValueRef::Time(*m),
            Value::Uuid(m) => ValueRef::Uuid(*m),
            Value::Number(n) => ValueRef::Number(*n),
            Value::Char(c) => ValueRef::Char(*c),
            // Value::Json(j) => ValueRef::Json(j),
            Value::Null => ValueRef::Null,
        }
    }
}

impl<'a> From<ValueRef<'a>> for Value {
    fn from(value: ValueRef<'a>) -> Self {
        match value {
            ValueRef::Bool(i) => Value::Bool(i),
            ValueRef::String(a) => Value::String(a.into()),
            ValueRef::Map(m) => Value::Map(m.clone()),
            ValueRef::List(m) => Value::List(m.clone()),
            ValueRef::Bytes(b) => Value::Bytes(b.clone()),
            ValueRef::Date(m) => Value::Date(m),
            ValueRef::DateTime(m) => Value::DateTime(m),
            ValueRef::Time(m) => Value::Time(m),
            ValueRef::Uuid(m) => Value::Uuid(m),
            ValueRef::Number(n) => Value::Number(n),
            ValueRef::Char(c) => Value::Char(c),
            // ValueRef::Json(j) => Value::Json(j.clone()),
            ValueRef::Null => Value::Null,
        }
    }
}

#[cfg(feature = "gerning")]
impl<'a> gerning::Value for ValueRef<'a> {
    type Type = crate::Type;

    fn get_type(&self) -> Self::Type {
        use crate::Type;
        match self {
            ValueRef::Bool(_) => Type::Bool,
            ValueRef::String(_) => Type::String,
            ValueRef::Map(_) => Type::Map,
            ValueRef::List(_) => Type::List,
            ValueRef::Bytes(_) => Type::Bytes,
            ValueRef::Date(_) => Type::Date,
            ValueRef::DateTime(_) => Type::DateTime,
            ValueRef::Time(_) => Type::Time,
            ValueRef::Uuid(_) => Type::Uuid,
            ValueRef::Number(_) => todo!(),
            ValueRef::Char(_) => Type::Char,
            // ValueRef::Json(_) => Type::Json,
            ValueRef::Null => Type::all(),
        }
    }
}

impl<'a> fmt::Display for ValueRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueRef::Bool(v) => v.fmt(f),
            ValueRef::String(v) => v.fmt(f),
            ValueRef::Map(v) => v.fmt(f),
            ValueRef::List(v) => v.fmt(f),
            ValueRef::Bytes(v) => write!(f, "<bytes {}>", v.len()),
            ValueRef::Date(v) => v.fmt(f),
            ValueRef::DateTime(v) => v.fmt(f),
            ValueRef::Time(v) => v.fmt(f),
            ValueRef::Uuid(v) => v.fmt(f),
            ValueRef::Number(v) => v.fmt(f),
            ValueRef::Char(v) => v.fmt(f),
            ValueRef::Null => write!(f, "null"),
        }
    }
}

pub trait AsValueRef {
    fn as_value_ref<'a>(&'a self) -> ValueRef<'a>;
}

impl<'c, T> AsValueRef for &'c T
where
    T: AsValueRef,
{
    fn as_value_ref<'a>(&'a self) -> ValueRef<'a> {
        (&**self).as_value_ref()
    }
}

impl<'c> AsValueRef for ValueRef<'c> {
    fn as_value_ref<'a>(&'a self) -> ValueRef<'a> {
        *self
    }
}

impl AsValueRef for Value {
    fn as_value_ref<'a>(&'a self) -> ValueRef<'a> {
        self.into()
    }
}
