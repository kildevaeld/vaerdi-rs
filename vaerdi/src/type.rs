use alloc::{fmt, string::ToString};
use bitflags::bitflags;

use crate::Value;

bitflags! {
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type: u32 {
    const String = 1 << 0;
    const Char = 1 << 1;
    const Bool = 1 << 2;
    const Bytes = 1 << 3;
    const Map = 1 << 4;
    const List = 1 << 5;
    const Date = 1 << 6;
    const DateTime = 1 << 7;
    const Time = 1 << 8;
    const Uuid = 1 << 9;
    const Json = 1 << 10;
    //
    const U8 = 1 << 11;
    const I8 = 1 << 12;
    const U16 = 1 << 13;
    const I16 = 1 << 14;
    const U32 = 1 << 15;
    const I32 = 1 << 16;
    const U64 = 1 << 17;
    const I64 = 1 << 18;
    const F32 = 1 << 19;
    const F64 = 1 << 20;

}
}

impl Type {
    pub fn int() -> Type {
        Type::U8 | Type::I8 | Type::U16 | Type::I16 | Type::U32 | Type::I32 | Type::U64 | Type::I64
    }

    pub fn float() -> Type {
        Type::F32 | Type::F64
    }

    pub fn number() -> Type {
        Type::int() | Type::float()
    }

    pub fn is_int(&self) -> bool {
        Type::int().contains(*self)
    }

    pub fn is_float(&self) -> bool {
        Type::float().contains(*self)
    }

    pub fn is_number(&self) -> bool {
        Type::number().contains(*self)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Type::Bool => f.write_str("bool"),
            Type::Bytes => f.write_str("bytes"),
            Type::Char => f.write_str("char"),
            Type::Date => f.write_str("date"),
            Type::DateTime => f.write_str("datetime"),
            Type::F32 => f.write_str("f32"),
            Type::F64 => f.write_str("f64"),
            Type::I16 => f.write_str("i16"),
            Type::I32 => f.write_str("i32"),
            Type::I64 => f.write_str("i64"),
            Type::I8 => f.write_str("i8"),
            Type::Json => f.write_str("json"),
            Type::List => f.write_str("list"),
            Type::Map => f.write_str("map"),
            Type::String => f.write_str("string"),
            Type::Time => f.write_str("time"),
            Type::U16 => f.write_str("u16"),
            Type::U32 => f.write_str("u32"),
            Type::U64 => f.write_str("u64"),
            Type::U8 => f.write_str("i8"),
            Type::Uuid => f.write_str("uuid"),
            v => {
                for (idx, t) in v.iter().enumerate() {
                    if idx > 0 {
                        write!(f, "|")?;
                    }
                    write!(f, "{}", t)?
                }

                Ok(())
            }
        }
    }
}

impl From<Type> for Value {
    fn from(value: Type) -> Self {
        Value::String(value.to_string().into())
    }
}
