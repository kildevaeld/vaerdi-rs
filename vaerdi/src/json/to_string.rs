use alloc::string::String;
use core::fmt::{Result, Write};

use crate::{List, Map, Value};

pub fn to_string(value: &Value) -> String {
    let mut out = String::new();
    display(value, &mut out).expect("should not fail");
    out
}

pub fn display<W: Write>(value: &Value, output: &mut W) -> Result {
    match value {
        Value::Bool(b) => write!(output, "{b}"),
        Value::String(s) => write!(output, "\"{s}\""),
        Value::Map(m) => display_object(m, output),
        Value::List(m) => display_list(m, output),
        Value::Bytes(_) => todo!(),
        Value::Date(_) => todo!(),
        Value::DateTime(time) => {
            write!(output, "{}", time.and_utc().to_rfc2822())
        }
        Value::Time(_) => todo!(),
        Value::Uuid(id) => {
            write!(output, "\"{}\"", id.as_hyphenated())
        }
        Value::Number(n) => {
            write!(output, "{n}")
        }
        Value::Char(c) => write!(output, "\"{c}\""),
        Value::Null => output.write_str("null"),
    }
}

pub fn display_list<W: Write>(value: &List, output: &mut W) -> Result {
    output.write_char('[')?;

    for (idx, key) in value.iter().enumerate() {
        if idx > 0 {
            output.write_char(',')?;
        }

        display(key, output)?
    }

    output.write_char(']')?;

    Ok(())
}

pub fn display_object<W: Write>(value: &Map, output: &mut W) -> Result {
    output.write_char('{')?;

    for (idx, (key, value)) in value.iter().enumerate() {
        if idx > 0 {
            output.write_char(',')?;
        }

        write!(output, "\"{key}\":")?;

        display(value, output)?
    }

    output.write_char('}')?;

    Ok(())
}
