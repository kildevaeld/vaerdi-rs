use core::{
    fmt::{self, Write},
    iter::FromIterator,
};

use crate::{value::Value, String};
use hashbrown::{
    hash_map::{DefaultHashBuilder, Entry, IntoIter, Iter, IterMut},
    HashMap,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<V = Value> {
    pub(crate) inner: HashMap<String, V>,
}

impl<V> Default for Map<V> {
    fn default() -> Self {
        Map {
            inner: Default::default(),
        }
    }
}

impl<V> Map<V> {
    pub fn with_capacity(len: usize) -> Map<V> {
        Map {
            inner: HashMap::with_capacity(len),
        }
    }

    #[inline]
    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<V>) -> Option<V> {
        self.inner.insert(name.into(), value.into())
    }

    #[inline]
    pub fn get(&self, name: impl AsRef<str>) -> Option<&V> {
        self.inner.get(name.as_ref())
    }

    #[inline]
    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut V> {
        self.inner.get_mut(name.as_ref())
    }

    #[inline]
    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        self.inner.contains_key(name.as_ref())
    }

    #[inline]
    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<V> {
        self.inner.remove(name.as_ref())
    }

    #[inline]
    pub fn entry<S>(&mut self, key: S) -> Entry<'_, String, V, DefaultHashBuilder>
    where
        S: Into<String>,
    {
        self.inner.entry(key.into())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, String, V> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, String, V> {
        self.inner.iter_mut()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('{')?;

        for (idx, (k, v)) in self.inner.iter().enumerate() {
            if idx > 0 {
                writeln!(f, ", ")?;
            }
            write!(f, "{k}: {v}")?;
        }

        f.write_char('}')
    }
}

impl<V> FromIterator<(String, V)> for Map<V> {
    fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
        Map {
            inner: HashMap::from_iter(iter),
        }
    }
}

impl<V> Extend<(String, V)> for Map<V> {
    fn extend<T: IntoIterator<Item = (String, V)>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

impl<V> IntoIterator for Map<V> {
    type Item = (String, V);
    type IntoIter = IntoIter<String, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, V> IntoIterator for &'a Map<V> {
    type Item = (&'a String, &'a V);
    type IntoIter = Iter<'a, String, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

// impl<'a> ops::Index<&'a str> for Map {
//     type Output = Value;

//     fn index(&self, index: &'a str) -> &Value {
//         static NULL: Value = Value::None;
//         self.inner.get(index).unwrap_or(&NULL)
//     }
// }

// impl<'a> ops::IndexMut<&'a str> for Map {
//     fn index_mut(&mut self, index: &'a str) -> &mut Value {
//         if !self.contains(index) {
//             self.inner.insert(index.to_string(), Value::None);
//         }
//         self.inner.get_mut(index).unwrap()
//     }
// }

impl<V> From<HashMap<String, V>> for Map<V> {
    fn from(map: HashMap<String, V>) -> Map<V> {
        Map { inner: map }
    }
}
