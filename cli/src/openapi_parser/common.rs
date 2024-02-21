use std::str::FromStr;

use yaml_rust::{yaml, Yaml};

pub fn check_unexpected_keys(keys: Vec<&str>, hash: &yaml::Hash) {
    for (k, _) in hash.iter() {
        if !keys.contains(&k.as_str().unwrap()) {
            dbg!(hash);
            panic!("unexpected key: {}", k.as_str().unwrap());
        }
    }
}

pub fn get_value<T: FromStr>(hash: &yaml::Hash, key: &str) -> Option<T> {
    hash.get(&Yaml::String(key.to_string()))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .and_then(|s| s.parse().ok())
}
