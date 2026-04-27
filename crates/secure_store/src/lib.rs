use std::collections::HashMap;

#[derive(Default)]
pub struct InMemorySecretStore {
    data: HashMap<String, String>,
}

impl InMemorySecretStore {
    pub fn put(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_and_reads_secret() {
        let mut store = InMemorySecretStore::default();
        store.put("provider.api_key", "secret");
        assert_eq!(store.get("provider.api_key"), Some("secret"));
    }
}
