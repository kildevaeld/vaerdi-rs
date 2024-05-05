use crate::{
    bytes::Bytes, number::Number, string::String, ConvertError, List, Map, Type, ValueRef,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use core::{fmt, iter::FromIterator};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    String(String),
    Map(Map),
    List(List),
    Bytes(Bytes),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Time(NaiveTime),
    Uuid(Uuid),
    Number(Number),
    Char(char),
    Null,
}

#[cfg(feature = "gerning")]
impl gerning::Value for Value {
    type Type = Type;

    fn get_type(&self) -> Self::Type {
        self.get_type()
    }
}

macro_rules! is_method {
    ($check: ident, $ty: ident) => {
        pub fn $check(&self) -> bool {
            match self {
                Value::$ty(_) => true,
                _ => false,
            }
        }
    };
}

macro_rules! into_method {
    ($into: ident, $ty: ident, $oty: ty) => {
        pub fn $into(self) -> Result<$oty, Value> {
            match self {
                Value::$ty(v) => Ok(v),
                _ => Err(self),
            }
        }
    };
}

macro_rules! as_method {
    ($as: ident, $as_mut: ident, $ty: ident, $oty: ty) => {
        pub fn $as(&self) -> Option<&$oty> {
            match &self {
                Value::$ty(v) => Some(v),
                _ => None,
            }
        }

        pub fn $as_mut(&mut self) -> Option<&mut $oty> {
            match self {
                Value::$ty(v) => Some(v),
                _ => None,
            }
        }
    };
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Map(_) => Type::Map,
            Value::List(_) => Type::List,
            Value::Bytes(_) => Type::Bytes,
            Value::Date(_) => Type::Date,
            Value::DateTime(_) => Type::DateTime,
            Value::Time(_) => Type::Time,
            Value::Uuid(_) => Type::Uuid,
            Value::Number(n) => n.get_type(),
            Value::Char(_) => Type::Char,
            Value::Null => Type::all(),
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    is_method!(is_string, String);
    is_method!(is_bytes, Bytes);
    is_method!(is_bool, Bool);
    is_method!(is_list, List);
    is_method!(is_map, Map);
    is_method!(is_char, Char);
    is_method!(is_time, Time);
    is_method!(is_date, Date);
    is_method!(is_datetime, DateTime);
    is_method!(is_uuid, Uuid);

    as_method!(as_number, as_number_mut, Number, Number);
    as_method!(as_string, as_string_mut, String, String);
    as_method!(as_bytes, as_bytes_mut, Bytes, Bytes);
    as_method!(as_bool, as_bool_mut, Bool, bool);
    as_method!(as_list, as_list_mut, List, List);
    as_method!(as_map, as_map_mut, Map, Map);
    as_method!(as_char, as_char_mut, Char, char);
    as_method!(as_time, as_time_mut, Time, NaiveTime);
    as_method!(as_datetime, as_datetime_mut, DateTime, NaiveDateTime);
    as_method!(as_date, as_date_mut, Date, NaiveDate);
    as_method!(as_uuid, as_uuid_mut, Uuid, Uuid);

    into_method!(into_string, String, String);
    into_method!(into_bytes, Bytes, Bytes);
    into_method!(into_bool, Bool, bool);
    into_method!(into_list, List, List);
    into_method!(into_map, Map, Map);
    into_method!(into_char, Char, char);
    into_method!(into_number, Number, Number);
    into_method!(into_time, Time, NaiveTime);
    into_method!(into_datetime, DateTime, NaiveDateTime);
    into_method!(into_date, Date, NaiveDate);
    into_method!(into_uuid, Uuid, Uuid);

    pub fn as_ref(&self) -> ValueRef<'_> {
        self.into()
    }

    pub fn convert_to(self, ty: Type) -> Result<Value, ConvertError> {
        crate::convert::convert(self, ty)
    }

    pub fn remove<S: AsRef<str>>(&mut self, field: S) -> Option<Value> {
        match self.as_map_mut() {
            Some(map) => map.remove(field),
            None => None,
        }
    }

    pub fn get<S: AsRef<str>>(&self, field: S) -> Option<&Value> {
        match self.as_map() {
            Some(map) => map.get(field),
            None => None,
        }
    }

    pub fn get_mut<S: AsRef<str>>(&mut self, field: S) -> Option<&mut Value> {
        match self.as_map_mut() {
            Some(map) => map.get_mut(field),
            None => None,
        }
    }

    pub fn insert<S: AsRef<str>, V: Into<Value>>(&mut self, field: S, value: V) -> Option<Value> {
        match self.as_map_mut() {
            Some(map) => map.insert(field.as_ref(), value.into()),
            None => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => v.fmt(f),
            Value::String(v) => v.fmt(f),
            Value::Map(v) => v.fmt(f),
            Value::List(v) => v.fmt(f),
            Value::Bytes(v) => write!(f, "<bytes {}>", v.len()),
            Value::Date(v) => v.fmt(f),
            Value::DateTime(v) => v.fmt(f),
            Value::Time(v) => v.fmt(f),
            Value::Uuid(v) => v.fmt(f),
            Value::Number(v) => v.fmt(f),
            Value::Char(v) => v.fmt(f),
            Value::Null => write!(f, "null"),
        }
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        self
    }
}

impl AsMut<Value> for Value {
    fn as_mut(&mut self) -> &mut Value {
        self
    }
}

impl FromIterator<(String, Value)> for Value {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Value::Map(Map::from_iter(iter))
    }
}

impl<V: Into<Value>> FromIterator<V> for Value {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Value::List(List::from_iter(iter.into_iter().map(Into::into)))
    }
}
