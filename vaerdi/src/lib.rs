#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod macros;

mod bytes;
pub mod convert;
mod floating;

#[cfg(feature = "json")]
pub mod json;
pub mod kow;
mod list;
mod map;
mod merge;
mod number;
mod string;
mod r#type;
mod value;
mod value_ref;

pub mod interning;

pub use self::{
    convert::ConvertError, list::*, map::*, merge::merge, number::Number, r#type::*, string::*,
    value::*, value_ref::*,
};

pub use ::{
    chrono::{self, NaiveDate, NaiveDateTime, NaiveTime},
    hashbrown,
    uuid::{self, Uuid},
};

#[cfg(feature = "macros")]
pub use vaerdi_macros::*;

#[cfg(feature = "serde")]
pub mod de;
#[cfg(feature = "serde")]
pub mod ser;
