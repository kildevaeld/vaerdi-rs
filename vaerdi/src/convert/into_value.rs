#[cfg(feature = "gerning")]
use crate::Type;
use crate::{kow::Kow, number::Number, string::String, value_ref::ValueRef, List, Map, Value};
use alloc::{
    borrow::Cow,
    boxed::Box,
    string::{String as StdString, ToString},
    sync::Arc,
    vec::Vec,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;

macro_rules! into_value {
    ($($ty: ty => $val: ident),*) => {
        $(
            impl From<$ty> for Value {
                fn from(from: $ty) -> Value {
                    Value::$val(from.into())
                }
            }

            impl<'a> From<&'a $ty> for Value {
                fn from(from: &'a $ty) -> Value {
                    Value::$val(from.clone().into())
                }
            }


            #[cfg(feature = "gerning")]
            impl gerning::Typed<Value> for $ty {
                fn get_type() -> Type {
                    Type::$val
                }
            }
        )*
    };
    (@number $($ty: ty),*) => {
        $(
            impl From<$ty> for Value {
                fn from(from: $ty) -> Value {
                    Value::Number(from.into())
                }
            }

            impl<'a> From<$ty> for ValueRef<'a> {
                fn from(from: $ty) -> Self {
                    ValueRef::Number(from.into())
                }
            }

            impl<'a> From<&'a $ty> for Value {
                fn from(from: &'a $ty) -> Value {
                    Value::Number((*from).into())
                }
            }

            impl<'a> From<&'a $ty> for ValueRef<'a> {
                fn from(from: &'a $ty) -> Self {
                    ValueRef::Number((*from).into())
                }
            }


        )*
    };

}

into_value!(
    StdString => String,
    String => String,
    bool => Bool,
    List => List,
    Map => Map,
    // HashMap<String, Value> => Map,
    NaiveDate => Date,
    NaiveDateTime => DateTime,
    NaiveTime => Time,
    Uuid => Uuid
);

into_value!(@number i8, u8, i16, u16, i32, u32, i64, u64, usize, f32, f64);

impl<'a> From<&'a str> for Value {
    fn from(from: &'a str) -> Value {
        Value::String(from.into())
    }
}

impl<'a> From<&'a str> for ValueRef<'a> {
    fn from(from: &'a str) -> Self {
        ValueRef::String(from)
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    fn from(value: Cow<'a, str>) -> Self {
        Value::String(value.into())
    }
}

impl<'a> From<Kow<'a, str>> for Value {
    fn from(value: Kow<'a, str>) -> Self {
        Value::String(value.into())
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        Value::List(value.into())
    }
}

// impl<K: ToString, V: Into<Value>> From<HashMap<K, V>> for Value {
//     fn from(value: HashMap<K, V>) -> Self {
//         Value::List(value.into())
//     }
// }

impl<V: Into<Value>> From<Option<V>> for Value {
    fn from(value: Option<V>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

// impl From<Value> for Option<Value> {
//     fn from(value: Value) -> Self {
//         if value.is_null() {
//             None
//         } else {
//             Some(value)
//         }
//     }
// }

// impl<'a> From<&'a [u8]> for Value {
//     fn from(value: &'a [u8]) -> Self {
//         Value::Bytes(value.to_vec().into())
//     }
// }

#[cfg(feature = "gerning")]
impl<T> gerning::Typed<Value> for Vec<T> {
    fn get_type() -> Type {
        Type::List
    }
}

impl From<Number> for Value {
    fn from(from: Number) -> Value {
        Value::Number(from)
    }
}

impl<'a> From<&'a Number> for Value {
    fn from(from: &'a Number) -> Value {
        Value::Number(*from)
    }
}

#[cfg(feature = "gerning")]
impl gerning::Typed<Value> for Number {
    fn get_type() -> crate::Type {
        Type::I8 | Type::U8 | Type::I16 | Type::U16 | Type::I32 | Type::U32 | Type::I64 | Type::U64
    }
}

impl From<Number> for String {
    fn from(value: Number) -> Self {
        value.to_string().into()
    }
}

impl<T> From<Box<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Box<T>) -> Self {
        let v = *value;
        <T as Into<Self>>::into(v)
    }
}

impl<T> From<Box<[T]>> for Value
where
    T: Into<Value>,
{
    fn from(value: Box<[T]>) -> Self {
        let v: Vec<_> = value.into();
        v.into()
    }
}

impl<T> From<Arc<T>> for Value
where
    T: Into<Value> + Clone,
{
    fn from(value: Arc<T>) -> Self {
        let v = value.as_ref().clone();
        v.into()
    }
}

impl<T> From<Arc<[T]>> for Value
where
    T: Into<Value> + Clone,
{
    fn from(value: Arc<[T]>) -> Self {
        let v: Vec<_> = value.as_ref().to_vec();
        v.into()
    }
}
