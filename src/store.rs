use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

pub type Value = Bytes;

#[derive(Clone)]
pub struct Database {
    pub data: Arc<RwLock<HashMap<String, Entry>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub struct Entry {
    value: Vec<u8>,
    expire_at: Option<Instant>,
}

impl Entry {
    pub fn new(value: Vec<u8>) -> Self {
        Entry {
            value,
            expire_at: None,
        }
    }

    pub fn get_value(&self) -> &Vec<u8> {
        &self.value
    }
}

pub enum SetMode {
    Default,
    Nx,
    Xx,
}

pub struct SetOptions {
    pub mode: SetMode,           // Default | Nx | Xx
    pub expire: Option<Instant>, // From EX/PX
    pub keep_ttl: bool,
}

#[async_trait::async_trait]
pub trait KvStore {
    async fn get(&self, key: &str) -> Option<Value>;
    async fn set(&self, key: &str, val: Value, opts: SetOptions) -> bool;
    async fn del(&self, keys: &[String]) -> usize;
    async fn exists(&self, keys: &[String]) -> usize;
    async fn expire_at(&self, key: &str, when: Instant) -> bool;
    async fn persist(&self, key: &str) -> bool;
    async fn ttl_ms(&self, key: &str) -> Option<i64>;
}

#[async_trait::async_trait]
impl KvStore for Database {
    async fn get(&self, key: &str) -> Option<Value> {
        let data = self.data.read().await;
        data.get(key)
            .map(|entry| Bytes::from(entry.get_value().clone()))
    }

    async fn set(&self, key: &str, val: Value, opts: SetOptions) -> bool {
        let mode = opts.mode;
        let mut data = self.data.write().await;
        match mode {
            SetMode::Default => {
                let entry = Entry::new(val.to_vec());
                data.insert(key.to_string(), entry);
                true
            }
            SetMode::Nx => {
                if data.contains_key(key) {
                    false
                } else {
                    let entry = Entry::new(val.to_vec());
                    data.insert(key.to_string(), entry);
                    true
                }
            }
            SetMode::Xx => {
                if data.contains_key(key) {
                    let entry = Entry::new(val.to_vec());
                    data.insert(key.to_string(), entry);
                    true
                } else {
                    false
                }
            }
        }
    }

    async fn del(&self, keys: &[String]) -> usize {
        let mut data = self.data.write().await;
        for key in keys {
            data.remove(key);
        }
        keys.len()
    }

    async fn exists(&self, keys: &[String]) -> usize {
        let data = self.data.read().await;
        let mut count = 0;
        for key in keys {
            if data.contains_key(key) {
                count += 1;
            }
        }
        count
    }

    async fn expire_at(&self, key: &str, when: Instant) -> bool {
        let mut data = self.data.write().await;
        if let Some(entry) = data.get_mut(key) {
            entry.expire_at = Some(when);
            true
        } else {
            false
        }
    }

    async fn persist(&self, key: &str) -> bool {
        let mut data = self.data.write().await;
        if let Some(entry) = data.get_mut(key) {
            entry.expire_at = None;
            true
        } else {
            false
        }
    }

    async fn ttl_ms(&self, key: &str) -> Option<i64> {
        let data = self.data.read().await;
        if let Some(entry) = data.get(key) {
            if let Some(expire_at) = entry.expire_at {
                let now = Instant::now();
                if expire_at > now {
                    let duration = expire_at.duration_since(now);
                    Some(duration.as_millis() as i64)
                } else {
                    Some(-1) // Key has expired
                }
            } else {
                Some(-1) // Key has no expiration
            }
        } else {
            None // Key does not exist
        }
    }
}
