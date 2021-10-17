use std::collections::HashMap;

fn example<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>

{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

fn main() {
    let map: HashMap<_, _> = vec![(2, 4), (1, 3), (5, 2)].into_iter().collect();
    dbg!(example(&map));
}