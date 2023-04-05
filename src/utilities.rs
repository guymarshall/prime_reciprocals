use std::collections::HashMap;

pub fn combine_vectors_into_hashmap(keys: Vec<i32>, values: Vec<i32>) -> HashMap<i32, i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for (key, value) in keys.into_iter().zip(values.into_iter()) {
        hash_map.insert(key, value);
    }
    hash_map
}

fn print_hashmap<K, V>(hash_map: &HashMap<K, V>)
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
{
    for (key, value) in hash_map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}