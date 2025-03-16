use std::{collections::HashMap, sync::{Arc, Mutex}};

// pub type Database = Arc<DatabaseInner>;
pub type Database = DatabaseInner;

pub struct Storage {
    name: Arc<Mutex<String>>,
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
        if self.storages.iter().any(|s| *s.name.lock().expect("Failed to lock storage").as_str() == name) {
            return Err(format!("Storage with name '{}' already exists", name));
        }
        self.storages.push(Storage { name: Arc::new(Mutex::new(name)), data: Arc::new(Mutex::new(HashMap::new())) });
        Ok(())
    }

    pub fn insert(&self, storage_name: String, key: String, value: String) -> Result<(), String> {
        let storage = self.storages.iter().find(|s| *s.name.lock().expect("Failed to lock storage").as_str() == storage_name).ok_or(format!("Storage with name '{}' not found", storage_name))?;
        storage.data.lock().expect("Failed to lock storage").insert(key, value);
        Ok(())
    }

    pub fn get(&self, storage_name: String, key: String) -> Result<String, String> {
        let storage = self.storages.iter().find(|s| *s.name.lock().expect("Failed to lock storage").as_str() == storage_name).ok_or(format!("Storage with name '{}' not found", storage_name))?;

        // TODO: remove clone
        let value = storage.data.lock().expect("Failed to lock storage").get(&key).ok_or(format!("Key '{}' not found in storage '{}'", key, storage_name))?.clone();
        Ok(value)
    }    
}