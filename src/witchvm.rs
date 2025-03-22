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

use crate::database::Database;
use crate::error::Error;

pub struct WitchVM {
    instruction_storage_name: Option<String>,
    output: Vec<String>,
}

impl WitchVM {
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
    ) -> Result<(), Error> {
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
                    self.instruction_storage_name = Some(name);
                }
                Instruction::FullScan { filter } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err(Error::ExecutionError(
                            "No storage name provided".to_string(),
                        ));
                    };

                    let storage = database.get_storage(storage_name)?;

                    for (key, value) in storage.data.iter() {
                        match filter {
                            Filter::Condition(ref condition) => {
                                if condition(key.clone(), value.clone()) {
                                    self.output.push(value.clone());
                                }
                            }
                        }
                    }
                }
                Instruction::MapOutput { map_fn } => {
                    self.output = self
                        .output
                        .iter()
                        .map(|value| map_fn(value.clone()))
                        .collect();
                }
                _ => (),
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub enum Instruction {
    UseStorage {
        name: String,
    },
    ClearOutput,
    FullScan {
        filter: Filter,
    },
    IndexScan {
        filter: Filter,
    },
    MapOutput {
        map_fn: Box<dyn Fn(String) -> String>,
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
    Condition(Box<dyn Fn(String, String) -> bool>),
}
