use std::{collections::HashMap, sync::{Arc, Mutex}};

pub type Database = Arc<DatabaseInner>;

pub struct Storage {
    name: String,
    data: Arc<Mutex<HashMap<String, String>>>,
}

pub struct DatabaseInner {
    storage_num: u64,
    storages: Vec<Storage>,
}

impl DatabaseInner {
    pub fn new() -> Self {
        Self { storage_num: 0, storages: Vec::new() }
    }

    pub fn create_storage(&mut self, name: String) -> Result<(), String> {
        if self.storages.iter().any(|s| s.name == name) {
            return Err(format!("Storage with name '{}' already exists", name));
        }
        self.storages.push(Storage { name, data: Arc::new(Mutex::new(HashMap::new())) });
        Ok(())
    }
}