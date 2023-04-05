pub use std::collections::HashMap;

fn combine_vectors_into_hashmap(keys: Vec<String>, values: Vec<i32>) -> HashMap<String, i32> {
    let mut hash_map: HashMap<String, i32> = HashMap::new();
    for (key, value) in keys.into_iter().zip(values.into_iter()) {
        hash_map.insert(key, value);
    }
    hash_map
}