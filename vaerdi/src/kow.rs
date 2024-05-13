use crate::{bytes::Bytes, String};
use core::{
    borrow::Borrow,
    fmt::{self, Display},
    hash::Hash,
};

/// Like ToOwned
pub trait ToKowned {
    type Owned;
    fn to_owned(&self) -> Self::Owned;
}

impl<'a, T> ToKowned for &'a T
where
    T: ToKowned,
{
    type Owned = T::Owned;
    fn to_owned(&self) -> Self::Owned {
        (**self).to_owned()
    }
}

impl ToKowned for str {
    type Owned = String;
    fn to_owned(&self) -> Self::Owned {
        self.into()
    }
}

impl ToKowned for String {
    type Owned = String;
    fn to_owned(&self) -> Self::Owned {
        self.clone()
    }
}

/// Like Cow in std
#[derive(Debug)]
pub enum Kow<'a, S: ToKowned + ?Sized + 'a> {
    Owned(S::Owned),
    Ref(&'a S),
}

impl<'a, S> Kow<'a, S>
where
    S: ToKowned,
{
    pub fn into_owned(self) -> S::Owned {
        match self {
            Self::Owned(o) => o,
            Self::Ref(o) => o.to_owned(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, 'a, T> serde::de::Deserialize<'de> for Kow<'a, T>
where
    T: ?Sized + ToKowned,
    T::Owned: serde::de::Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        T::Owned::deserialize(deserializer).map(Kow::Owned)
    }
}

#[cfg(feature = "serde")]
impl<'a, T> serde::ser::Serialize for Kow<'a, T>
where
    T: ?Sized + ToKowned + serde::ser::Serialize,
    T::Owned: Borrow<T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<'a, S> AsRef<S> for Kow<'a, S>
where
    S: ToKowned + ?Sized,
    S::Owned: Borrow<S>,
{
    fn as_ref(&self) -> &S {
        match self {
            Kow::Owned(o) => o.borrow(),
            Kow::Ref(s) => *s,
        }
    }
}

impl<'a, S> core::ops::Deref for Kow<'a, S>
where
    S: ToKowned + ?Sized,
    S::Owned: Borrow<S>,
{
    type Target = S;
    fn deref(&self) -> &Self::Target {
        match self {
            Kow::Owned(o) => o.borrow(),
            Kow::Ref(s) => *s,
        }
    }
}

impl<S> Clone for Kow<'_, S>
where
    S: ToKowned + ?Sized,
    S::Owned: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Owned(o) => Self::Owned(o.clone()),
            Self::Ref(e) => Self::Ref(e),
        }
    }
}

impl<S> Copy for Kow<'_, S>
where
    S: ToKowned + ?Sized + Copy,
    S::Owned: Copy,
{
}

impl<S> PartialEq for Kow<'_, S>
where
    S: ToKowned + ?Sized + PartialEq,
    S::Owned: Borrow<S>,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<S> Eq for Kow<'_, S>
where
    S: ToKowned + Eq + ?Sized,
    S::Owned: Borrow<S>,
{
}

impl<S> PartialOrd for Kow<'_, S>
where
    S: ToKowned + ?Sized + PartialOrd,
    S::Owned: Borrow<S>,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<S> Ord for Kow<'_, S>
where
    S: ToKowned + Ord + ?Sized,
    S::Owned: Borrow<S>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<'a, S> Hash for Kow<'a, S>
where
    S: ToKowned + Hash + ?Sized,
    S::Owned: Borrow<S>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<'a, S> fmt::Display for Kow<'a, S>
where
    S: ?Sized + Display + ToKowned,
    S::Owned: Borrow<S>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'a> From<String> for Kow<'a, str> {
    fn from(value: String) -> Self {
        Kow::Owned(value)
    }
}

impl<'a> From<&'a String> for Kow<'a, str> {
    fn from(value: &'a String) -> Self {
        Kow::Ref(value.as_str())
    }
}

impl<'a> From<&'a str> for Kow<'a, str> {
    fn from(value: &'a str) -> Self {
        Kow::Ref(value)
    }
}

impl ToKowned for [u8] {
    type Owned = Bytes;

    fn to_owned(&self) -> Self::Owned {
        self.into()
    }
}

impl<'a> From<Bytes> for Kow<'a, [u8]> {
    fn from(value: Bytes) -> Self {
        Kow::Owned(value)
    }
}

impl<'a> From<&'a Bytes> for Kow<'a, [u8]> {
    fn from(value: &'a Bytes) -> Self {
        Kow::Ref(value)
    }
}

impl<'a> From<&'a [u8]> for Kow<'a, [u8]> {
    fn from(value: &'a [u8]) -> Self {
        Kow::Ref(value)
    }
}
