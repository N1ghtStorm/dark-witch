// MIT License
//
// Copyright (c) 2025
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMds+:--------:+sdNMMMMMMMMMMM
// MMMMMMMMms:-+sdNMMMMMMMMNdy+--omMMMMMMMM
// MMMMMMh:` /mMMMMMMMMMMMMMMMMm+ `-yMMMMMM
// MMMMd--hN``--sNMMMMMMMMMMNy:..`md:.hMMMM
// MMM+`yMMMy hd+./hMMMMMMh/.+dd sMMMh`/MMM
// MM:.mMMMMM:.NMMh/.+dd+./hMMM--MMMMMm--NM
// M+`mMMMMMMN`+MMMMm-  .dMMMMo mMMMMMMN.:M
// d yMMMMMMMMy dNy:.omNs--sNm oMMMMMMMMh h
// /`MMMMMMMMMM.`.+dMMMMMMm+.``NMMMMMMMMM-:
// .:MMMMMMMd+./`oMMMMMMMMMMs /.+dMMMMMMM/`
// .:MMMMmo.:yNMs dMMMMMMMMm`oMNy:.omMMMM/`
// /`MNy:.omMMMMM--MMMMMMMM:.MMMMMNs--sNM.:
// d -` :++++++++: /++++++/ :++++++++:  : h
// M+ yddddddddddd+ yddddy /dddddddddddy`/M
// MM/.mMMMMMMMMMMM.-MMMM/.NMMMMMMMMMMm.:NM
// MMMo`sMMMMMMMMMMd sMMy hMMMMMMMMMMy`+MMM
// MMMMd--hMMMMMMMMM+`mN`/MMMMMMMMMh--hMMMM
// MMMMMMh:.omMMMMMMN.:/`NMMMMMMms.:hMMMMMM
// MMMMMMMMNs:./shmMMh  yMMNds/.:smMMMMMMMM
// MMMMMMMMMMMMdy+/---``---:+sdMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM

use crate::common::FieldType;
use crate::error::Error;
use crate::index::{Index, IndexList};
use serde_json;
use std::collections::HashMap;

pub struct Storage {
    pub name: String,
    pub data: HashMap<String, String>,
    pub indexes: IndexList,
}

