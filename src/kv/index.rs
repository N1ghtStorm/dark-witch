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

use crate::kv::error::Error;
use std::collections::{BTreeMap, HashMap};

pub type FieldName = String;
pub type FieldValue = String;
pub type Key = String;

#[derive(Debug)]
pub enum Index {
    // Index for numbers
    // key - key
    // value - list of ids
    BTreeUnique(BTreeMap<i64, Key>),
    // Uniqe Index for strings
    HashUnique(HashMap<FieldValue, Key>),
    // Index for strings
    Hash(HashMap<FieldValue, Vec<Key>>),
}

impl Index {
    pub fn new_unique_hashmap() -> Self {
        Self::HashUnique(HashMap::new())
    }

    pub fn new_unique_btreemap() -> Self {
        Self::BTreeUnique(BTreeMap::new())
    }

    pub fn new_hashmap() -> Self {
        Self::Hash(HashMap::new())
    }

    pub fn add_string_unique(&mut self, key: Key, field_value: FieldValue) -> Result<(), Error> {
        if let Self::HashUnique(hashmap) = self {
            if hashmap.contains_key(&key) {
                return Err(Error::IndexError("Duplicate key".to_string()));
            }
            hashmap.insert(field_value, key);
        }
        Ok(())
    }

    pub fn add_number_unique(&mut self, key: Key, num_value: i64) -> Result<(), Error> {
        if let Self::BTreeUnique(btreemap) = self {
            if btreemap.contains_key(&num_value) {
                return Err(Error::IndexError("Duplicate key".to_string()));
            }
            btreemap.insert(num_value, key);
        }
        Ok(())
    }

    pub fn add_string(&mut self, key: Key, field_value: FieldValue) -> Result<(), Error> {
        if let Self::Hash(hashmap) = self {
            hashmap
                .entry(field_value)
                .or_insert_with(Vec::new)
                .push(key);
        }
        Ok(())
    }

    pub fn get_unique_hash_key(&self, field_value: FieldValue) -> Option<&Key> {
        match self {
            Self::HashUnique(hashmap) => hashmap.get(&field_value),
            Self::BTreeUnique(_) => None,
            Self::Hash(_) => None,
        }
    }

    pub fn get_hash_keys(&self, field_value: FieldValue) -> Option<&Vec<Key>> {
        match self {
            Self::Hash(hashmap) => hashmap.get(&field_value),
            Self::HashUnique(_) => None,
            Self::BTreeUnique(_) => None,
        }
    }
}

pub struct IndexList {
    list: HashMap<FieldName, Index>,
}

impl IndexList {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    pub fn create_index(&mut self, field_name: FieldName, index: Index) {
        self.list.insert(field_name, index);
    }

    pub fn get_index(&self, field_name: &FieldName) -> Option<&Index> {
        self.list.get(field_name)
    }

    pub fn get_index_mut(&mut self, field_name: &FieldName) -> Option<&mut Index> {
        self.list.get_mut(field_name)
    }

    pub fn index_exists(&self, field_name: &FieldName) -> bool {
        self.list.contains_key(field_name)
    }
}
