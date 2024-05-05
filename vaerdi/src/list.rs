use core::{
    fmt::{self, Write},
    iter::FromIterator,
};

use alloc::vec::Vec;

use crate::value::Value;

#[cfg_attr(feature = "ord", derive(Hash, PartialOrd, Ord))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<V = Value> {
    pub(crate) v: Vec<V>,
}

impl<V> Default for List<V> {
    fn default() -> Self {
        List {
            v: Default::default(),
        }
    }
}

impl<V> List<V> {
    pub const fn new() -> List<V> {
        List { v: Vec::new() }
    }

    pub fn get(&self, idx: usize) -> Option<&V> {
        self.v.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut V> {
        self.v.get_mut(idx)
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn extend<I: IntoIterator<Item = V>>(&mut self, iter: I) {
        self.v.extend(iter)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, V> {
        self.v.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, V> {
        self.v.iter_mut()
    }

    pub fn push(&mut self, value: impl Into<V>) {
        self.v.push(value.into());
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;

        for (idx, v) in self.v.iter().enumerate() {
            if idx > 0 {
                writeln!(f, ", ")?;
            }
            write!(f, "{v}")?;
        }

        f.write_char(']')
    }
}

impl<V, T: Into<V>> From<Vec<T>> for List<V> {
    fn from(value: Vec<T>) -> Self {
        List {
            v: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<V> IntoIterator for List<V> {
    type Item = V;

    type IntoIter = alloc::vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

impl<V> FromIterator<V> for List<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        List {
            v: Vec::from_iter(iter),
        }
    }
}

impl<V> Extend<V> for List<V> {
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        self.v.extend(iter)
    }
}
