//! Hash maps and sets for the host side (renderer, frame sink): std's
//! containers with foldhash instead of SipHash, which is several times
//! slower for the small integer keys these use (and slower still in wasm).
//! A fixed seed: runs stay reproducible (nothing here takes outside keys).
//! Make them with `default()` (`new()` is std's `RandomState` only).

pub type BuildHasher = foldhash::fast::FixedState;
pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasher>;
pub type HashSet<T> = std::collections::HashSet<T, BuildHasher>;
