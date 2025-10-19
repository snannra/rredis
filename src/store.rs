use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct Database {
    pub data: RwLock<HashMap<String, String>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: RwLock::new(HashMap::new()),
        }
    }
}
