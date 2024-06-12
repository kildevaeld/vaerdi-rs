use alloc::string::String;
use chrono::DateTime;
use udled::{
    token::{Bool, Int, Opt, Str, Ws},
    Input, Tokenizer,
};

use crate::{List, Map, Value};

const WS: Opt<Ws> = Opt(Ws);

pub fn parse(input: &str) -> Result<Value, udled::Error> {
    let mut input = udled::Input::new(input);

    // input.parse(WS)?;

    parse_value(&mut input)
}

fn parse_value(input: &mut Input<'_>) -> Result<Value, udled::Error> {
    let Some(ch) = input.peek_ch() else {
        return Err(input.error("unexpected eof"));
    };

    match ch {
        "{" => parse_object(input),
        "[" => parse_list(input),
        "t" | "f" => {
            let bool = input.parse(Bool)?;
            Ok(Value::Bool(bool.value))
        }
        "n" => {
            let _ = input.parse("null")?;
            Ok(Value::Null)
        }
        "\"" => input.parse(JsonStringValue),
        _ => input.parse(JsonNumber),
    }
}

fn parse_object(input: &mut Input<'_>) -> Result<Value, udled::Error> {
    let _ = input.parse((WS, '{'))?;

    let mut map = Map::default();

    let _ = input.parse(WS)?;

    if input.peek('}')? {
        input.eat('}')?;
        return Ok(Value::Map(map));
    }

    loop {
        if input.eos() {
            return Err(input.error("unexpected eof"));
        }

        let prop = input.parse(JsonString)?;
        let _ = input.parse((WS, ':', WS))?;

        let value = parse_value(input)?;

        map.insert(prop, value);

        let _ = input.parse(WS)?;

        if input.peek('}')? {
            input.parse('}')?;
            break;
        }

        let _ = input.parse((WS, ','))?;

        let _ = input.parse(WS)?;
    }

    Ok(Value::Map(map))
}

fn parse_list(input: &mut Input<'_>) -> Result<Value, udled::Error> {
    let _ = input.parse((WS, '['))?;

    let mut map = List::default();

    loop {
        if input.eos() {
            return Err(input.error("unexpected eof"));
        }

        let _ = input.parse(WS)?;

        if input.peek(']')? {
            input.parse(']')?;
            break;
        }

        let value = parse_value(input)?;

        map.push(value);

        input.eat(WS)?;

        if input.peek(']')? {
            input.parse(']')?;
            break;
        }

        let _ = input.parse((WS, ','))?;
    }

    Ok(Value::List(map))
}

struct JsonNumber;

impl Tokenizer for JsonNumber {
    type Token<'a> = Value;
    fn to_token<'a>(
        &self,
        reader: &mut udled::Reader<'_, 'a>,
    ) -> Result<Self::Token<'a>, udled::Error> {
        let int = reader.parse(Int)?;
        Ok(Value::Number((int.value as i64).into()))
    }
}

struct JsonString;

impl Tokenizer for JsonString {
    type Token<'a> = String;
    // TODO: Impl json string parsing
    fn to_token<'a>(
        &self,
        reader: &mut udled::Reader<'_, 'a>,
    ) -> Result<Self::Token<'a>, udled::Error> {
        let output = reader.parse(Str)?;

        Ok(output.as_str().into())
    }

    fn peek<'a>(&self, reader: &mut udled::Reader<'_, '_>) -> Result<bool, udled::Error> {
        reader.peek('"')
    }
}

struct JsonStringValue;

impl Tokenizer for JsonStringValue {
    type Token<'a> = Value;
    // TODO: Impl json string parsing
    fn to_token<'a>(
        &self,
        reader: &mut udled::Reader<'_, 'a>,
    ) -> Result<Self::Token<'a>, udled::Error> {
        let output = reader.parse(Str)?;

        // Uuid?
        if output.value.len() == 36 {
            if let Ok(uuid) = uuid::Uuid::parse_str(output.value) {
                return Ok(Value::Uuid(uuid));
            }
        }
        if let Ok(date) = DateTime::parse_from_rfc3339(&output.value) {
            return Ok(Value::DateTime(date.naive_utc()));
        }

        Ok(output.as_str().into())
    }

    fn peek<'a>(&self, reader: &mut udled::Reader<'_, '_>) -> Result<bool, udled::Error> {
        reader.peek('"')
    }
}
