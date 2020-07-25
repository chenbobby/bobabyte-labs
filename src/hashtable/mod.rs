use std::fmt;

/// A trait for buckets of key-value tuples. Keys must be strings.
pub trait Bucket<T> {
    fn get(&self, key: &str) -> Option<T>;
    fn set(&mut self, key: &str, value: T);
    fn remove(&mut self, key: &str);
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T> fmt::Debug for dyn Bucket<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

/// A trait for a hashtable that takes strings as keys.
pub trait Hashtable<T> {
    fn get(&self, key: &str) -> Option<T>;
    fn set(&mut self, key: &str, value: T);
    fn remove(&mut self, key: &str);
}

mod int32;
pub use int32::*;
