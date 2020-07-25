use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::fmt;

use super::*;

struct Int32Array {
    pub array: Box<[Option<(String, i32)>]>,
    pub length: usize,
    pub capacity: usize,
}

impl Int32Array {
    fn new() -> Int32Array {
        Int32Array{
            array: vec![None].into_boxed_slice(),
            length: 0,
            capacity: 1,
        }
    }

    fn grow(&mut self) {
        self.capacity <<= 1;
        let mut new_array = vec![None; self.capacity];
        new_array[..self.length].clone_from_slice(&self.array);
        self.array = new_array.into_boxed_slice();
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.array.iter()).finish()
    }
}

impl Bucket<i32> for Int32Array {
    fn get(&self, key: &str) -> Option<i32> {
        for x in self.array.iter() {
            match &*x {
                Some((k, v)) if k == key => {
                    return Some(*v);
                },
                _ => continue,
            }
        }
        None
    }

    fn set(&mut self, key: &str, value: i32) {
        // Store a ref to the first `None`, in case the key does not already exist.
        let mut first_none = None;

        // Set value for `key`, if `key` already exists.
        for x in self.array[..self.length].iter_mut() {
            match &*x {
                Some((k, _)) if k == key => {
                    *x = Some((String::from(key), value));
                    return;
                },
                None if first_none == None => {
                    first_none = Some(x);
                },
                _ => continue,
            }
        }

        // Insert key-value tuple in a `None` slot, if it exists.
        if let Some(x) = first_none {
            *x = Some((String::from(key), value));
            return;
        }

        // Insert key-value tuple at the end of the array.
        if self.length == self.capacity {
            self.grow();
        }
        self.array[self.length] = Some((String::from(key), value));
        self.length += 1;
    }

    fn remove(&mut self, key: &str) {
        for x in self.array.iter_mut() {
            match &*x {
                Some((k, _)) if k == key => {
                    *x = None;
                    return;
                },
                _ => continue,
            }
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

#[derive(Debug)]
pub struct Int32Hashtable {
    pub buckets: Box<[Box<dyn Bucket<i32>>]>,
    pub size: usize,
}

impl Int32Hashtable {
    pub fn new_with_array(size: usize) -> Self {
        let mut buckets: Vec<Box<dyn Bucket<i32>>> = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(Box::new(Int32Array::new()));
        }
        Int32Hashtable{
            buckets: buckets.into_boxed_slice(),
            size,
        }
    }

    fn get_bucket(&self, key: &str) -> &Box<dyn Bucket<i32>> {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.as_bytes());
        let hash = hasher.finish();
        let index = (hash as usize) % self.size;
        &self.buckets[index]
    }

    fn get_mut_bucket(&mut self, key: &str) -> &mut Box<dyn Bucket<i32>> {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.as_bytes());
        let hash = hasher.finish();
        let index = (hash as usize) % self.size;
        &mut self.buckets[index]
    }
}

impl Hashtable<i32> for Int32Hashtable {
    fn get(&self, key: &str) -> Option<i32> {
        self.get_bucket(key).get(key)
    }

    fn set(&mut self, key: &str, value: i32) {
        self.get_mut_bucket(key).set(key, value)
    }

    fn remove(&mut self, key: &str) {
        self.get_mut_bucket(key).remove(key)
    }
}

#[cfg(test)]
mod test {
    use std::iter;
    use rand::Rng;
    use rand::distributions::Alphanumeric;

    use super::*;

    fn gen_unique_keys(num_keys: usize, key_length: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let mut keys = Vec::with_capacity(num_keys);
        for i in 0..num_keys {
            let mut key: String = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(key_length - 1)
                .collect();
            key.push(i as u8 as char);
            keys.push(key);
        }
        keys
    }

    fn gen_values(num_values: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        iter::repeat(())
            .map(|()| rng.gen())
            .take(num_values)
            .collect()
    }

    #[test]
    fn hashtable_stores_and_deletes_key_values() {
        let mut ht = Int32Hashtable::new_with_array(16);

        let keys = gen_unique_keys(1024, 16);
        let values = gen_values(1024);

        // Test `set` and `get`.
        for (key, value) in keys.iter().zip(values.iter()).take(99) {
            ht.set(key, *value);
        }
        for (key, value) in keys.iter().zip(values.iter()).take(99) {
            assert!(ht.get(key) == Some(*value));
        }
        assert!(ht.get(&keys[99]) == None);

        // Test `remove`.
        for key in keys.iter().take(30) {
            ht.remove(key);
        }
        for key in keys.iter().take(30) {
            assert!(ht.get(key) == None);
        }

    }
}
