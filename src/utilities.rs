#![forbid(unsafe_code)]

use std::collections::HashMap;

pub fn combine_vectors_into_hashmap(keys: Vec<i32>, values: Vec<i32>) -> HashMap<i32, i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for (key, value) in keys.into_iter().zip(values.into_iter()) {
        hash_map.insert(key, value);
    }
    hash_map
}

pub fn sort_hashmap_by_key<K: Ord, V>(hash_map: &HashMap<K, V>) -> Vec<(&K, &V)> {
    let mut sorted_pairs: Vec<(&K, &V)> = hash_map
        .into_iter()
        .collect();
    sorted_pairs.sort_by_key(|pair| pair.0);
    sorted_pairs
}