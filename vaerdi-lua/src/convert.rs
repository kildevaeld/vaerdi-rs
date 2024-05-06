use vaerdi::{List, Map, Value};

pub fn from_lua(value: mlua::Value<'_>) -> mlua::Result<vaerdi::Value> {
    let val = match value {
        mlua::Value::Boolean(b) => Value::Bool(b),
        mlua::Value::Integer(i) => Value::Number(i.into()),
        mlua::Value::Number(i) => Value::Number(i.into()),
        mlua::Value::String(s) => Value::String(s.to_str()?.into()),
        mlua::Value::Table(table) => {
            let len = table.raw_len();
            if len > 0 {
                let mut list = List::with_capacity(table.len()? as usize);
                for item in table.sequence_values::<mlua::Value>() {
                    let item = item?;
                    list.push(from_lua(item)?);
                }
                Value::List(list.into())
            } else {
                let mut map = Map::with_capacity(table.len()? as usize);
                for pair in table.pairs::<mlua::String, mlua::Value>() {
                    let (k, v) = pair?;
                    map.insert(k.to_str()?, from_lua(v)?);
                }
                Value::Map(map)
            }
        }
        mlua::Value::Nil => Value::Null,
        _ => return Err(mlua::Error::external("unsupported lua type")),
    };

    Ok(val)
}

pub fn into_lua<'lua>(
    vm: &'lua mlua::Lua,
    value: &Value,
    raw: bool,
) -> mlua::Result<mlua::Value<'lua>> {
    match value {
        Value::Bool(b) => mlua::Value::Boolean(*b),
        Value::String(s) => mlua::Value::String(vm.create_string(&**s)?),
        Value::Map(m) => {
            let table = vm.create_table()?;

            for (k, v) in m {
                table.set(&**k, into_lua(vm, v, raw)?)?;
            }

            mlua::Value::Table(table)
        }
        Value::List(m) => {
            let table = vm.create_table_with_capacity(m.len(), 0)?;

            for v in m {
                table.push(into_lua(vm, v, raw)?)?;
            }

            mlua::Value::Table(table)
        }
        Value::Bytes(i) => mlua::Value::String(vm.create_string(i)?),
        Value::Date(i) => mlua::Value::String(vm.create_string(i.to_string())?),
        Value::DateTime(i) => mlua::Value::String(vm.create_string(i.to_string())?),
        Value::Time(i) => mlua::Value::String(vm.create_string(i.to_string())?),
        Value::Uuid(i) => mlua::Value::String(vm.create_string(i.hyphenated().to_string())?),
        Value::Number(n) => {
            if n.is_float() {
                mlua::Value::Number(n.as_f64().into())
            } else {
                mlua::Value::Integer(n.as_i64().into())
            }
        }
        Value::Char(c) => mlua::Value::Integer(*c as i64),
        Value::Null => mlua::Value::Nil,
    };

    todo!()
}
