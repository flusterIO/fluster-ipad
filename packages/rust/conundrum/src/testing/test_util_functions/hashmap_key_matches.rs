use std::hash::Hash;

use dashmap::DashMap;

pub fn hashmap_key_matches<K, V, T: FnOnce(&V) -> bool>(data: &DashMap<K, V>, key: &K, key_label: &str, compare: T)
    where K: Hash + Eq {
    let x = data.get(key).unwrap_or_else(|| panic!("Failed to match hasmap key {}", key_label));
    let res = compare(x.value());

    assert!(res, "Found value for key {}, but values do not match.", key_label);
}
