use super::error::ConvertError;
use crate::{bytes::Bytes, string::String, List, Map, Number, Type, Value};
use alloc::{string::ToString, vec::Vec};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use core::convert::{Infallible, TryInto};
use uuid::Uuid;

pub trait FromValue: Sized {
    type Error: Into<ConvertError>;
    fn from_value(value: Value) -> Result<Self, Self::Error>;
}

impl FromValue for Value {
    type Error = Infallible;
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        Ok(value)
    }
}

macro_rules! from_impl {
    ($variant: ident => $type: ty, $method: ident, $as: ident, $as_mut: ident) => {
        impl FromValue for $type {
            type Error = ConvertError;
            fn from_value(from: Value) -> Result<Self, Self::Error> {
                match from.$method() {
                    Ok(s) => Ok(s),
                    Err(err) => Err(ConvertError::invalid_type(Type::$variant, err.get_type())),
                }
            }
        }

    };
    (@integer $($type: ty => $variant: ident),*) => {
        $(
            impl FromValue for $type {
                type Error = ConvertError;
                fn from_value(from: Value) -> Result<Self, Self::Error> {
                    match from.into_number() {
                        Ok(n) => match n {
                            Number::$variant(n) => Ok(n),
                            n => {
                                let n = n.as_u64().try_into().map_err(|_| ConvertError::invalid_type(Type::$variant, n.get_type()))?;
                                Ok(n)
                            }
                        },
                        Err(err) => Err(ConvertError::invalid_type(Type::$variant, err.get_type())),
                    }
                }
            }

        )*
    };

    (@float $($type: ty => $variant: ident),*) => {
        $(
            impl FromValue for $type {
                type Error = ConvertError;
                fn from_value(from: Value) -> Result<Self, Self::Error> {
                    match from.into_number() {
                        Ok(n) => match n {
                            Number::$variant(n) => Ok(n),
                            n => {
                                Err(ConvertError::invalid_type(Type::$variant, n.get_type()))
                            }
                        },
                        Err(err) => Err(ConvertError::invalid_type(Type::$variant, err.get_type())),
                    }
                }
            }

        )*
    };

}

from_impl!(String => String, into_string, as_string, as_string_mut);
from_impl!(Bytes => Bytes, into_bytes, as_bytes, as_bytes_mut);
from_impl!(Bool => bool, into_bool, as_bool, as_bool_mut);
from_impl!(Map => Map, into_map, as_map, as_map_mut);
from_impl!(List => List, into_list, as_list, as_list_mut);
from_impl!(Time => NaiveTime, into_time, as_time, as_time_mut);
from_impl!(DateTime => NaiveDateTime, into_datetime, as_datetime, as_datetime_mut);
from_impl!(Date => NaiveDate, into_date, as_date, as_date_mut);
from_impl!(Uuid => Uuid, into_uuid, as_uuid, as_uuid_mut);
// from_impl!(JsonValue, into_json, as_json, as_json_mut);

from_impl!(
    @integer
    u8 => U8,
    i8 => I8,
    u16 => U16,
    i16 => I16,
    u32 => U32,
    i32 => I32,
    u64 => U64,
    i64 => I64
);

from_impl!(
    @float

    f32 => F32,
    f64 => F64
);

// from_impl!(
//     @time
//     NaiveDate => as_date,
//     NaiveDateTime => as_datetime,
//     NaiveTime => as_time
// );

// impl<'a> TryFrom<&'a Value> for &'a str {
//     type Error = ConvertError;
//     fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
//         match from.as_string() {
//             Some(s) => Ok(s),
//             None => Err(ConvertError::invalid_type(Type::String, from.get_type())),
//         }
//     }
// }

// impl<'a> TryFrom<&'a Value> for &'a [u8] {
//     type Error = ConvertError;
//     fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
//         match from.as_bytes() {
//             Some(s) => Ok(s.as_ref()),
//             None => Err(ConvertError::invalid_type(Type::Bytes, from.get_type())),
//         }
//     }
// }

// impl<'a> TryFrom<&'a Value> for Value {
//     type Error = ConvertError;
//     fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
//         Ok(value.clone())
//     }
// }

impl<T> FromValue for Vec<T>
where
    T: FromValue,
{
    type Error = ConvertError;
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        let Value::List(list) = value else {
            return Err(ConvertError::invalid_type(Type::List, value.get_type()));
        };

        let ret = list
            .into_iter()
            .map(T::from_value)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)?;
        Ok(ret)
    }
}

impl FromValue for alloc::string::String {
    type Error = ConvertError;
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v.to_string()),
            _ => Err(ConvertError::invalid_type(Type::String, value.get_type())),
        }
    }
}

// impl<K, T, H> TryFrom<Value> for HashMap<K, T, H>
// where
//     K: From<String> + Eq + core::hash::Hash,
//     T: TryFrom<Value, Error = ConvertError>,
//     H: BuildHasher + Default,
// {
//     type Error = ConvertError;
//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         let Value::Map(map) = value else {
//             return Err(ConvertError::invalid_type(Type::Map, value.get_type()));
//         };

//         let ret = map
//             .into_iter()
//             .map(|(k, v)| Ok((k.into(), v.try_into()?)))
//             .collect::<Result<HashMap<_, _, H>, _>>()?;
//         Ok(ret)
//     }
// }

impl FromValue for Type {
    type Error = ConvertError;
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        let Some(name) = value.as_string() else {
            return Err(ConvertError::invalid_type(Type::String, value.get_type()));
        };

        let ret = match name.as_str() {
            "bool" => Type::Bool,
            "bytes" => Type::Bytes,
            "string" => Type::String,
            "i8" => Type::I8,
            "u8" => Type::U8,
            "i16" => Type::I16,
            "u16" => Type::U16,
            "i32" => Type::I32,
            "u32" => Type::U32,
            "i64" => Type::I64,
            "u64" => Type::U64,
            "f32" => Type::F32,
            "f64" => Type::F64,
            "date" => Type::Date,
            "datetime" => Type::DateTime,
            "time" => Type::Time,
            "uuid" => Type::Uuid,
            "json" => Type::Json,
            _ => {
                panic!("unknown {name}")
            }
        };

        Ok(ret)
    }
}

impl<T> FromValue for Option<T>
where
    T: FromValue,
{
    type Error = T::Error;
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            Ok(None)
        } else {
            T::from_value(value).map(Option::Some)
        }
    }
}
