use vaerdi::{convert::FromValue, kow::Kow, FromValue, IntoValue, Value};
// use worm_macros::{FromValue, IntoValue};

#[derive(Debug, IntoValue, FromValue)]
enum Flag {
    Flag1,
    Flag2,
}

#[derive(Debug, IntoValue, FromValue)]
enum IntFlag {
    Flag1 = 1,
    Flag2 = 2,
    Flag3 = 300,
}

#[derive(Debug, IntoValue, FromValue)]
enum Kind<'a> {
    Flag(Flag),
    Int(IntFlag),
    Struct(Struct<'a>),
}

#[derive(Debug, IntoValue, FromValue)]
pub struct Struct<'a> {
    name: Kow<'a, str>,
    age: u8,
}

#[derive(Debug, IntoValue, FromValue)]
pub struct Payload<'a> {
    kind: Kind<'a>,
    other: String,
}

fn main() {
    let payload = Payload {
        kind: Kind::Struct(Struct {
            name: "Rasmus".into(),
            age: 39,
        }),
        other: "Test".to_string(),
    };

    let payloadv: Value = payload.into();

    println!("{:#?}", vaerdi::json::to_string(&payloadv));

    let payload: Payload = Payload::from_value(payloadv.clone()).unwrap();

    println!("{:#?}", payload);

    println!(
        "Value {:?}",
        vaerdi::json::parse(&vaerdi::json::to_string(&payloadv))
    )
}