pub struct Database {
    pub storages: Vec<Storage>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            storages: Vec::new(),
        }
    }

    pub fn create_storage(&mut self, name: String) -> Result<(), Error> {
        if self.storages.iter().any(|s| s.name.as_str() == name) {
            return Err(Error::StorageError(format!(
                "Storage with name '{}' already exists",
                name
            )));
        }
        self.storages.push(Storage {
            name,
            data: HashMap::new(),
            indexes: IndexList::new(),
        });
        Ok(())
    }

    pub fn get_storage(&self, name: String) -> Result<&Storage, Error> {
        self.storages
            .iter()
            .find(|s| s.name.as_str() == name)
            .ok_or(Error::StorageError(format!(
                "Storage with name '{}' not found",
                name
            )))
    }

    pub fn delete_storage(&mut self, storage_name: String) -> Result<(), Error> {
        self.storages.retain(|s| s.name != storage_name);
        Ok(())
    }

    pub fn get(&self, storage_name: String, key: String) -> Result<String, Error> {
        let storage = self
            .storages
            .iter()
            .find(|s| s.name.as_str() == storage_name)
            .ok_or(Error::StorageError(format!(
                "Storage with name '{}' not found",
                storage_name
            )))?;

        let value = storage
            .data
            .get(&key)
            .ok_or(Error::KeyNotFound(format!(
                "Key '{}' not found in storage '{}'",
                key, storage_name
            )))?
            .clone();
        Ok(value)
    }

    pub fn insert(
        &mut self,
        storage_name: String,
        key: String,
        value: String,
    ) -> Result<(), Error> {
        let storage = self
            .storages
            .iter_mut()
            .find(|s| s.name.as_str() == storage_name)
            .ok_or(Error::StorageError(format!(
                "Storage with name '{}' not found",
                storage_name
            )))?;

        if storage.data.contains_key(&key) {
            return Err(Error::KeyAlreadyExists(format!(
                "Key '{}' already exists in storage '{}'",
                key, storage_name
            )));
        }

        let indexes = &mut storage.indexes;

        // Parse the value as JSON
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&value) {
            // Check if it's an object
            if let Some(obj) = json_value.as_object() {
                // Iterate through all fields in the JSON object
                for (field_name, field_value) in obj {
                    // Process each field here
                    if let Some(index) = indexes.get_index_mut(field_name) {
                        match index {
                            Index::HashUnique(hashmap) => {
                                if let Some(field_str) = field_value.as_str() {
                                    if hashmap.contains_key(&field_str.to_string()) {
                                        return Err(Error::IndexError(format!(
                                            "Unique constraint violation: '{}' = '{}' already exists",
                                            field_name, field_str
                                        )));
                                    }
                                    hashmap.insert(field_str.to_string(), key.clone());
                                }                       
                            }
                            Index::BTreeUnique(btreemap) => {
                                // btreemap.insert(field_value.as_i64().unwrap(), key.clone());
                                if let Some(field_num) = field_value.as_i64() {
                                    if btreemap.contains_key(&field_num) {
                                        return Err(Error::IndexError(format!(
                                            "Unique constraint violation: '{}' = '{}' already exists",
                                            field_name, field_num
                                        )));
                                    }
                                    btreemap.insert(field_num, key.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        storage.data.insert(key, value);
        Ok(())
    }

    pub fn delete(&mut self, storage_name: String, key: String) -> Result<(), Error> {
        let storage = self
            .storages
            .iter_mut()
            .find(|s| s.name.as_str() == storage_name)
            .ok_or(Error::StorageError(format!(
                "Storage with name '{}' not found",
                storage_name
            )))?;

        let value = storage.data.get(&key).ok_or(Error::KeyNotFound(format!(
            "Key '{}' not found in storage '{}'",
            key, storage_name
        )))?;

        let indexes = &mut storage.indexes;

        // Parse the value as JSON
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&value) {
            // Check if it's an object
            if let Some(obj) = json_value.as_object() {
                // Iterate through all fields in the JSON object
                for (field_name, field_value) in obj {
                    // Process each field here
                    if let Some(index) = indexes.get_index_mut(field_name) {
                        match index {
                            Index::HashUnique(hashmap) => {
                                if let Some(field_str) = field_value.as_str() {
                                    hashmap.remove(&field_str.to_string());
                                }                       
                            }
                            Index::BTreeUnique(btreemap) => {
                                if let Some(field_num) = field_value.as_i64() {
                                    btreemap.remove(&field_num);
                                }
                            }
                        }
                    }
                }
            }
        }

        storage.data.remove(&key);
        Ok(())
    }

    pub fn update(
        &mut self,
        storage_name: String,
        key: String,
        new_value: String,
    ) -> Result<(), Error> {
        let storage = self
            .storages
            .iter_mut()
            .find(|s| s.name.as_str() == storage_name)
            .ok_or(Error::StorageError(format!(
                "Storage with name '{}' not found",
                storage_name
            )))?;

        storage.data.get(&key).ok_or(Error::KeyNotFound(format!(
            "Key '{}' not found in storage '{}'",
            key, storage_name
        )))?;

        let indexes = &mut storage.indexes;

        // Parse the value as JSON
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&new_value) {
            // Check if it's an object
            if let Some(obj) = json_value.as_object() {
                // Iterate through all fields in the JSON object
                for (field_name, field_value) in obj {
                    // Process each field here
                    if let Some(index) = indexes.get_index_mut(field_name) {
                        match index {
                            Index::HashUnique(hashmap) => {
                                if let Some(field_str) = field_value.as_str() {
                                    if hashmap.contains_key(&field_str.to_string()) {
                                        return Err(Error::IndexError(format!(
                                            "Unique constraint violation: '{}' = '{}' already exists",
                                            field_name, field_str
                                        )));
                                    }
                                    hashmap.insert(field_str.to_string(), key.clone());
                                }                       
                            }
                            Index::BTreeUnique(btreemap) => {
                                // btreemap.insert(field_value.as_i64().unwrap(), key.clone());
                                if let Some(field_num) = field_value.as_i64() {
                                    if btreemap.contains_key(&field_num) {
                                        return Err(Error::IndexError(format!(
                                            "Unique constraint violation: '{}' = '{}' already exists",
                                            field_name, field_num
                                        )));
                                    }
                                    btreemap.insert(field_num, key.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        storage.data.insert(key, new_value);
        Ok(())
    }

    pub fn create_index(
        &mut self,
        storage_name: String,
        field_name: String,
        field_type: FieldType,
        unique: bool,
    ) -> Result<(), Error> {
        let storage = self
            .storages
            .iter_mut()
            .find(|s| s.name.as_str() == storage_name)
            .ok_or(Error::StorageError(format!(
                "Storage with name '{}' not found",
                storage_name
            )))?;

        let mut index = match field_type {
            FieldType::String => {
                if unique {
                    Index::new_unique_hashmap()
                } else {
                    return Err(Error::IndexError(
                        "Non unique indexes are not supported for strings".to_string(),
                    ));
                }
            }
            FieldType::Number => {
                if unique {
                    Index::new_unique_btreemap()
                } else {
                    return Err(Error::IndexError(
                        "Non unique indexes are not supported for numbers".to_string(),
                    ));
                }
            }
        };

        // Iterate over all key-value pairs in the storage
        for (key, value) in &storage.data {
            // Try to parse the value as JSON
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&value) {
                // If the field exists in the JSON, add it to the index
                if let Some(field_value) = json_value.get(&field_name) {
                    match index {
                        Index::HashUnique(_) => {
                            if let Some(field_str) = field_value.as_str() {
                                index
                                    .add_string_unique(key.clone(), field_str.to_string())
                                    .map_err(|e| {
                                        Error::IndexError(format!("Error creating index: {:?}", e))
                                    })?;
                            }
                        }
                        Index::BTreeUnique(_) => {
                            if let Some(field_num) = field_value.as_i64() {
                                index
                                    .add_number_unique(key.clone(), field_num)
                                    .map_err(|e| {
                                        Error::IndexError(format!("Error creating index: {:?}", e))
                                    })?;
                            }
                        }
                    }
                }
            }
        }

        storage.indexes.create_index(field_name, index);
        Ok(())
    }
}
