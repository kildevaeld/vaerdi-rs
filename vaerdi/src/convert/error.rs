use crate::{number::TryFromNumberError, Type};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use avagarden::error::BoxError;
use core::{
    convert::Infallible,
    fmt,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub enum ConvertErrorKind {
    Type { expected: Type, found: Type },
    UnknownVariant { name: String },
    Unknown(BoxError<'static>),
    Infallible,
}

impl fmt::Display for ConvertErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infallible => write!(f, "infallible"),
            Self::Type { expected, found } => write!(f, "expected: {expected}, found: {found}"),
            Self::UnknownVariant { name } => write!(f, "unknown variant: {name}"),
            Self::Unknown(err) => write!(f, "{err}"),
        }
    }
}

#[derive(Debug)]
pub struct ConvertError {
    kind: ConvertErrorKind,
    #[allow(unused)]
    context: Option<String>,
}

impl ConvertError {
    pub fn invalid_type(expected: Type, found: Type) -> ConvertError {
        ConvertError {
            kind: ConvertErrorKind::Type { expected, found },
            context: None,
        }
    }

    pub fn unknown<S>(error: S) -> ConvertError
    where
        S: Into<BoxError<'static>>, // S: worm_shared::Error + Send + Sync + 'static,
    {
        ConvertError {
            kind: ConvertErrorKind::Unknown(error.into()),
            context: None,
        }
    }

    pub fn unknown_variant(name: impl ToString) -> ConvertError {
        ConvertError {
            kind: ConvertErrorKind::UnknownVariant {
                name: name.to_string(),
            },
            context: None,
        }
    }

    pub fn with_context(mut self, ctx: impl ToString) -> ConvertError {
        self.context = Some(ctx.to_string());
        self
    }
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ctx) = &self.context {
            write!(f, "{}: ", ctx)?;
        }

        self.kind.fmt(f)
    }
}

impl avagarden::error::Error for ConvertError {}

impl From<Infallible> for ConvertError {
    fn from(_: Infallible) -> Self {
        ConvertError {
            kind: ConvertErrorKind::Infallible,
            context: None,
        }
    }
}

#[cfg(feature = "gerning")]
impl From<ConvertError> for gerning::arguments::ArgumentError<crate::Value> {
    fn from(value: ConvertError) -> Self {
        match value.kind {
            ConvertErrorKind::Infallible => gerning::arguments::ArgumentError::Infallible,
            ConvertErrorKind::Type { expected, found } => {
                gerning::arguments::ArgumentError::IvalidType { expected, found }
            }
            ConvertErrorKind::Unknown(_err) => {
                unimplemented!("cannot be represented as an argument error")
            }
            ConvertErrorKind::UnknownVariant { .. } => {
                unimplemented!("cannot be represented as an argument error")
            }
        }
    }
}

impl From<ConvertError> for ConvertErrorKind {
    fn from(value: ConvertError) -> Self {
        value.kind
    }
}

impl From<TryFromNumberError> for ConvertError {
    fn from(value: TryFromNumberError) -> Self {
        ConvertError {
            kind: ConvertErrorKind::Unknown(Box::new(value)),
            context: None,
        }
    }
}

impl From<ParseIntError> for ConvertError {
    fn from(value: ParseIntError) -> Self {
        ConvertError::unknown(value.to_string())
    }
}

impl From<ParseFloatError> for ConvertError {
    fn from(value: ParseFloatError) -> Self {
        ConvertError::unknown(value.to_string())
    }
}
