use nohash_hasher::NoHashHasher;
use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;

pub(crate) type TermKey = u64;
pub(crate) type StateKey = u64;

pub(crate) type NoHashMap<K, V> = HashMap<K, V, BuildHasherDefault<NoHashHasher<K>>>;
pub(crate) type NoHashSet<K> = HashSet<K, BuildHasherDefault<NoHashHasher<K>>>;

pub(crate) type FxHashSet<K> = HashMap<K, BuildHasherDefault<FxHasher>>;
pub(crate) type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
