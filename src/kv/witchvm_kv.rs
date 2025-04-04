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

use serde::{Deserialize, Serialize};

use crate::kv::error::Error;
use crate::kv::{database::Database, index::Index};
use tokio::time::{Duration, Instant};

pub struct WitchVMKV {
    instruction_storage_name: Option<String>,
    output: Vec<String>,
}

impl WitchVMKV {
    pub fn new() -> Self {
        Self {
            instruction_storage_name: None,
            output: Vec::new(),
        }
    }

    pub fn into_output(self) -> Vec<String> {
        self.output
    }

    pub fn execute(
        &mut self,
        database: &mut Database,
        instructions: Vec<Instruction>,
    ) -> Result<Vec<ExplainStep>, Error> {
        let mut explain = Vec::new();
        for instruction in instructions {
            match instruction {
                Instruction::Get { key } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err(Error::ExecutionError(
                            "No storage name provided".to_string(),
                        ));
                    };

                    let value = database.get(storage_name, key.clone())?;
                    self.output.push(value);
                }
                Instruction::Set { key, value } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err(Error::ExecutionError(
                            "No storage name provided".to_string(),
                        ));
                    };
                    database.insert(storage_name, key.clone(), value)?;
                }
                Instruction::GetJsonField { key, field } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err(Error::ExecutionError(
                            "No storage name provided".to_string(),
                        ));
                    };

                    match database.get(storage_name, key.clone()) {
                        Ok(value) => match serde_json::from_str::<serde_json::Value>(&value) {
                            Ok(json_value) => match json_value.get(&field) {
                                Some(field_value) => println!(
                                    "JSON field '{}' in key '{}': {}",
                                    field, key, field_value
                                ),
                                None => {
                                    return Err(Error::ExecutionError(format!(
                                        "JSON field '{}' not found in key '{}'",
                                        field, key
                                    )))
                                }
                            },
                            Err(e) => {
                                return Err(Error::ExecutionError(format!(
                                    "Value for key '{}' is not valid JSON: {}",
                                    key, e
                                )))
                            }
                        },
                        Err(e) => {
                            return Err(Error::ExecutionError(format!(
                                "Key '{}' not found: {:?}",
                                key, e
                            )))
                        }
                    }
                }
                Instruction::UseStorage { name } => {
                    self.instruction_storage_name = Some(name.clone());
                    explain.push(ExplainStep::SetStorage(name));
                }
                Instruction::Scan {
                    index_filter,
                    full_scan_filter,
                    string_fields_values,
                    number_fields_values,
                } => {
                    let start = Instant::now();
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err(Error::ExecutionError(
                            "No storage name provided".to_string(),
                        ));
                    };

                    let indexes = &database.get_storage(storage_name.clone())?.indexes;

                    let mut start_index_search = false;

                    // Check if all fields are indexed
                    // if not - then full scan
                    // not sure it's correct
                    let maybe_string_fields_indexed = string_fields_values
                        .iter()
                        .map(|x| indexes.index_exists(&x.0))
                        .reduce(|x, y| x && y);

                    // TODO: implement Number indexes not supported yet
                    // let maybe_num_fields_indexed = number_fields_values
                    //     .iter()
                    //     .map(|x| indexes.index_exists(&x.0))
                    //     .reduce(|x, y| x && y);

                    if string_fields_values.len() > 0 && number_fields_values.len() > 0 {
                        start_index_search = maybe_string_fields_indexed.unwrap_or(false)
                        // TODO: implement Number indexes not supported yet
                        // && maybe_num_fields_indexed.unwrap_or(false);
                    } else if string_fields_values.len() > 0 && number_fields_values.len() == 0 {
                        start_index_search = maybe_string_fields_indexed.unwrap_or(false);
                    } else if string_fields_values.len() == 0 && number_fields_values.len() > 0 {
                        // TODO: implement Number indexes not supported yet
                        // all_fields_indexed = maybe_num_fields_indexed.unwrap_or(false);
                    }

                    println!("All fields indexed: {}", start_index_search);

                    if start_index_search {
                        for (field, _) in string_fields_values.iter() {
                            if let Some(index) = indexes.get_index(field) {
                                match index {
                                    Index::Hash(_) => {
                                        let string_values = database.string_index_search(
                                            storage_name.clone(),
                                            index,
                                            index_filter.condition(),
                                        )?;
                                        self.output.extend(string_values);
                                    }
                                    Index::HashUnique(_) => {
                                        let string_values = database.string_index_search(
                                            storage_name.clone(),
                                            index,
                                            index_filter.condition(),
                                        )?;
                                        self.output.extend(string_values);
                                    }
                                    Index::BTreeUnique(_) => {
                                        return Err(Error::ExecutionError(
                                            "BTreeUnique indexes are for numbers only".to_string(),
                                        ));
                                    }
                                }
                            }

                            // for number fields
                            // TODO: implement
                        }
                        explain.push(ExplainStep::IndexScan {
                            time: start.elapsed(),
                        });
                    } else {
                        let storage = database.get_storage(storage_name)?;
                        for (_, value) in storage.data.iter() {
                            match full_scan_filter {
                                Filter::Condition(ref condition) => {
                                    if condition(value.clone()) {
                                        self.output.push(value.clone());
                                    }
                                }
                            }
                        }
                        explain.push(ExplainStep::FullScan {
                            time: start.elapsed(),
                        });
                    }
                }
                Instruction::MapOutput { map_fn } => {
                    self.output = self
                        .output
                        .iter()
                        .map(|value| map_fn(value.clone()))
                        .collect();
                    explain.push(ExplainStep::MapOutput);
                }
                Instruction::SortOutput { field } => {
                    self.output.sort_by(|x, y| {
                        let json_x: serde_json::Value = serde_json::from_str(x).unwrap_or_default();
                        let json_y: serde_json::Value = serde_json::from_str(y).unwrap_or_default();

                        let x_field = json_x[&field].to_string();
                        let y_field = json_y[&field].to_string();

                        x_field.cmp(&y_field)
                    });
                    explain.push(ExplainStep::SortOutput);
                }
                Instruction::SetLimit { count } => {
                    if count < self.output.len() as u64 {
                        self.output = self.output[0..count as usize].to_vec();
                    }
                    explain.push(ExplainStep::Limit);
                }
                Instruction::SetOffset { count } => {
                    if count < self.output.len() as u64 {
                        self.output = self.output[count as usize..].to_vec();
                        explain.push(ExplainStep::Offset);
                    } else {
                        self.output = Vec::new();
                        explain.push(ExplainStep::Offset);
                    }
                }
                _ => (),
            }
        }

        Ok(explain)
    }
}

#[allow(dead_code)]
pub enum Instruction {
    UseStorage {
        name: String,
    },
    ClearOutput,
    ChooseScanPath {
        filter: Filter,
    },
    Scan {
        index_filter: Filter,
        full_scan_filter: Filter,
        string_fields_values: Vec<(String, String)>,
        number_fields_values: Vec<(String, f64)>,
    },
    MapOutput {
        map_fn: Box<dyn Fn(String) -> String>,
    },
    SortOutput {
        field: String,
    },
    SetLimit {
        count: u64,
    },
    SetOffset {
        count: u64,
    },
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Delete {
        key: String,
    },
    Print {
        key: String,
    },
    GetJsonField {
        key: String,
        field: String,
    },
    Clear,
}

pub enum Filter {
    Condition(Box<dyn Fn(String) -> bool>),
}

impl Filter {
    pub fn condition(&self) -> &Box<dyn Fn(String) -> bool> {
        match self {
            Filter::Condition(condition) => condition,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExplainStep {
    SetStorage(String),
    FullScan { time: Duration },
    IndexScan { time: Duration },
    MapOutput,
    SortOutput,
    Limit,
    Offset,
}
