use alloc::vec;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};

#[doc(inline)]
pub use alloc::collections::*;

struct FnvHasher(u64);

impl FnvHasher {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    fn new() -> Self {
        Self(Self::OFFSET_BASIS)
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= *byte as u64;
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }
}

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets
            .iter()
            .flat_map(|bucket| bucket.iter().map(|(k, v)| (k, v)))
    }
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    fn resize(&mut self) {
        let new_cap = if self.buckets.is_empty() {
            16
        } else {
            self.buckets.len() * 2
        };

        let mut new_buckets = Vec::with_capacity(new_cap);
        new_buckets.resize_with(new_cap, Vec::new);

        let old_buckets = core::mem::replace(&mut self.buckets, new_buckets);
        for (k, v) in old_buckets.into_iter().flatten() {
            let idx = Self::bucket_index(&k, new_cap);
            self.buckets[idx].push((k, v));
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || (self.len + 1) * 4 > self.buckets.len() * 3 {
            self.resize();
        }

        let idx = Self::bucket_index(&key, self.buckets.len());
        let bucket = &mut self.buckets[idx];

        for (k, v) in bucket.iter_mut() {
            if k == &key {
                return Some(core::mem::replace(v, value));
            }
        }

        bucket.push((key, value));
        self.len += 1;

        None
    }

    fn hash_key(key: &K) -> u64 {
        let mut hasher = FnvHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn bucket_index(key: &K, bucket_count: usize) -> usize {
        Self::hash_key(key) as usize % bucket_count
    }
}
