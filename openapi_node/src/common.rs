use yaml_rust::yaml;

pub type YamlEntry<'a> = (&'a yaml::Yaml, &'a yaml::Yaml);

pub type YamlWithKey<'a> = (&'a yaml::Yaml, Option<&'a String>);

pub fn check_unexpected_keys(keys: Vec<&str>, hash: &yaml::Hash) -> Result<(), String> {
    let mut unexpected_keys: Vec<String> = vec![];

    for (k, _) in hash.iter() {
        let key = k.as_str().unwrap();
        if !keys.contains(&key) {
            unexpected_keys.push(key.to_string());
        }
    }

    if unexpected_keys.len() > 0 {
        Err(format!(
            "[check_unexpected_keys] Unexpected keys: {:?}",
            unexpected_keys.join(", ")
        ))
    } else {
        Ok(())
    }
}

pub struct YamlHash<'a> {
    hash: &'a yaml::Hash,
}

impl<'a> YamlHash<'a> {
    pub fn new(hash: &'a yaml::Hash) -> Self {
        Self { hash }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get_value(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get_value(key).and_then(|v| v.as_bool())
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get_value(key).and_then(|v| v.as_i64())
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.get_value(key).and_then(|v| v.as_f64())
    }

    pub fn get_vec(&self, key: &str) -> Option<&yaml::Array> {
        self.get_value(key).and_then(|v| v.as_vec())
    }

    pub fn get_hash(&self, key: &str) -> Option<&yaml::Hash> {
        self.get_value(key).and_then(|v| v.as_hash())
    }

    pub fn get_value(&self, key: &str) -> Option<&yaml::Yaml> {
        self.hash.get(&yaml::Yaml::String(key.to_string()))
    }
}
