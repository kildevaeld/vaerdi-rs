use avagarden::sync::{Lazy, RwLock};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::String;

static STRINGS: Lazy<RwLock<hashbrown::HashSet<String>>> =
    Lazy::new(|| RwLock::new(Default::default()));

static COUNT: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

pub fn get_or_intern<T>(string: T) -> String
where
    T: AsRef<str> + Into<String>,
{
    let strings = STRINGS.read();
    if let Some(value) = strings.get(string.as_ref()) {
        return value.clone();
    }

    drop(strings);

    COUNT.fetch_add(string.as_ref().len(), Ordering::Relaxed);

    let string: String = string.as_ref().into();

    STRINGS.write().insert(string.clone());

    string
}

pub fn get(string: impl AsRef<str>) -> Option<String> {
    let strings = STRINGS.read();
    strings.get(string.as_ref()).cloned()
}

pub fn clear() {
    let mut lock = STRINGS.write();
    lock.clear();
    COUNT.store(0, Ordering::SeqCst);
}

pub fn total_allocated() -> usize {
    COUNT.load(Ordering::Relaxed)
}
