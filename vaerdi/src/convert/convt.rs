use alloc::{
    string::{String, ToString},
    vec,
};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use core::convert::TryInto;
use uuid::Uuid;

use super::ConvertError;
use crate::{Number, Type, Value};

macro_rules! convert_n {
    ($to: expr, $value: expr, $($ident: ident),*) => {
        match $to {
           $(
            Type::$ident => {
                Number::$ident($value.try_into()?)
            }
           ),*
            _ => {
                unreachable!()
            }
        }
    };
}

macro_rules! parse_n {
    ($to: expr, $value: expr, $($ident: ident),*) => {
        match $to {
           $(
            Type::$ident => {
                Number::$ident($value.parse()?)
            }
           ),*
            _ => {
                unreachable!()
            }
        }
    };
}

fn convert_number(value: Number, to: Type) -> Result<Number, ConvertError> {
    if to.is_number() {
        let number = convert_n!(to, value, U8, I8, U16, I16, U32, I32, U64, I64, F32, F64);
        Ok(number)
    } else {
        todo!("cannot convert")
    }
}

pub(crate) fn parse_number(value: &str, to: Type) -> Result<Number, ConvertError> {
    if to.is_number() {
        let number = parse_n!(to, value, U8, I8, U16, I16, U32, I32, U64, I64, F32, F64);
        Ok(number)
    } else {
        todo!("cannot convert")
    }
}

pub fn convert(value: Value, to: Type) -> Result<Value, ConvertError> {
    if value.get_type() == to {
        return Ok(value);
    }

    let out = match value {
        Value::Bool(b) => {
            //
            if to == Type::Bool {
                Value::Bool(b)
            } else if to == Type::String {
                Value::String(if b { "true" } else { "false" }.into())
            } else if to.is_number() {
                Value::Number(convert_number(if b { 1u8 } else { 0u8 }.into(), to)?)
            } else {
                return Err(ConvertError::invalid_type(Type::Bool, to));
            }
        }
        Value::Bytes(b) => {
            //
            match to {
                Type::Bytes => Value::Bytes(b),
                Type::String => {
                    let str = String::from_utf8(b.to_vec())
                        .map_err(|err| ConvertError::unknown(err.to_string()))?;
                    Value::String(str.into())
                }
                _ => return Err(ConvertError::invalid_type(to, Type::Bytes)),
            }
        }
        Value::Char(c) => match to {
            Type::Bytes => Value::Bytes(vec![c as u8].into()),
            Type::String => Value::String(c.to_string().into()),
            _ => {
                if to.is_int() {
                    Value::Number((c as u32).try_into()?)
                } else {
                    return Err(ConvertError::invalid_type(to, Type::Char));
                }
            }
        },
        Value::Uuid(u) => match to {
            Type::Bytes => Value::Bytes(u.as_bytes().to_vec().into()),
            Type::String => Value::String(u.to_string().into()),
            _ => {
                return Err(ConvertError::invalid_type(to, Type::Uuid));
            }
        },
        Value::String(s) => match to {
            Type::Bytes => Value::Bytes(s.as_bytes().into()),
            Type::Date => {
                let date = NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                    .map_err(|err| ConvertError::unknown(err.to_string()))?;
                Value::Date(date)
            }
            Type::DateTime => {
                let datetime = DateTime::parse_from_rfc3339(&s)
                    .map_err(|err| ConvertError::unknown(err.to_string()))?;
                Value::DateTime(datetime.naive_utc())
            }
            Type::Time => {
                let time = NaiveTime::parse_from_str(&s, "%H:%M:S")
                    .map_err(|err| ConvertError::unknown(err.to_string()))?;
                Value::Time(time)
            }
            Type::Uuid => {
                let uuid =
                    Uuid::parse_str(&s).map_err(|err| ConvertError::unknown(err.to_string()))?;
                Value::Uuid(uuid)
            }
            _ => {
                if to.is_number() {
                    Value::Number(parse_number(&s, to)?)
                } else {
                    return Err(ConvertError::invalid_type(Type::Bytes, to));
                }
            }
        },
        Value::Number(n) => match to {
            Type::Bool => {
                if n > Number::I32(0) {
                    Value::Bool(true)
                } else {
                    Value::Bool(false)
                }
            }
            Type::DateTime if n.is_i64() => {
                let Some(ms) = DateTime::<Utc>::from_timestamp_millis(n.as_i64()) else {
                    return Err(ConvertError::unknown("invalid timestamp"));
                };

                Value::DateTime(ms.naive_utc())
            }
            Type::String => Value::String(n.into()),
            _ => {
                if to.is_number() {
                    Value::Number(convert_number(n, to)?)
                } else {
                    return Err(ConvertError::invalid_type(to, n.get_type()));
                }
            }
        },
        _ => return Err(ConvertError::invalid_type(Type::Bytes, to)),
    };

    Ok(out)
}
