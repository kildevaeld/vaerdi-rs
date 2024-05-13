use alloc::{sync::Arc, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bytes(pub(crate) Arc<[u8]>);

impl core::ops::Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes(value.into())
    }
}

impl<'a> From<&'a [u8]> for Bytes {
    fn from(value: &'a [u8]) -> Self {
        value.to_vec().into()
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(value: Bytes) -> Self {
        value.0.to_vec()
    }
}
