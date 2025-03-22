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

use crate::error::Error;
use std::collections::{BTreeMap, HashMap};

pub type FieldName = String;
pub type FieldValue = String;
pub type Key = String;

pub enum Index {
    // Index for numbers
    // key - key
    // value - list of ids
    BTreeUnique(BTreeMap<Key, i64>),
    // Index for strings
    HashUnique(HashMap<Key, FieldValue>),
}

impl Index {
    pub fn new_unique_hashmap() -> Self {
        Self::HashUnique(HashMap::new())
    }

    pub fn new_unique_btreemap() -> Self {
        Self::BTreeUnique(BTreeMap::new())
    }

    pub fn add_string_unique(&mut self, key: Key, value: FieldValue) -> Result<(), Error> {
        if let Self::HashUnique(hashmap) = self {
            if hashmap.contains_key(&key) {
                return Err(Error::IndexError("Key already exists".to_string()));
            }
            hashmap.insert(key, value);
        }
        Ok(())
    }

    pub fn add_number_unique(&mut self, key: Key, value: i64) -> Result<(), Error> {
        if let Self::BTreeUnique(btreemap) = self {
            if btreemap.contains_key(&key) {
                return Err(Error::IndexError("Key already exists".to_string()));
            }
            btreemap.insert(key, value);
        }
        Ok(())
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
}
