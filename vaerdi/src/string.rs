use alloc::{borrow::Cow, boxed::Box, string::ToString, sync::Arc};
use avagarden::error::BoxError;
use core::{borrow::Borrow, fmt};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String(Arc<str>);

impl String {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> PartialEq<&'a str> for String {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl core::ops::Deref for String {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<alloc::string::String> for String {
    fn from(value: alloc::string::String) -> Self {
        String(Arc::from(value))
    }
}

impl<'a> From<&'a alloc::string::String> for String {
    fn from(value: &'a alloc::string::String) -> Self {
        String(Arc::from(value.as_str()))
    }
}

impl<'a> From<&'a str> for String {
    fn from(value: &'a str) -> Self {
        String(Arc::from(value))
    }
}

impl From<String> for alloc::string::String {
    fn from(value: String) -> Self {
        value.to_string()
    }
}

impl<'a> From<Cow<'a, str>> for String {
    fn from(value: Cow<'a, str>) -> Self {
        value.as_ref().into()
    }
}

impl Borrow<str> for String {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

pub fn format(args: core::fmt::Arguments<'_>) -> String {
    String::from(alloc::fmt::format(args))
}

#[macro_export]
macro_rules! format {
    ($($toks:tt)*) => {
        $crate::format(core::format_args!($($toks)*))
    };
}

impl<'a> From<String> for BoxError<'a> {
    fn from(value: String) -> BoxError<'a> {
        struct StringError(String);

        impl fmt::Display for StringError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl fmt::Debug for StringError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("StringError").field("", &self.0).finish()
            }
        }

        impl avagarden::error::Error for StringError {}

        Box::new(StringError(value))
    }
}
